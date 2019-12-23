use crate::*;

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
            EnvItem::User  => self.user  = std::env::var("USER")
                .context(EnvError)?.to_string(),
            EnvItem::Shell => self.shell = {
                let sh = std::env::var("SHELL")
                    .context(EnvError)?;
                let sh_pieces = sh.split("/").collect::<Vec<&str>>();
                sh_pieces[sh_pieces.len() - 1].to_string()
            },

            // fallback to $env:SHELL
            EnvItem::Editor => match std::env::var("VISUAL") {
               Ok(v)  => self.editor = v.to_string(),
               Err(_) => self.editor = std::env::var("EDITOR")
                   .context(EnvError)?.to_string(),
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
