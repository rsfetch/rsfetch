use std::vec::Vec;
use prettytable::{ cell, format, row, Table };

struct KeyValue {
    key: String,
    val: String,
}

pub enum OutputType {
    Minimal,
    Rsfetch,
    Neofetch,
}

pub struct OutputOptions {
    output_type: OutputType,
    caps:        bool,
    bold:        bool,
    use_borders: bool,
    borders:     char,
}

impl OutputOptions {
    pub fn new(output: OutputType) -> OutputOptions {
        OutputOptions {
            output_type: output,
            caps:        true,
            bold:        true,
            use_borders: true,
            borders:     '■',
        }
    }

    pub fn caps(&mut self, v: bool) {
        self.caps = v;
    }

    pub fn bold(&mut self, v: bool) {
        self.bold = v;
    }

    pub fn use_borders(&mut self, v: bool) {
        self.use_borders = v;
    }

    pub fn borders(&mut self, c: usize) {
        self.borders = std::char::from_u32(c as u32).unwrap();
    }
}

pub struct OutputHelper {
    table:   Table,
    ascii:   String,
    options: OutputOptions,
    data:    Vec<KeyValue>,
}

fn bold(text: &str) -> String {
    let e: char = 0x1B as u8 as char;
    format!("{}[1m{}{}[0m", e, text, e)
}

impl OutputHelper {

    // initialize new OutputHelper
    pub fn new(options: OutputOptions) -> OutputHelper {
        let mut table = Table::new();
        let bdr = if options.output_type == OutputType::Minimal {
            ' '
        } else { '│' }
        let sep = if options.output_type == OutputType::Minimal {
            format::LineSeparator::new(' ', ' ', 
                                       options.border, options.border)
        } else {
            format::LineSeparator::new('─', '─',
                                       options.border, options.border)
        }

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
            table:   table,
            logo:    String::new(),
            options: options,
            data:    Vec::new(),
        }
    }

    pub fn add(&mut self, key: String, val: String) {
        let item = Keyval {
            key: key,
            val: val,
        }

        self.data.push(item);
    }

    pub fn ascii(&mut self, ascii: String) {
        self.ascii = ascii
    }
    pub fn output(&mut self) {
        // minimal output style
        if self.options.output_style == OutputType::Minimal {
            for thing in data {
                println!("{}", thing.val);
            }
        } else if self.options.output_style == OutputType::Rsfetch {
            // print logo
            println!("{}", bold(&self.ascii));

            // convert self.data to table, then print
            for thing in self.data {
                let mut key = thing.key.clone();
                let mut val = thing.val.clone();
                if !self.options.caps {
                    key = key.to_lowercase();
                }

                if !self.options.bold {
                    key = bold(&key);
                }

                if !self.options.use_border {
                    self.table.add_row(row![key, val]);
                } else {
                    self.table.add_row(row![key, "=", val]);
                }
            }
            self.table.printstd();
        } else if self.options.output_style == OutputType::Neofetch {
            // don't do anything
            // TODO: implement
        }
    }
}
