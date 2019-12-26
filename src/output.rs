use std::vec::Vec;
use prettytable::{ cell, format, row, Table };

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
    pub caps:        bool,
    pub bold:        bool,
    pub use_borders: bool,
    pub borders:     char,
}

pub struct OutputHelper {
    table:    Table,
    ascii:    String,
    options:  OutputOptions,
    data:     Vec<KeyValue>,
}

pub fn bold(text: &str) -> String {
    format!("{}[1m{}{}[0m", E, text, E)
}

impl OutputHelper {

    // initialize new OutputHelper
    pub fn new(options: OutputOptions) -> OutputHelper {
        let mut table = Table::new();
        let bdr = if options.output_type == OutputType::Minimal {
            ' '
        } else { '│' };

        let sep = if options.output_type == OutputType::Minimal {
            format::LineSeparator::new(' ', ' ',
                                       options.borders, options.borders)
        } else {
            format::LineSeparator::new('─', '─',
                                       options.borders, options.borders)
        };

        let format = format::FormatBuilder::new()
            .column_separator(' ')
            .borders(bdr)
            .separators(
                &[format::LinePosition::Top, format::LinePosition::Bottom],
                sep,
            )
            .padding(1, 1)
            .build();
        table.set_format(format);

        OutputHelper {
            table:    table,
            ascii:    String::new(),
            options:  options,
            data:     Vec::new(),
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
                println!("{}", thing.val);
            }

            print!("\n");

        } else if self.options.output_type == OutputType::Rsfetch {
            // print logo
            println!("{}", bold(&self.ascii));

            // print newline, if necessary
            let chr = self.ascii.clone().chars().last();
            match chr {
                Some(ch) => if (ch as u32) != 10 {
                    print!("\n");
                },
                None     => print!("\n"),
            }

            // convert self.data to table, then print
            for thing in self.data.clone() {
                let mut key = thing.key.clone();
                let val = thing.val.clone();

                if !self.options.caps {
                    key = key.to_lowercase();
                }

                if self.options.bold {
                    key = bold(&key);
                    self.ascii = bold(&self.ascii.clone());
                }

                if !self.options.use_borders {
                    self.table.add_row(row![key, val]);
                } else {
                    self.table.add_row(row![key, "=", val]);
                }
            }
            self.table.printstd();
        } else if self.options.output_type == OutputType::Neofetch {
            let mut width = 0;
            let mut key_width = 0;
            let ascii = self.ascii.clone()
                .split("\n")
                .map(|l| {
                    if l.len() > width {
                        width = l.len();
                    }

                    l.to_string()
                }).collect::<Vec<String>>();

            if ascii.len() > 0 {
                width += 2;
            }

            let stuff = self.data.clone();

            let _ = stuff.iter().map(|i| {
                let key = &i.key;
                if key.len() > key_width {
                    key_width = key.len();
                }
            }).collect::<()>();
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
                        print!("{}{}[{}C", bold(&ascii[c]), E,
                            (width - ascii[c].len()));
                    } else {
                        print!("{}{}[{}C", ascii[c], E,
                            (width - ascii[c].len()));
                    }
                } else {
                    print!("{}[{}C", E, width);
                }

                // print key and value
                if key != "" {
                    if self.options.bold {
                        print!("{}{}[{}C{}\n", bold(&key), E,
                               (key_width - key.len()), val);
                    } else {
                        print!("{}{}[{}C{}\n", key, E,
                               (key_width - key.len()), val);
                    }
                } else {
                    if self.options.bold {
                        print!("{}\n", bold(&val));
                    } else {
                        print!("{}\n", val);
                    }
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
