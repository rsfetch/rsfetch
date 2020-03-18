use crate::*;
use std::fs;
use std::process::Command;
use std::vec::Vec;

pub struct UptimeInfo {
    pub days: u64,
    pub hours: u64,
    pub minutes: u64,
}

impl UptimeInfo {
    pub fn new() -> UptimeInfo {
        UptimeInfo {
            days: 0,
            hours: 0,
            minutes: 0,
        }
    }

    pub fn get(&mut self) -> Result<()> {
        let seconds: u64;
        if fs::metadata("/proc/uptime").is_ok() {
            let mut proc_uptime: &str = &*fs::read_to_string("/proc/uptime").context(Uptime)?;

            // right now, proc_uptime looks like this:
            // 98798798.98 12897928l.12
            // we need to trim off everything after the first dot
            proc_uptime = proc_uptime.split('.').collect::<Vec<&str>>()[0];

            // convert proc_uptime (a string) to usize
            seconds = proc_uptime.parse::<u64>().unwrap();
        } else {
            let mut sysctl: String = String::new();
            Command::new("sysctl")
                .arg("-n")
                .arg("kern.boottime")
                .output()
                .context(Uptime)?
                .stdout
                .iter()
                .map(|b| sysctl.push(*b as char))
                .collect::<()>();
            let boottime: u64 = sysctl.split(',').collect::<Vec<&str>>()[0]
                .split("sec =")
                .collect::<Vec<&str>>()[1]
                .trim()
                .parse::<u64>()
                .unwrap();
            let current: u64 = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Your system time is not configured correctly.")
                .as_secs();
            seconds = current - boottime;
        }

        // convert seconds to days, hours, and minutes
        let days: u64 = seconds / 60 / 60 / 24;
        let hours: u64 = (seconds / 60 / 60) % 24; // only 24 hours in a day!
        let minutes: u64 = (seconds / 60) % 60; // only 60 minutes in an hour!

        self.days = days;
        self.hours = hours;
        self.minutes = minutes;

        Ok(())
    }

    pub fn format(&self) -> String {
        let mut uptime = "".to_owned();

        if self.days > 0 {
            uptime = format!("{}d ", self.days);
        }
        if self.hours > 0 {
            uptime = format!("{}{}h ", uptime, self.hours);
        }
        if self.minutes > 0 {
            uptime = format!("{}{}m ", uptime, self.minutes);
        }

        uptime
    }
}
