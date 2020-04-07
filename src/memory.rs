// sincere thanks to the contributors of `pfetch`,
// who may recognize some of their code here ;P

use crate::util::*;
use crate::*;
use std::fs;
use std::process::Command;

// all measures are in MiB
pub struct RAMInfo {
    total: Option<f64>,
    used: Option<f64>,
}

impl RAMInfo {
    pub fn new() -> RAMInfo {
        RAMInfo {
            total: None,
            used: None,
        }
    }

	#[cfg(target_os = "linux")]
    pub fn get(&mut self) -> Result<()> {
        // temporary buffers
        let mut total = 0_f64;
        let mut used = 0_f64;

        fs::read_to_string("/proc/meminfo")
            .context(RAMErr)?
            .split('\n')
            .for_each(|i| {
                let inf = i.split(':').collect::<Vec<&str>>();
                if inf.len() > 1 {
                    let key = inf[0].trim();
                    let val = inf[1]
                        .replace("kB", "")
                        .replace("\n", "")
                        .trim()
                        .parse::<f64>()
                        .unwrap();
			
                    match key {
                        "MemTotal" => {
                            used += val;
                            total = val;
                        }
                        "Shmem" => used += val,
                        "SReclaimable" | "Buffers" | "Cached" | "MemFree" => used -= val,
                        &_ => (),
                    }
                }
            });
        self.used = Some(used / 1024_f64);
        self.total = Some(total / 1024_f64);
        Ok(())
    }

	#[cfg(any (freebsd, openbsd, netbsd, dragonfly))]
    pub fn get(&mut self) -> Result<()> {
        // temporary buffers
        let mut total = 0_f64;
        let mut used = 0_f64;
        let os = match OSInfo::get_os() {
            Ok(o) => o,
            Err(_) => {
                error!("unable to detect OS - results may be inaccurate.");
            }
        };
		if os == &OS::OpenBSD {
            let mut buffer = String::new();
            Command::new("sysctl")
                .arg("-n")
                .arg("hw.physmem")
                .output()
                .context(RAMErr)?
                .stdout
                .iter()
                .for_each(|b| buffer.push(*b as char));
            total = buffer.parse::<f64>().unwrap();

            // flush buffer
            buffer = "".to_owned();

            Command::new("vmstat")
                .output()
                .context(RAMErr)?
                .stdout
                .iter()
                .for_each(|b| buffer.push(*b as char));
            used = buffer
                .split('\n')
                .last()
                .unwrap()
                .split(' ')
                .nth(2)
                .unwrap()
                .parse::<f64>()
                .unwrap();

            self.used = Some(used / 1024_f64 / 1024_f64);
            self.total = Some(total / 1024_f64 / 1024_f64);
            Ok(())
        } else if os == &OS::FreeBSD || os == &OS::DragonflyBSD {
            let mut buffer = String::new();
            Command::new("sysctl")
                .arg("-n")
                .arg("hw.physmem")
                .output()
                .context(RAMErr)?
                .stdout
                .iter()
                .for_each(|b| buffer.push(*b as char));

            // remove non-integer chars from buffer
            buffer = buffer.trim().replace("\n", "");
            total = buffer.parse::<u64>().unwrap() as f64;

            let pagesize: f64;
            let inactive: f64;
            let free: f64;
            let cache: f64;
            buffer = "".to_owned();

            Command::new("sysctl")
                .arg("-n")
                .arg("hw.pagesize")
                .arg("vm.stats.vm.v_inactive_count")
                .arg("vm.stats.vm.v_free_count")
                .arg("vm.stats.vm.v_cache_count")
                .output()
                .context(RAMErr)?
                .stdout
                .iter()
                .for_each(|b| buffer.push(*b as char));

            let info = buffer.split('\n').collect::<Vec<&str>>();
            pagesize = info[0].parse::<f64>().unwrap();
            inactive = info[1].parse::<f64>().unwrap();
            free = info[2].parse::<f64>().unwrap();
            cache = info[3].parse::<f64>().unwrap();

            self.total = Some(total / 1024_f64 / 1024_f64);
            self.used = Some(
                self.total.unwrap() - ((inactive + free + cache) * pagesize / 1024_f64 / 1024_f64),
            );

            Ok(())
        } else if os == &OS::NetBSD {
            let mut buffer = String::new();

            Command::new("sysctl")
                .arg("-n")
                .arg("hw.physmem64")
                .output()
                .context(RAMErr)?
                .stdout
                .iter()
                .for_each(|b| buffer.push(*b as char));
            total = buffer.parse::<f64>().unwrap();
            let mut free: f64 = 0_f64;

            fs::read_to_string("/proc/meminfo")
                .context(RAMErr)?
                .split('\n')
                .for_each(|i| {
                    let inf = i.split(':').collect::<Vec<&str>>();
                    if inf.len() > 1 {
                        let key = inf[0].trim();
                        let val = inf[1]
                            .replace("kB", "")
                            .replace("\n", "")
                            .trim()
                            .parse::<f64>()
                            .unwrap();

                        match key {
                            "MemFree" => free = val,
                            &_ => (),
                        }
                    }
                });

            self.total = Some(total);
            self.used = Some(total - free);

            Ok(())
        } else {
            // leave memory information null,
            // it will be displayed later
            // as simply "?MiB / ?MiB"
            Ok(())
        }
    }

    pub fn format(&self) -> String {
        let mut info = String::new();

        if let Some(u) = self.used {
            if u < 1024_f64 {
                info = format!("{}{:.0}MiB", info, u);
            } else {
                info = format!("{}{:.2}GiB", info, u / 1024_f64);
            }
        } else {
            info = "?".to_string();
        }

        if let Some(t) = self.total {
            if t < 1024_f64 {
                info = format!("{} / {:.0}MiB", info, t);
            } else {
                info = format!("{} / {:.2}GiB", info, t / 1024_f64);
            }
        } else {
            info = format!("{} / ?", info);
        }

        info
    }
}
