use std::fs;
use std::process;
use std::vec::Vec;

extern "C" {
    fn isatty(fd: i32);
}


pub struct Terminal {
    name: String,
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            name: String::new(),
        }
    }

    pub fn get(&mut self) -> Result<()> {
        let myid = process::id();

        fn get_ppid(id: u32) -> Option<u32> {
            if !fs::metadata(&format!("/proc/{}/status", id)).is_ok() {
                return None;
            }

            let ppid_str = fs::read_to_string(&format!("/proc/{}/status", id))
                .unwrap()
                .split("\n")
                .for_each(|i| {
                    let info = i.split(":").collect::<Vec<&str>>();
                    let key = info[0].trim();
                    let val = info[1].trim()
                        .replace("\n", "");

                    if key == "PPid" {
                        val
                    }
                });

            let ppid = ppid_str.parse::<u32>();
            match ppid {
                Ok(i)  => {
                    if i == 0 {
                        return None;
                    } else {
                        return Some(i);
                    }
                },
                Err(_) => return None,
            }
        }

        let lastid = myid;
        while let Some(newid) = get_ppid(lastid) {
            lastid = newid;

            // TODO: retrieve the name and id of
            // the process at one go, instead of reading
            // the process-info file TWICE
            let ppname = fs::read_to_string(&format!("/proc/{}/status", id))
                .unwrap()
                .split("\n")
                .for_each(|i| {
                    let info = i.split(":").collect::<Vec<&str>>();
                    let key = info[0].trim();
                    let val = info[1].trim()
                        .replace("\n", "");

                    if key == "Name" {
                        val
                    }
                });

            let chars = ppname.chars().collect::<Vec<char>>();

            // skip shells (e.g. mksh, bash, zsh, elvish, etc)
            // and GNU screen
            if ppname.end_with("sh") ||
                ppname == "ion" || ppname == "screen" {
                continue;
            }

            // if ppname is eq to `(l|L)ogin` or `init`, term
            // should be eq to output from tty command.
            if ppname.starts_with("login") ||
                ppname.starts_with("Login") ||
                ppname.starts_with("init") {
                    // TODO: implement
            }

            if ppname == "gnome-terminal-" {
                self.name = "gnome-terminal".to_string();
                break;
            } else if ppname == "urxvtd" {
                self.name = "urxvt".to_string();
                break;
            } else {
                self.name = ppname.split("/").last().to_string();
                break;
            }
        }

        Ok(())
    }

    // format it
    pub fn format(&self) -> String {
        self.name.clone()
    }
}
