use std::fs;
use crate::*;
use std::vec::Vec;
use std::process::Command;

pub struct CPUInfo {
    pub model: String,
    pub cores: usize,
    pub freq:  usize,
}

// TODO: cleanup this function
pub fn is_bsd() -> bool {
    let uname = Command::new("uname").arg("-s")
        .output().unwrap();

    let os = uname.stdout.iter()
        .map(|b| *b as char).collect::<Vec<char>>();

    if os.len() < 3 {
        return false;
    }

    let mut last3: String = String::new();
    last3.push(os[os.len() - 3]);
    last3.push(os[os.len() - 2]);
    last3.push(os[os.len() - 1]);

    if last3.to_ascii_lowercase() != "bsd" {
        return false;
    } else {
        return true;
    }
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
        // check if it's BSD first...
        // TODO: test this
        if is_bsd() {
            let mut out = "".to_string();
            let _ = Command::new("sysctl").arg("-n hw.model")
                .output().context(BSDCPUErr)?
                .stdout.iter().map(|b|
            {
                out.push(*b as char);
            }).collect::<()>();

            self.model = out.split('@')
                .collect::<Vec<&str>>()[0].trim().to_string();

            let mut cores: String = String::new();
            let mut speed: String = String::new();

            let _ = Command::new("sysctl")
                .arg("-n hw.ncpu").output().context(BSDCPUErr)?
                .stdout.iter().map(|b| cores.push(*b as char))
                .collect::<()>();

            let _ = Command::new("sysctl")
                .arg("-n hw.cpuspeed").output().context(BSDCPUErr)?
                .stdout.iter().map(|b| cores.push(*b as char))
                .collect::<()>();
            if speed == "" {
                speed = "".to_string();
                let _ = Command::new("sysctl")
                    .arg("-n hw.clockrate").output().context(BSDCPUErr)?
                    .stdout.iter().map(|b| cores.push(*b as char))
                    .collect::<()>();
            }

            self.cores = cores.parse::<usize>().context(BSDCPUParseErr)?;
            self.freq  = speed.parse::<usize>().context(BSDCPUParseErr)?;
            return Ok(());
        }

        // model and number of cores
        let cpuinfos = fs::read_to_string("/proc/cpuinfo").context(CPUErr)?;
        for line in cpuinfos.split("\n") {
            let cpuinfo = line.split(":").map(|i| i.trim()).collect::<Vec<&str>>();
            match cpuinfo[0] {
                "Hardware"   => self.model = cpuinfo[1].to_string(),
                "processor"  => self.cores = cpuinfo[1].parse::<usize>().unwrap() + 1,
                "model name" => self.model = cpuinfo[1].to_string(),
                _ => (),
            }
        }

        // frequency
        let freq_file = "/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq";
        if fs::metadata(freq_file).is_ok() {
            self.freq = (fs::read_to_string(freq_file).context(CPUErr)?
                .trim_end().parse::<usize>().unwrap()) / 1000000;
        } else {
            self.freq = 0;
        }

        // remove junk from CPU model
        self.model = self.model.clone().replace("(TM)", "")
            .replace("(tm)", "")
            .replace("(R)", "")
            .replace("CPU", "")
            .replace("Processor", "")
            .replace("Intel ", "")
            .replace("AMD ", "")
            .replace("Qualcomm ", "")
            .trim().to_string();

        Ok(())
    }

    // format it, depending on whether we were able to get the frequency
    pub fn format(&self) -> String {
        if self.freq != 0 {
            format!("{} ({}) @ {}GHz", self.model, self.cores, self.freq)
        } else {
            format!("{} ({})", self.model, self.cores)
        }
    }
}
