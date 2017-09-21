use ::common::clap::{App, arg, flag, ArgMatches, parse_int};

use ::std::time;
use ::std::convert;
use ::std::net::{SocketAddr, ToSocketAddrs};

use ::socket2::{Domain};

use ::ping;

const NAME: &'static str = crate_name!();
const AUTHOR: &'static str = crate_authors!();
const VERSION: &'static str = crate_version!();
const ABOUT: &'static str = "
Kawaii ping tool.";

const AFTER_HELP: &'static str = "Destination format: <host>[:<port>]
";

pub fn parser() -> App<'static, 'static> {
    App::new(NAME).about(ABOUT)
                  .author(AUTHOR)
                  .version(VERSION)
                  .after_help(AFTER_HELP)
                  .arg(arg("destination").required(true).help("Destination to ping."))
                  .arg(flag("forever").short("f")
                                      .help("Keeps going forever."))
                  .arg(flag("4").help("Enforce IPv4 version.").overrides_with("6"))
                  .arg(flag("6").help("Enforce IPv6 version.").overrides_with("4"))
                  .arg(flag("protocol").short("p")
                                       .takes_value(true)
                                       .default_value("tcp")
                                       .help("Specifies protocol to use. Available: tcp"))
                  .arg(flag("number").short("n")
                                     .takes_value(true)
                                     .default_value("4")
                                     .help("Number of ping packets to send."))
                  .arg(flag("interval").short("i")
                                       .takes_value(true)
                                       .default_value("500")
                                       .help("Time interval between pings in milliseconds."))
                  .arg(flag("timeout").short("w")
                                       .takes_value(true)
                                       .default_value("1000")
                                       .help("Time to wait for each response in milliseconds."))
}

///CLI flags
#[derive(Default)]
pub struct Flags {
    ///Flag whether to keep ping forever.
    pub forever: bool
}

impl Flags {
    fn from_parsed(matches: &ArgMatches<'static>) -> Self {
        let forever = matches.is_present("forever");

        Flags {
            forever
        }
    }
}

///IP Version option
pub enum IPversion {
    V4,
    V6,
    None
}

impl IPversion {
    pub fn socket2(&self) -> Domain {
        match *self {
            IPversion::V4 => Domain::ipv4(),
            IPversion::V6 => Domain::ipv6(),
            IPversion::None => unreachable!()
        }
    }
}

pub enum Protocol {
    TCP
}

impl Protocol {
    fn from_str(name: &str) -> Result<Self, String> {
        match name.to_lowercase().as_str() {
            "tcp" => Ok(Protocol::TCP),
            _ => Err(format!("Unknown protocol '{}' provided", name))
        }
    }
}

impl convert::Into<ping::FnType> for Protocol {
    fn into(self) -> ping::FnType {
        match self {
            Protocol::TCP => ping::tcp
        }
    }
}

///CLI options
pub struct Options {
    pub number: usize,
    pub interval: time::Duration,
    pub timeout: time::Duration,
    pub domain: IPversion,
    pub protocol: Protocol
}

impl Options {
    fn from_parsed(matches: &ArgMatches<'static>) -> Result<Self, String> {
        let number = parse_int(matches.value_of("number").unwrap())?;
        let interval = parse_int::<u64>(matches.value_of("interval").unwrap())?;
        let timeout = parse_int(matches.value_of("timeout").unwrap())?;
        let protocol = Protocol::from_str(matches.value_of("protocol").unwrap())?;

        let domain = if matches.is_present("4") {
            IPversion::V4
        } else if matches.is_present("6") {
            IPversion::V6
        } else {
            IPversion::None
        };

        let interval = time::Duration::from_millis(interval);
        let timeout = time::Duration::from_millis(timeout);

        Ok(Options {
            number,
            interval,
            timeout,
            domain,
            protocol
        })
    }
}

///Parsed CLI arugments
pub struct Args {
    pub flags: Flags,
    pub opts: Options,
    pub destination: SocketAddr,
}

///Resolves which IP use for ping.
fn resolve_ip(dest: &str, opts: &mut Options) -> Result<SocketAddr, String> {
    let addrs = match dest.to_socket_addrs() {
        Ok(iter) => iter,
        Err(_) => {
            if let Ok(addrs) = (dest, 0).to_socket_addrs() {
                addrs
            }
            else {
                return Err(format!("Invalid destination: {}", dest));
            }
        }
    };

    let mut destination: Option<SocketAddr> = None;
    let mut destination4: Option<SocketAddr> = None;
    let mut destination6: Option<SocketAddr> = None;

    for dest in addrs {
        if destination.is_none() {
            destination = Some(dest);
        }

        match dest {
            SocketAddr::V4(_) => if destination4.is_none() { destination4 = Some(dest) },
            SocketAddr::V6(_) => if destination6.is_none() { destination6 = Some(dest) }
        }
    }

    if destination.is_none() {
        return Err("Cannot resolve address of your destination :(".to_string());
    }

    let mut destination = match opts.domain {
        IPversion::V4 => {
            match destination4 {
                Some(dest) => dest,
                None => return Err("There is no IPv4 address to ping, baka!".to_string())
            }
        },
        IPversion::V6 => {
            match destination6 {
                Some(dest) => dest,
                None => return Err("There is no IPv6 address to ping, baka!".to_string())
            }
        },
        _ => {
            let dest = destination.unwrap();
            opts.domain = match dest {
                SocketAddr::V4(_) => IPversion::V4,
                SocketAddr::V6(_) => IPversion::V6,
            };
            dest
        }
    };

    if destination.port() == 0 {
        destination.set_port(80);
    }

    Ok(destination)
}

impl Args {
    ///Parses commandline arguments and creates new instance
    pub fn new() -> Result<Args, String> {
        let matches = parser().get_matches();

        let flags = Flags::from_parsed(&matches);
        let mut opts = Options::from_parsed(&matches)?;
        let destination = resolve_ip(matches.value_of("destination").unwrap(), &mut opts)?;

        Ok(Args {
            flags,
            opts,
            destination,
        })
    }
}
