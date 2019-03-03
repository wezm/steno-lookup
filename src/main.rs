use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;

use directories::{ProjectDirs, UserDirs};
use rayon::prelude::*;
use structopt::StructOpt;
use url::Url;

use steno_lookup::{Dictionary, Error, InvertedDictionary, Stroke};

const ADDR: &str = "127.0.0.1:25040";
const SERVER_THREADS: usize = 2;

#[derive(Debug)]
enum OutputFormat {
    Text,
    Json,
    Alfred,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "steno-lookup", about = "Look up strokes for word.")]
struct Opt {
    /// Specify path to plover.cfg
    ///
    /// If not supplied will look in standard location unless `--noconfig` is supplied.
    #[structopt(short, long, parse(from_os_str))]
    config: Option<PathBuf>,

    /// Disable loading the plover.cfg
    #[structopt(long)]
    noconfig: bool,

    /// Config section
    ///
    /// Name of section in Plover config to get dictionary list from.
    #[structopt(short, long, default_value = "System: English Stenotype")]
    section: String,

    /// Path to dictionary file to load
    ///
    /// Can be specified multiple times. The dictionaries are loaded in the order they appear on
    /// the command line.
    #[structopt(short = "d", long = "dict", parse(from_os_str), number_of_values = 1)]
    dictionaries: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    match run(&opt) {
        Ok(()) => (),
        Err(Error::Io(err)) => eprintln!("IO error {}", err),
        Err(Error::Json(err)) => eprintln!("JSON error {}", err),
        Err(Error::Ini(err)) => eprintln!("INI error {}", err),
        Err(Error::FileNotFound(path)) => eprintln!("File not found: {}", path.to_string_lossy()),
        Err(Error::SectionMissing) => {
            eprintln!("section '{}' not found in plover config", opt.section)
        }
        Err(Error::HomeNotFound) => eprintln!("Unable to dermine home directory"),
    }
}

fn run(opt: &Opt) -> Result<(), Error> {
    // Need to load each of the dicts, then transform them into a HashMap that maps word to strokes
    let dictionaries = Arc::new(
        dictionary_list(&opt)?
            .par_iter()
            .inspect(|path| eprintln!("Loading {}", path.to_string_lossy()))
            .map(|path| Dictionary::load(path).map(Dictionary::invert))
            .collect::<Result<Vec<_>, Error>>()?,
    );

    let server = Arc::new(tiny_http::Server::http(ADDR).unwrap());
    println!("Now listening on port 25040");

    let mut handles = Vec::new();

    for _ in 0..SERVER_THREADS {
        let server = server.clone();
        let dictionaries = dictionaries.clone();

        handles.push(thread::spawn(move || {
            let base_url = Url::parse(&format!("https://{}", ADDR)).unwrap();
            for req in server.incoming_requests() {
                let url = base_url.join(req.url()).unwrap(); // Can this fail?
                println!("{} {}", req.method(), url);

                let response = {
                    use tiny_http::Method::*;

                    match (req.method(), url.path()) {
                        (Get, "/") => {
                            tiny_http::Response::from_string("steno-lookup".to_string()).boxed()
                        }
                        (Get, "/lookup") => handle_lookup(&dictionaries, &url),
                        _ => handle_not_found(),
                    }
                };

                let _ = req.respond(response);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    Ok(())
}

fn handle_not_found() -> tiny_http::ResponseBox {
    tiny_http::Response::from_string("Not Found".to_string()).boxed()
}

fn handle_lookup(dictionaries: &[InvertedDictionary], url: &Url) -> tiny_http::ResponseBox {
    // Get the search term
    if let Some((_k, search_term)) = url.query_pairs().find(|(k, _v)| k == "q") {
        println!("Lookup: '{}'", search_term);
        let output = format!("{:?}", lookup(dictionaries, &search_term));
        tiny_http::Response::from_string(output).boxed()
    } else {
        tiny_http::Response::from_string("Bad Request".to_string()).boxed()
    }
}

// TODO: Move this out of main
fn lookup<'a>(dictionaries: &'a [InvertedDictionary], search_term: &str) -> Vec<&'a Stroke> {
    dictionaries.iter().fold(vec![], |mut results, dict| {
        // FIXME: Deal with argument to `get` (should be &str)
        if let Some(strokes) = dict.get(search_term.to_owned()) {
            results.extend(strokes.iter());
        }

        results
    })
}

fn dictionary_list(opt: &Opt) -> Result<Vec<PathBuf>, Error> {
    let mut dictionaries = Vec::new();

    // Add dictionaries from plover config unless --noconfig was passed
    if !opt.noconfig {
        let plover_config_path = opt.plover_config_path()?;

        // TODO: Handle file not found here and turn it into Error::ConfigNotFound
        let plover_dicts =
            steno_lookup::plover_config::dictionaries(plover_config_path, &opt.section)?;

        let user_dirs = UserDirs::new().ok_or_else(|| Error::HomeNotFound)?;
        dictionaries.extend(plover_dicts.into_iter().filter_map(|dict| {
            if dict.enabled {
                Some(expand_tilde(&user_dirs, dict.path))
            } else {
                None
            }
        }));
    }

    // Add any that were passed on the command line
    dictionaries.append(&mut opt.dictionaries.clone());

    Ok(dictionaries)
}

impl From<&str> for OutputFormat {
    fn from(s: &str) -> Self {
        match s {
            "text" => OutputFormat::Text,
            "json" => OutputFormat::Json,
            "alfred" => OutputFormat::Alfred,
            // It is expected that StructOpt will have already sanitised the option so this should
            // not happen unless there is a bug in the code.
            _ => panic!("unexpected output format {}", s),
        }
    }
}

impl Opt {
    fn plover_config_path(&self) -> Result<PathBuf, Error> {
        self.config
            .clone()
            .or_else(|| {
                // https://git.io/fhAAL
                ProjectDirs::from("org", "plover", "plover")
                    .map(|proj_dirs| proj_dirs.data_local_dir().to_path_buf())
            })
            .ok_or_else(|| Error::HomeNotFound)
    }
}

// https://stackoverflow.com/a/54306906/38820
fn expand_tilde<P: Into<PathBuf>>(user_dirs: &UserDirs, path: P) -> PathBuf {
    let path = path.into();

    if path.starts_with("~") {
        let mut home = user_dirs.home_dir().to_path_buf();

        if path == Path::new("~") {
            home
        } else {
            if home == Path::new("/") {
                // Corner case: `home` root directory;
                // don't prepend extra `/`, just drop the tilde.
                path.strip_prefix("~").unwrap().to_path_buf()
            } else {
                home.push(path.strip_prefix("~/").unwrap());
                home
            }
        }
    } else {
        path
    }
}
