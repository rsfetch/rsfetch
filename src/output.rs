use std::vec::Vec;

const E: char = 0x1B as char;

#[derive(Clone)]
struct KeyValue {
    key: String,
    val: String,
}

#[derive(PartialEq, Clone, Debug)]
pub enum OutputType {
    Minimal,
    Rsfetch,
    Neofetch,
}

pub struct OutputOptions {
    pub output_type: OutputType,
    pub caps: bool,
    pub bold: bool,
    pub use_borders: bool,
    pub borders: char,
}

pub struct OutputHelper {
    ascii: String,
    options: OutputOptions,
    data: Vec<KeyValue>,
}

pub fn bold(text: &str) -> String {
    format!("{0}[1m{1}{0}[0m", E, text)
}

impl OutputHelper {
    // initialize new OutputHelper
    pub fn new(options: OutputOptions) -> OutputHelper {
        OutputHelper {
            ascii: String::new(),
            options,
            data: Vec::new(),
        }
    }

    pub fn add(&mut self, key: &str, val: &str) {
        let item = KeyValue {
            key: key.to_owned(),
            val: val.to_owned(),
        };

        self.data.push(item);
    }

    pub fn ascii(&mut self, ascii: String) {
        self.ascii = ascii;
    }

    pub fn output(&mut self) {
        // minimal output style
        if self.options.output_type == OutputType::Minimal {
            for thing in self.data.clone() {
                println!("{}", thing.val.replace("\n", ""));
            }

            println!();
        } else if self.options.output_type == OutputType::Rsfetch {
            // Print logo
            println!("{}", bold(&self.ascii));
            let mut data = self.data.clone();

            let mut max_len_key = 0;
            let mut max_len_val = 0;
            //Process data
            for j in &mut data {
                if self.options.bold {
                    j.key = bold(&j.key);
                }

                if !self.options.caps {
                    j.key = j.key.to_lowercase();
                }
                // Calculate length of key and value for padding
                if j.key.len() > max_len_key {
                    max_len_key = j.key.len();
                }
                // If not using borders, no need to calculate padding for `values`
                if self.options.use_borders{
                    if j.val.len() > max_len_val {
                        max_len_val = j.val.len();
                    }
                }
            }
            // Set most options for borders
            let mut border = "";
            let mut width = 0;
            if self.options.use_borders {
                border = "│";

                width = if self.options.bold {
                    max_len_key + max_len_val + 2
                } else {
                    max_len_key + max_len_val + 10
                };
                // Top border
                println!("{0}{1:─<2$}{0}", self.options.borders, "", width);
            }

            // Print data content
            for info in &data {
                println!("{0} {1:<3$} = {5:2}{2:<4$} {0}", border, info.key, info.val, max_len_key + 2, max_len_val + 1, "");
            }
            // Bottom border
            if self.options.use_borders {
                println!("{0}{1:─<2$}{0}", self.options.borders, "", width);
            } else {
                println!();
            }
        } else if self.options.output_type == OutputType::Neofetch {
            let mut width = 0;
            let mut key_width = 0;
            let ascii = self
                .ascii
                .clone()
                .split('\n')
                .map(|l| {
                    if l.len() > width {
                        width = l.len();
                    }

                    l.to_string()
                })
                .collect::<Vec<String>>();

            if !ascii.is_empty() {
                width += 2;
            }

            let stuff = self.data.clone();

            for i in &stuff {
                let key = &i.key;
                if key.len() > key_width {
                    key_width = key.len();
                }
            }

            key_width += 2;

            let mut printed = 0;
            for c in 0..stuff.len() {
                let thing = stuff[c].clone();
                let mut key = thing.key;
                let val = thing.val;

                if !self.options.caps {
                    key = key.to_lowercase();
                }

                // print logo
                if c < ascii.len() {
                    if self.options.bold {
                        print!("{}{}[{}C", bold(&ascii[c]), E, (width - ascii[c].len()));
                    } else {
                        print!("{}{}[{}C", ascii[c], E, (width - ascii[c].len()));
                    }
                } else {
                    print!("{}[{}C", E, width);
                }

                // print key and value
                if key != "" {
                    if self.options.bold {
                        print!("{}{}[{}C{}\n", bold(&key), E, (key_width - key.len()), val);
                    } else {
                        print!("{}{}[{}C{}\n", key, E, (key_width - key.len()), val);
                    }
                } else if self.options.bold {
                    print!("{}\n", bold(&val));
                } else {
                    print!("{}\n", val);
                }

                printed = c;
            }

            if ascii.len() > printed {
                for i in (printed + 1)..ascii.len() {
                    if self.options.bold {
                        print!("{}\n", bold(&ascii[i]));
                    } else {
                        print!("{}\n", ascii[i]);
                    }
                }
            }

            print!("\n"); // newline
        }
    }
}
