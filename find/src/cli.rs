use ::common::clap::{App, arg, flag, ArgMatches, parse_int};
use ::regex::Regex;

const NAME: &'static str = crate_name!();
const AUTHOR: &'static str = crate_authors!();
const VERSION: &'static str = crate_version!();
const ABOUT: &'static str = "
Kawaii Toa shall find all your files(porn) recursively.";

pub fn parser() -> App<'static, 'static> {
    App::new(NAME).about(ABOUT)
                  .author(AUTHOR)
                  .version(VERSION)
                  .arg(arg("PATTERN").required(true).help("Regex pattern to filter by"))
                  .arg(flag("sym").short("s")
                                  .help("Follow symbolic links. By default they are not followed."))
                  .arg(flag("minhop").takes_value(true)
                                     .help("Minimum number of hops before starting to look."))
                  .arg(flag("hop").takes_value(true)
                                  .help("Specifies depth of recursion."))
                  .arg(flag("quiet").short("q")
                                    .help("Ignore errors during search."))
                  .arg(flag("file").short("f")
                                   .help("Prints files."))
                  .arg(flag("dir").short("d")
                                  .help("Prints directories."))
                  .arg(arg("PATH").multiple(true).last(true).help("Folders on which to perform find"))
}

///CLI flags
#[derive(Default)]
pub struct Flags {
    ///Flag whether to print directories or not.
    pub dir: bool,
    ///Flag whether to print executables or not.
    pub file: bool,
    ///Flag whether to follow symbolic links.
    pub sym: bool,
    ///Flag whether to ignore errrors or not.
    pub quiet: bool
}

impl Flags {
    fn from_parsed(matches: &ArgMatches<'static>) -> Self {
        let mut dir = matches.is_present("dir");
        let mut file = matches.is_present("file");
        let sym = matches.is_present("sym");
        let quiet = matches.is_present("quiet");

        if dir == false && file == false {
            dir = true;
            file = true;
        }

        Flags {
            dir,
            file,
            sym,
            quiet
        }
    }
}

///CLI options
pub struct Options {
    ///Hop range (min, max)
    pub hop: (usize, usize)
}

impl Options {
    fn from_parsed(matches: &ArgMatches<'static>) -> Result<Self, String> {
        let mut result = Options::default();

        if let Some(minhop) = matches.value_of("minhop") {
            result.hop.0 = parse_int(minhop)?
        }
        if let Some(hop) = matches.value_of("hop") {
            result.hop.1 = parse_int(hop)?
        }

        return Ok(result);
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            hop: (0, ::std::usize::MAX)
        }
    }
}

///Parsed CLI arugments
pub struct Args {
    pub flags: Flags,
    pub opts: Options,
    pub pattern: Regex,
    pub paths: Vec<String>
}

impl Args {
    ///Parses commandline arguments and creates new instance
    pub fn new() -> Result<Args, String> {
        let matches = parser().get_matches();

        let pattern = matches.value_of("PATTERN").unwrap();
        let pattern = match Regex::new(pattern) {
            Ok(regex) => regex,
            Err(error) => return Err(format!("Couldn't compile pattern. {}", error))
        };

        let flags = Flags::from_parsed(&matches);
        let opts = Options::from_parsed(&matches)?;

        let paths = match matches.values_of("PATH") {
            Some(values) => values.map(|value| value.to_string()).collect(),
            None => vec![".".to_string()]
        };

        Ok(Args {
            flags,
            opts,
            pattern,
            paths
        })
    }

    #[cfg(test)]
    pub fn from_args(args: &[&str]) -> Result<Args, String> {
        let matches = parser().get_matches_from(args);

        let pattern = matches.value_of("PATTERN").unwrap();
        let pattern = match Regex::new(pattern) {
            Ok(regex) => regex,
            Err(error) => return Err(format!("Couldn't compile pattern. {}", error))
        };

        let flags = Flags::from_parsed(&matches);
        let opts = Options::from_parsed(&matches)?;

        let paths = match matches.values_of("PATH") {
            Some(values) => values.map(|value| value.to_string()).collect(),
            None => vec![".".to_string()]
        };

        Ok(Args {
            flags,
            opts,
            pattern,
            paths
        })
    }
}
