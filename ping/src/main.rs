#[macro_use]
extern crate common;
#[macro_use]
extern crate lazy_static;

extern crate socket2;

use std::process::exit;
use std::sync::RwLock;
use std::thread;

mod ping;
mod cli;
mod crtl_c;
mod stats;

lazy_static! {
    pub static ref STATS: RwLock<stats::Stats> = RwLock::new(stats::Stats::new());
}

fn run() -> Result<i32, String> {
    let args = match cli::Args::new() {
        Ok(args) => args,
        Err(error) => return Err(error)
    };

    let ping_fn: ping::FnType = args.opts.protocol.into();
    crtl_c::set_handler();
    println!("Pinging {}/{}", args.destination.ip(), args.destination.port());

    let mut idx = 0;
    loop {
        if !args.flags.forever && idx == args.opts.number {
            break;
        }

        let (ok, elapsed) = match (ping_fn)(args.opts.domain.socket2(), &args.destination, args.opts.timeout) {
            Ok(result) => result,
            Err(error) => return Err(error)
        };

        let reply = match ok {
            false => format!("No reply"),
            true => format!("Reply from {}", args.destination.ip())
        };

        let elapsed_ms = (elapsed.as_secs() * 1000) as f64 + elapsed.subsec_nanos() as f64 / 1000000.0;
        println!("    {}: {} - rto={:.3}ms",
                 idx,
                 reply,
                 elapsed_ms);

        STATS.write().unwrap().add_ping(ok, elapsed_ms);
        idx += 1;
        thread::sleep(args.opts.interval);
    }

    let stats = STATS.read().unwrap();
    println!("{}", *stats);
    Ok(!stats.is_ok() as i32)
}

fn main() {
    let code: i32 = match run() {
        Ok(res) => res,
        Err(error) => {
            eprintln!("{}", error);
            1
        }
    };

    exit(code);
}

