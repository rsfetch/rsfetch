use crate::*;
use std::process::Command;

pub enum EnvItem {
    User,
    Shell,
    Editor,
}

pub struct EnvInfo {
    user:   String,
    shell:  String,
    editor: String,
}

impl EnvInfo {
    pub fn new() -> EnvInfo {
        EnvInfo {
            user:   String::new(),
            shell:  String::new(),
            editor: String::new(),
        }
    }

    pub fn get(&mut self, item: EnvItem) -> Result<()> {
        match item {
            EnvItem::User  => 
            {
                let command = Command::new("id").arg("-un").output();
                match command {
                    Ok(o)  => {
                        let mut user = String::new();
                        let _ = o.stdout.iter().map(|b| {
                            user.push(*b as char);
                        }).collect::<()>();

                        if user != "" {
                            self.user = user;
                            return Ok(());
                        }
                    },
                    Err(_) => (),
                }

                // fallback to reading the USER variable
                self.user  = std::env::var("USER")
                .context(EnvError)?.to_string();
                self.user = self.user.clone().trim().to_string();
            },
            EnvItem::Shell => self.shell = {
                let sh = std::env::var("SHELL")
                    .context(EnvError)?;
                let sh_pieces = sh.split("/").collect::<Vec<&str>>();
                sh_pieces[sh_pieces.len() - 1].trim().to_string()
            },

            // fallback to $env:SHELL
            EnvItem::Editor => match std::env::var("VISUAL") {
               Ok(v)  => self.editor = v.trim().to_string(),
               Err(_) => self.editor = std::env::var("EDITOR")
                   .context(EnvError)?.trim().to_string(),
            },
        }

        Ok(())
    }

    // format it
    pub fn format(&self, item: EnvItem) -> String {
        match item {
            EnvItem::User   => return self.user.clone(),
            EnvItem::Shell  => return self.shell.clone(),
            EnvItem::Editor => return self.editor.clone(),
        }
    }
}
