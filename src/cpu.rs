use std::fs;
use crate::*;

pub struct CPUInfo {
    pub model: String,
    pub cores: usize,
    pub freq:  usize,
}

impl CPUInfo {
    pub fn new() -> CPUInfo {
        CPUInfo {
            model: String::new(),
            cores: 0,
            freq:  0,
        }
    }

    // retrieve model, cores, and frequency
    pub fn get(&mut self) -> Result<()> {
        // model and number of cores
        let cpuinfos = fs::read_to_string("/proc/cpuinfo").context(CPUErr)?;
        for line in cpuinfos.split("\n") {
            let cpuinfo = line.split(":").map(|i| i.trim()).collect::<Vec<&str>>();
            match cpuinfo[0] {
                "Hardware" => self.model = cpuinfo[1].to_string(),
                "processor" => self.cores = cpuinfo[1].parse::<usize>().unwrap() + 1,
                _ => (),
            }
        }

        // frequency
        let freq_file = "/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq";
        self.freq = (fs::read_to_string(freq_file).context(CPUErr)?
            .trim_end().parse::<usize>().unwrap()) / 1000000;

        Ok(())
    }

    // format it
    pub fn format(&self) -> String {
        format!("{} ({}) @ {}GHz", self.model, self.cores, self.freq)
    }
}
