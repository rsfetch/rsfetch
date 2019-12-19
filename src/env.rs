use std::env;
use std::result::Result;

pub EnvItem {
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

    pub fn get(&mut self, item: EnvItem) -> Result<(), env::VarError> {
        match item {
            EnvItem::User  => self.user  = env::var("USER")?.to_string(),
            EnvItem::Shell => self.shell = Path::new(env::var("SHELL")?).file_name(),

            // fallback to $env:SHELL
            match env::var("VISUAL") {
               Ok(v)  => self.editor = v.to_string(),
               Err(e) => self.editor = env::var("EDITOR")?.to_string(),
            }
        }

        Ok(())
    }

    // format it
    pub fn format(&self, item: EnvItem) -> String {
        match item {
            EnvItem::User   => return self.user.clone(),
            EnvItem::Shell  => return self.shell.clone(),
            EnvItem::Editor => return self.editor.clone(),
            _ => (),
        }
    }
}
