// sincere thanks to the contributors of `pfetch`,
// who may recognize some of their code here ;P

use std::fs;
use crate::*;
use crate::util::*;
use std::process::Command;

// all measures are in MiB
pub struct RAMInfo {
    total:     Option<u64>,
    used:      Option<u64>,
}

impl RAMInfo {
    pub fn new() -> RAMInfo {
        RAMInfo {
            total:     None,
            used:      None,
        }
    }

    pub fn get(&mut self, os: &OS) -> Result<()> {
        // temporary buffers
        let mut total = 0_u64;
        let mut used  = 0_u64;

        if os == &OS::Linux {
            // read contents of /proc/meminfo,
            // and simple split on `:` and parse into
            // proper format.
            fs::read_to_string("/proc/meminfo")
                .context(RAMErr)?.split("\n").for_each(|i| {
                    let inf = i.split(":").collect::<Vec<&str>>();
                    if inf.len() > 1 {
                        let key = inf[0].trim();
                        let val = inf[1].replace("kB", "")
                            .replace("\n", "").trim().parse::<u64>()
                            .unwrap();

                        match key {
                            "MemTotal"    => {
                                used += val;
                                total = val;
                            },
                            "Shmem"       => used += val,
                            "SReclaimable"|
                                "Buffers" |
                                "Cached"  |
                                "MemFree" => used -= val,
                            &_            => (),
                        }
                    }
            });
            self.used  = Some(used  / 1024);
            self.total = Some(total / 1024);
            return Ok(());
        } else if os == &OS::OpenBSD {
            let mut buffer = String::new();
            Command::new("sysctl").arg("-n").arg("hw.physmem")
                .output().context(RAMErr)?.stdout.iter()
                .for_each(|b| buffer.push(*b as char));
            total = buffer.parse::<u64>().unwrap();

            // flush buffer
            buffer = "".to_owned();

            Command::new("vmstat").output().context(RAMErr)?
                .stdout.iter().for_each(|b| buffer.push(*b as char));
            used = buffer.split("\n").last().unwrap().split(" ")
                .nth(2).unwrap()
                .parse::<u64>().unwrap();

            self.used  = Some(used  / 1024 / 1024);
            self.total = Some(total / 1024 / 1024);
            return Ok(());
        } else if os == &OS::FreeBSD || os == &OS::Other {
            let mut buffer = String::new();
            Command::new("sysctl").arg("-n").arg("hw.physmem")
                .output().context(RAMErr)?.stdout.iter()
                .for_each(|b| buffer.push(*b as char));
            total = buffer.parse::<u64>().unwrap();

            let pagesize: u64;
            let inactive: u64;
            let free:     u64;
            let cache:    u64;
            buffer = "".to_owned();

            Command::new("sysctl").arg("-n")
                .arg("hw.pagesize")
                .arg("vm.stats.vm.v_inactive_count")
                .arg("vm.stats.vm.v_free_count")
                .arg("vm.stats.vm.v_cache_count")
                .output().context(RAMErr)?.stdout
                .iter().for_each(|b| buffer.push(*b as char));

            let info = buffer.split("\n").collect::<Vec<&str>>();
            pagesize = info[0].parse::<u64>().unwrap();
            inactive = info[1].parse::<u64>().unwrap();
            free     = info[2].parse::<u64>().unwrap();
            cache    = info[3].parse::<u64>().unwrap();

            self.total = Some(total / 1024 / 1024);
            self.used  = Some(self.total.unwrap() -
                ((inactive + free + cache) * pagesize / 1024 / 1024));

            return Ok(());
        } else if os == &OS::NetBSD {
            let mut buffer = String::new();

            Command::new("sysctl").arg("-n").arg("hw.physmem64")
                .output().context(RAMErr)?.stdout
                .iter().for_each(|b| buffer.push(*b as char));
            total = buffer.parse::<u64>().unwrap();
            let mut free: u64 = 0_u64;

            fs::read_to_string("/proc/meminfo")
                .context(RAMErr)?.split("\n").for_each(|i| {
                    let inf = i.split(":").collect::<Vec<&str>>();
                    if inf.len() > 1 {
                        let key = inf[0].trim();
                        let val = inf[1].replace("kB", "")
                            .replace("\n", "").trim().parse::<u64>()
                            .unwrap();

                        match key {
                            "MemFree" => free = val,
                            &_        => (),
                        }
                    }
            });

            self.total = Some(total);
            self.used  = Some(total - free);

            return Ok(());
        } else {
            // leave memory information null,
            // it will be displayed later
            // as simply "?MiB / ?MiB"
            return Ok(());
        }
    }

    pub fn format(&self) -> String {
        let mut info = String::new();

        if let Some(u) = self.used {
            if u < 1024 {
                info = format!("{}{}MiB", info, u);
            } else {
                info = format!("{}{}GiB", info, u / 1024);
            }
        } else {
            info = format!("?");
        }

        if let Some(t) = self.total {
            if t < 1024 {
                info = format!("{} / {:.0}MiB", info, t);
            } else {
                info = format!("{} / {:.2}GiB", info, t / 1024);
            }
        } else {
            info = format!("{} / ?", info);
        }

        info
    }
}
