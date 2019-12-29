use std::fs;
use crate::*;

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

    pub fn get(&mut self) -> Result<()> {
        // for Linux OSs
        if fs::metadata("/proc/meminfo").is_ok() {
            let mut total: u64 = 0_u64;
            let mut used:  u64 = 0_u64;
            let _ = fs::read_to_string("/proc/meminfo")
                .context(RAMErr)?.split("\n").map(|i| {
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
                }).collect::<()>();
            self.used  = Some(used  / 1024);
            self.total = Some(total / 1024);
            return Ok(());        
        } else {
            return Ok(());
        }
    }

    pub fn format(&self) -> String {
        let mut info = String::new();

        if let Some(u) = self.used {
            info = format!("{}{}MiB", info, u);
        } else {
            info = format!("?MiB");
        }

        if let Some(t) = self.total {
            info = format!("{} / {}MiB", info, t);
        } else {
            info = format!("{} / ?MiB", info);
        }

        info
    }
}
