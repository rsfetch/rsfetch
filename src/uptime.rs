use std::fs;
use std::result::Result;

pub struct UptimeInfo {
    pub days:    usize,
    pub hours:   usize,
    pub minutes: usize,
}

impl UptimeInfo {
    pub fn new() -> UptimeInfo {
        UptimeInfo {
            days:    0,
            hours:   0,
            minutes: 0,
        }
    }

    // retrieve model, cores, and frequency
    pub fn get(&mut self) -> Result<(), std::io::Error> {
        let mut proc_uptime: &str = &*std::fs::read_to_string("/proc/uptime")?;

        // right now, proc_uptime looks like this:
        // 98798798.98 12897928l.12
        // we need to trim off everything after the first dot
        proc_uptime = proc_uptime.split(".").collect::<Vec<&str>>()[0];

        // convert proc_uptime (a string) to usize
        let seconds: i32 = proc_uptime.parse::<i32>().unwrap();

        // convert seconds to days, hours, and minutes
        let days:    i32 = (seconds / 60 / 60 / 24);
        let hours:   i32 = (seconds / 60 / 60) % 24; // only 24 hours in a day!
        let minutes: i32 = (seconds / 60) % 60;      // only 60 minutes in an hour!

        self.days    = days;
        self.hours   = hours;
        self.minutes = minutes;

        Ok(())
    }

    pub fn format(&self) -> String {
        let uptime = "".to_owned();

        if self.days    > 0 { uptime = format!("{}d ", self.days);              }
        if self.hours   > 0 { uptime = format!("{}{}h ", uptime, self.hours);   }
        if self.minutes > 0 { uptime = format!("{}{}m ", uptime, self.minutes); }

        uptime
    }
}
