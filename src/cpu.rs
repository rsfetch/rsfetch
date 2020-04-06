use crate::util::*;
use crate::*;
use std::fs;
use std::process::Command;
use std::vec::Vec;

pub struct CPUOptions {
    pub farenheit: bool,
}

pub struct CPUInfo {
    pub model: String,
    pub cores: usize,
    pub freq: f64,
    pub temp: String,
    pub options: CPUOptions
}

impl CPUInfo {
    pub fn new(options: CPUOptions) -> CPUInfo {
        CPUInfo {
            model: String::new(),
            cores: 0,
            freq: 0_f64,
            temp: String::new(),
            options,
        }
    }

    // retrieve model, cores, and frequency
    pub fn get(&mut self, os: &OS) -> Result<()> {
        let freq_file = "/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq";
        let cpu_file = "/proc/cpuinfo";
        let temp_file = "/sys/class/thermal/thermal_zone0/temp";

        // check if it's BSD first...
        if os != &OS::Linux {
            let cpu_model = Command::new("sysctl")
                .arg("-n")
                .arg("hw.model")
                .output()
                .context(BSDCPUErr)?;

            let out = std::str::from_utf8(&cpu_model.stdout)
                .unwrap()
                .replace("\n", "");

            self.model = out.split('@').collect::<Vec<&str>>()[0].trim().to_string();

            // get core count
            let cpu_cores = Command::new("sysctl")
                .arg("-n")
                .arg("hw.ncpu")
                .output()
                .context(BSDCPUErr)?;

            let cores = String::from_utf8(cpu_cores.stdout)
                .unwrap()
                .replace("\n", "")
                .trim()
                .to_string();

            // get cpu clocking
            let cpu_speed = Command::new("sysctl")
                .arg("-n")
                .arg("hw.cpuspeed")
                .output()
                .context(BSDCPUErr)?;

            let mut speed = String::from_utf8(cpu_speed.stdout)
                .unwrap()
                .replace("\n", "")
                .trim()
                .to_string();

            if speed == "" {
                let cpu_clockrate = Command::new("sysctl")
                    .arg("-n")
                    .arg("hw.clockrate")
                    .output()
                    .context(BSDCPUErr)?;

                speed = String::from_utf8(cpu_clockrate.stdout)
                    .unwrap()
                    .replace("\n", "")
                    .trim()
                    .to_string();
            }

            self.cores = cores.parse::<usize>().context(BSDCPUParseErr)?;
            self.freq = speed.parse::<f64>().context(CPUFreqParseErr)? / 1000_f64;
            return Ok(());
        }

        // model and number of cores
        let cpuinfos = fs::read_to_string(cpu_file).context(CPUErr)?;
        for line in cpuinfos.split('\n') {
            let cpuinfo = line.split(':').map(|i| i.trim()).collect::<Vec<&str>>();
            match cpuinfo[0] {
                "Hardware" => self.model = cpuinfo[1].to_string(),
                "processor" => self.cores = cpuinfo[1].parse::<usize>().unwrap() + 1,
                "model name" => self.model = cpuinfo[1].to_string(),
                _ => (),
            }
        }

        // frequency
        if fs::metadata(freq_file).is_ok() {
            self.freq = fs::read_to_string(freq_file)
                .context(CPUErr)?
                .trim_end()
                .parse::<f64>()
                .context(CPUFreqParseErr)?
                / 1_000_000_f64;
        } else {
            self.freq = 0_f64;
        }

        if fs::metadata(temp_file).is_ok() {
            let mut temp = fs::read_to_string(temp_file)
                .context(CPUErr)?
                .trim_end()
                .parse::<f64>()
                .unwrap()/1000.0;
            let temp_scale = if self.options.farenheit {
                temp = (temp * (9.0 / 5.0)) + 32.0;
                "F"
            } else {
                "C"
            };
            self.temp = format!("{:.1}°{}", temp, temp_scale);
        }

        // remove junk from CPU model
        self.model = self.model.clone().split('@').collect::<Vec<&str>>()[0]
            .replace("(TM)", "")
            .replace("(tm)", "")
            .replace("(R)", "")
            .replace("CPU", "")
            .replace("Processor", "")
            .replace("Core ", "")
            .trim()
            .to_string();

        Ok(())
    }

    // format it, depending on whether we were able to get the frequency
    pub fn format(&self) -> String {
        if self.freq != 0_f64 {
            format!("{} ({}) @ {:.3}GHz ({})", self.model, self.cores, self.freq, self.temp)
        } else {
            format!("{} ({}) ({})", self.model, self.cores, self.temp)
        }
    }
}
