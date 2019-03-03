use std::path::{Path, PathBuf};

use directories::{ProjectDirs, UserDirs};
use structopt::StructOpt;

use steno_lookup::{Dictionary, Error};

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

    /// The format in which results are printed
    ///
    /// `text` is the default.
    #[structopt(long, short, raw(possible_values = "&[\"text\", \"json\", \"alfred\"]"))]
    format: Option<String>,

    /// Word to look up
    search_term: String,
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
    // Build the list of dictionaries to load
    let dictionary_paths = dictionary_list(&opt)?;

    // Need to load each of the dicts
    let dictionaries = dictionary_paths
        .iter()
        .map(|path| Dictionary::load(path))
        .collect::<Result<Vec<_>, Error>>();

    dbg!(&dictionaries);

    Ok(())
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
