use std::fs;
use crate::*;

pub struct UptimeInfo {
    pub days:    i32,
    pub hours:   i32,
    pub minutes: i32,
}

impl UptimeInfo {
    pub fn new() -> UptimeInfo {
        UptimeInfo {
            days:    0,
            hours:   0,
            minutes: 0,
        }
    }

    pub fn get(&mut self) -> Result<()> {
        let mut proc_uptime: &str = &*fs::read_to_string("/proc/uptime")
            .context(Uptime)?;

        // right now, proc_uptime looks like this:
        // 98798798.98 12897928l.12
        // we need to trim off everything after the first dot
        proc_uptime = proc_uptime.split(".").collect::<Vec<&str>>()[0];

        // convert proc_uptime (a string) to usize
        let seconds: i32 = proc_uptime.parse::<i32>().unwrap();

        // convert seconds to days, hours, and minutes
        let days:    i32 = seconds / 60 / 60 / 24;
        let hours:   i32 = (seconds / 60 / 60) % 24; // only 24 hours in a day!
        let minutes: i32 = (seconds / 60) % 60;      // only 60 minutes in an hour!

        self.days    = days;
        self.hours   = hours;
        self.minutes = minutes;

        Ok(())
    }

    pub fn format(&self) -> String {
        let mut uptime = "".to_owned();

        if self.days    > 0 { uptime = format!("{}d ", self.days);              }
        if self.hours   > 0 { uptime = format!("{}{}h ", uptime, self.hours);   }
        if self.minutes > 0 { uptime = format!("{}{}m ", uptime, self.minutes); }

        uptime
    }
}
