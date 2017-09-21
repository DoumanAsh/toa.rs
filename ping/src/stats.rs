use ::std::f64;
use ::std::fmt;

///Ping statics structure.
///
///Just print it to get info.
pub struct Stats {
    num: usize,
    num_success: usize,
    min_time: f64,
    max_time: f64,
    sum_time: f64
}

impl Stats {
    ///Initialize Stats.
    pub fn new() -> Stats {
        Stats {
            num: 0,
            num_success: 0,
            min_time: f64::MAX,
            max_time: 0.0,
            sum_time: 0.0
        }
    }

    ///Adds ping to Stats.
    pub fn add_ping(&mut self, is_ok: bool, time: f64) {
        self.num += 1;
        if is_ok {
            self.num_success += 1;

            if time < self.min_time {
                self.min_time = time;
            }
            else if time > self.max_time {
                self.max_time = time;
            }

            self.sum_time += time;
        }
    }

    ///Returns whether all pings are successful.
    pub fn is_ok(&self) -> bool {
        self.num == self.num_success
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.num_success > 0 {
            let success_rate = (self.num_success * 100) as f64 / self.num as f64;
            write!(f, "
Summary:
    {} pings sent.
    {} succesful. Success Rate: {:.2}%
RTT statics:
    min={:.3}ms, average={:.3}ms, max={:.3}ms
               ",
               self.num, self.num_success, success_rate,
               self.min_time, self.sum_time / self.num_success as f64, self.max_time)
        }
        else {
            write!(f, "
Summary:
    {} pings sent.
    {} succesful. Rate: {:.2}%
RTT statics:
    Was unable to collect any statics :(
               ",
               self.num, self.num_success, 0)
        }
    }
}
