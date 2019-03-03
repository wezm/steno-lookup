use std::path::{Path, PathBuf};

use directories::{ProjectDirs, UserDirs};
use structopt::StructOpt;

use steno_lookup::Error;

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

    /// Word to look up
    search_term: String,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    // Build the list of dictionaries to load
    dbg!(&dictionary_list(&opt).expect("dict error"));
}

fn dictionary_list(opt: &Opt) -> Result<Vec<PathBuf>, Error> {
    let mut dictionaries = Vec::new();

    // Add dictionaries from plover config unless --noconfig was passed
    if !opt.noconfig {
        let plover_config_path = opt
            .plover_config_path()
            .ok_or_else(|| Error::ConfigNotFound)?;
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

impl Opt {
    fn plover_config_path(&self) -> Option<PathBuf> {
        self.config.clone().or_else(|| {
            // https://git.io/fhAAL
            ProjectDirs::from("org", "plover", "plover")
                .map(|proj_dirs| proj_dirs.data_local_dir().to_path_buf())
        })
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
