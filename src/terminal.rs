use crate::*;
use std::fs;
use std::process;
use std::vec::Vec;
use libc::{ c_int, isatty, ttyname };
use std::ffi::CStr;

fn get_ppid(id: u32) -> Option<u32> {
    if !fs::metadata(&format!("/proc/{}/status", id)).is_ok() {
        return None;
    }

    let mut ppid_str = String::new();
    fs::read_to_string(&format!("/proc/{}/status", id))
        .unwrap()
        .split("\n")
        .for_each(|i| {
            let info = i.split(":").collect::<Vec<&str>>();
            if info.len() > 1 {
                let key = info[0].trim();
                let val = info[1].trim()
                    .replace("\n", "");

                if key == "PPid" {
                    ppid_str = val;
                }
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

        let mut lastid = myid;

        // TODO: cleanup
        while let Some(newid) = get_ppid(lastid) {
            lastid = newid;

            // TODO: retrieve the name and id of
            // the process at one go, instead of reading
            // the process-info file TWICE
            let mut ppname = String::new();
            fs::read_to_string(&format!("/proc/{}/status", lastid))
                .unwrap()
                .split("\n")
                .for_each(|i| {
                    let info = i.split(":").collect::<Vec<&str>>();
                    if info.len() > 1 {
                        let key = info[0].trim();
                        let val = info[1].trim().to_string();

                        if key == "Name" {
                            ppname = val;
                        }
                    }
            });

            // remove spaces/newlines
            ppname.trim().replace("\n", "").to_string();
            print!("found proc '{}' of id {}\n", ppname, lastid);

            // skip mosh, ssh, and shells (e.g. bash, zsh, etc)
            // and GNU screen/tmux
            if ppname.ends_with("sh") ||
                ppname == "ion" || ppname == "screen" ||
                ppname.starts_with("tmux") || ppname == "tmux" {
                print!("proc is shell, skipping\n");
                continue;
            }

            // if ppname is eq to `(l|L)ogin` or `init`, term
            // should be eq to output from tty command.
            if ppname.starts_with("login") ||
                ppname.starts_with("Login") ||
                ppname.starts_with("init") {
                print!("proc is init, retrieving tty name\n");
                    let mut istty = true;
                    unsafe {
                        if isatty(0 as c_int) == 0 {
                            print!("stdin isn't tty :(\n");
                            istty = false;
                        }
                    }

                    if istty {
                        unsafe {
                            self.name = CStr::from_ptr(ttyname(0 as c_int))
                                .to_str().unwrap().to_owned();
                            print!("got tty: {}\n", self.name.clone());
                        }
                    } else {
                        self.name = "tty".to_string();
                        break;
                    }
            }

            if ppname == "gnome-terminal-" {
                self.name = "gnome-terminal".to_string();
                break;
            } else if ppname == "urxvtd" {
                self.name = "urxvt".to_string();
                break;
            } else {
                print!("is a terminal\n");
                self.name = ppname.split("/").last()
                    .unwrap().to_string();
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
