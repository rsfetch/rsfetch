use std::fs;
use crate::*;
use crate::util::*;

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

    pub fn get(&mut self, os: OS) -> Result<()> {
        // for Linux OSs
        if os == OS::Linux {
            let mut total: u64 = 0_u64;
            let mut used:  u64 = 0_u64;
            let _ = fs::read_to_string("/proc/meminfo")
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
        } else if os == OS::OpenBSD {
            let mut total: u64 = 0_u64;
            let mut used:  u64 = 0_u64;

            let mut buffer = String::new();
            Command::new("sysctl").arg("-n").arg("hw.physmem")
                .output().context(RAMErr)?.stdout.iter()
                .for_each(|b| buffer.push(*b as char));
            total = buffer.parse::<u64>();

            // flush buffer
            buffer = "".to_owned();

            Command::new("vmstat").output().context(RAMErr)?
                .stdout.iter().for_each(|b| buffer.push(*b as char));
            used = buffer.split("\n").last().split(" ").nth(2).parse();

            self.used  = Some(used  / 1024 / 1024);
            self.total = Some(total / 1024 / 1024);
            return Ok(());
        } else if os == OS::FreeBSD ||
            os == OS::Other {
            return Ok(());
        } else if os == OS::NetBSD {
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
                info = format!("{} / {}MiB", info, t);
            } else {
                info = format!("{} / {}GiB", info, t / 1024);
            }
        } else {
            info = format!("{} / ?", info);
        }

        info
    }
}
