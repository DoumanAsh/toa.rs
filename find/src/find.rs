use ::walkdir;
use walkdir::{WalkDir};
use ::path;

use ::cli;

///Find App runner
pub struct Find {
    args: cli::Args
}

impl Find {
    pub fn from_parser(parser: cli::Args) -> Self {
        Find {
            args: parser
        }
    }

    #[inline]
    ///Filter by type of entry.
    fn filter_type(&self, entry: &walkdir::DirEntry) -> bool {
        let entry_type = entry.file_type();

        (entry_type.is_file() && self.args.flags.file) || (entry_type.is_dir() && self.args.flags.dir) || false
    }

    #[inline]
    ///Filter by name of entry.
    fn filter_name(&self, entry: &walkdir::DirEntry) -> bool {
        let name = entry.file_name().to_str().unwrap();
        self.args.pattern.is_match(name)
    }

    #[inline]
    ///Filters errors out and prints them, if needed.
    fn filter_error(&self, value: walkdir::Result<walkdir::DirEntry>) -> Option<walkdir::DirEntry> {
        match value {
            Ok(entry) => Some(entry),
            Err(error) => {
                if !self.args.flags.quiet {
                    eprintln!("ERROR: {}", error);
                }
                None
            }
        }
    }

    pub fn run(&self) -> i32 {
        let mut result = 1;
        let paths = self.args.paths.iter();

        for path in paths {
            let path = path::Path::new(&path);

            if !path.exists() {
                if !self.args.flags.quiet {
                    eprintln!("toa: {} cannot access", path.display());
                }
                continue;
            }

            let walker = WalkDir::new(&path).min_depth(self.args.opts.hop.0)
                                            .max_depth(self.args.opts.hop.1)
                                            .follow_links(self.args.flags.sym)
                                            .into_iter()
                                            .filter_map(|elem| self.filter_error(elem))
                                            .filter(|elem| self.filter_type(elem))
                                            .filter(|elem| self.filter_name(elem));
            for entry in walker {
                result = 0;
                println!("{}", entry.path().display());
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::{
        cli,
        Find
    };

    #[test]
    fn find_some_rs() {
        let args = cli::Args::from_args(&["toa-find", "-f", ".rs$"]);
        assert!(args.is_ok());
        let args = args.unwrap();

        let result = Find::from_parser(args).run();
        assert_eq!(result, 0);
    }

    #[test]
    fn find_some_rs_dirs() {
        let args = cli::Args::from_args(&["toa-find", "-d", ".rs$"]);
        assert!(args.is_ok());
        let args = args.unwrap();

        let result = Find::from_parser(args).run();
        assert_eq!(result, 1);
    }

    #[test]
    fn find_some_src_dir() {
        let args = cli::Args::from_args(&["toa-find", "-d", "src"]);
        assert!(args.is_ok());
        let args = args.unwrap();

        let result = Find::from_parser(args).run();
        assert_eq!(result, 0);
    }

    #[test]
    fn find_test_file_w_file() {
        let args = cli::Args::from_args(&["toa-find", "-f", "Cargo.toml"]);
        assert!(args.is_ok());
        let args = args.unwrap();

        let result = Find::from_parser(args).run();
        assert_eq!(result, 0);
    }

    #[test]
    fn find_test_file_w_dir() {
        let args = cli::Args::from_args(&["toa-find", "-d", "Cargo.toml"]);
        assert!(args.is_ok());
        let args = args.unwrap();

        let result = Find::from_parser(args).run();
        assert_eq!(result, 1);
    }
}
