// External crates to be used.
#[macro_use] extern crate prettytable; // For a nice organized output.
extern crate clap; // For cmd line arguments.

// use commands.
use std::char;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::io::prelude::*;
use std::process::Command;
use std::process;
use prettytable::format;
use prettytable::Table;
use clap::{Arg, App};
use std::path::PathBuf;
use std::env;

// escape character (U+001B)
const E: char = 0x1B as char;

// Function for making bold text.
fn makebold(text: &str) -> String {
	format!("{}[1m{}{}[0m", E, text, E)
}

// Function for adding rows to the table.
fn addrow(
		mut table: Table, 
		abold: &str, 
		caps: &str,
		border: &str, 
		title: &str, 
		value: &str
	) -> Table {
	let mut title_str: String = title.to_string();
	if caps != "true" {
		title_str = title_str.to_lowercase();
	}
	if abold != "false" {
		title_str = makebold(&title_str);
	}
	if border != "true" {
		table.add_row(row![title_str, value]);
	} else {
		table.add_row(row![title_str, "=", value]);
	}
	return table
}

// For custom art.
fn printlogo(file: String) -> Result<()> {
    let fs = File::open(file)?;
    for line in BufReader::new(fs).lines() {
        println!("{}", makebold(&line?));
    }
	Ok(())
}

// Default art.
fn print_defaultlogo() {
	println!("{}", makebold(" \\    / /\\   |    |    |--- \\   /"));
	println!("{}", makebold("  \\  / /__\\  |    |    |---  \\ /"));
	println!("{}", makebold("   \\/ /----\\ |___ |___ |---   |"));
}

// Main function
fn main() {
	// Variables
	let mut table = Table::new();
	let matches = App::new("fetch")
					.version("1.3.0")
					.about("\nMy info fetch tool for Linux. Fast (0.01s - 0.2s execution time) and somewhat(?) minimal.\nAll \"BOOL\" options default to \"true\" (with the exception of editor), and \"SOURCE\" defaults to no.")
					.arg(Arg::with_name("credits")
						.long("credits")
						.value_name(" ")
						.help("Links to those who helped make this, and thanks to others who've helped me with my struggles.")
						.takes_value(false))
					.arg(Arg::with_name("bold")
						.short("b")
						.long("bold")
						.value_name("BOOL")
						.help("Turn bold for field titles on or off.")
						.takes_value(true))
					.arg(Arg::with_name("borders")
						.short("B")
						.long("borders")
						.value_name("BOOL")
						.help("Turn borders on or off.")
						.takes_value(true))
					.arg(Arg::with_name("caps")
						.short("c")
						.long("caps")
						.value_name("BOOL")
						.help("Turn all caps on or off.")
						.takes_value(true))
					.arg(Arg::with_name("user")
						.short("U")
						.long("user")
						.value_name("BOOL")
						.help("Turn user name on or off.")
						.takes_value(true))
					.arg(Arg::with_name("host")
						.short("h")
						.long("host")
						.value_name("BOOL")
						.help("Turn device name on or off.")
						.takes_value(true))
					.arg(Arg::with_name("ip_address")
						.short("i")
						.long("ip_address")
						.value_name("BOOL")
						.help("Turn ip address display on or off.")
						.takes_value(true))
					.arg(Arg::with_name("editor")
						.short("e")
						.long("editor")
						.value_name("BOOL")
						.help("Turn default editor name on or off. (Must have $EDITOR variable set.).")
						.takes_value(true))
					.arg(Arg::with_name("shell")
						.short("s")
						.long("shell")
						.value_name("BOOL")
						.help("Turn default shell name on or off.")
						.takes_value(true))
					.arg(Arg::with_name("window_manager")
						.short("w")
						.long("window_manager")
						.value_name("BOOL")
						.help("Turn window manager name on or off.")
						.takes_value(true))
					.arg(Arg::with_name("distro")
						.short("d")
						.long("distro")
						.value_name("BOOL")
						.help("Turn distro name on or off.")
						.takes_value(true))
					.arg(Arg::with_name("kernel")
						.short("k")
						.long("kernel")
						.value_name("BOOL")
						.help("Turn kernel info on or off.")
						.takes_value(true))
					.arg(Arg::with_name("uptime")
						.short("u")
						.long("uptime")
						.value_name("BOOL")
						.help("Turn uptime info on or off.")
						.takes_value(true))
					.arg(Arg::with_name("packages")
						.short("p")
						.long("packages")
						.value_name("BOOL")
						.help("Turn total package count on or off.")
						.takes_value(true))
					.arg(Arg::with_name("music")
						.short("m")
						.long("music")
						.value_name("SOURCE")
						.help("Choose where to get music info. Supported options are \"mpd\" (mpc) and no (none).\n")
						.takes_value(true))
					.arg(Arg::with_name("logo")
						.short("l")
						.long("logo")
						.value_name("BOOL")
						.help("Turn the logo (VALLEY) on or off.")
						.takes_value(true))
					.arg(Arg::with_name("logofile")
						.short("L")
						.long("logofile")
						.value_name("FILE")
						.help("Specifies the file from which to read a custom ASCII logo.")
						.takes_value(true))
					.arg(Arg::with_name("corners")
						.short("C")
						.long("corners")
						.value_name("CHARACTER")
						.help("Specifies the corner style. Choose either \"■\" or \"0\".")
						.takes_value(true))
					.get_matches();
	if matches.is_present("credits") {
		println!("");
        println!("Main Developer:   valley (Reddit: /u/Valley6660) (Github: Phate6660)");
        println!("Contributor:      kiedtl (Reddit: /u/kiedtl)     (Github: kiedtl)\n");
        println!("With thanks to:   \"/r/rust\", \"/u/tablair\", \"/u/kabocha_\" for all the help they gave; and the tool \"neofetch\" for giving me the inspiration to make this.");
        println!("");
        process::exit(0x0100); // Exit program here so that nothing else is output.
	}
	let caps = matches.value_of("caps").unwrap_or("true");
	let abold = matches.value_of("bold").unwrap_or("true");
	let corners = matches.value_of("corners").unwrap_or("■");
	let borders = matches.value_of("borders").unwrap_or("true");
	let user = matches.value_of("user").unwrap_or("true");
	let host = matches.value_of("host").unwrap_or("true");
	let ip_address = matches.value_of("ip_address").unwrap_or("true");
	let editor = matches.value_of("editor").unwrap_or("false");
	let shell = matches.value_of("shell").unwrap_or("true");
	let window_manager = matches.value_of("window_manager").unwrap_or("true");
	let distro = matches.value_of("distro").unwrap_or("true");
	let kernel = matches.value_of("kernel").unwrap_or("true");
	let uptime = matches.value_of("uptime").unwrap_or("true");
	let packages = matches.value_of("packages").unwrap_or("true");
	let music = matches.value_of("music").unwrap_or("no");
	let logo = matches.value_of("logo").unwrap_or("true");
	let logofile = matches.value_of("logofile").unwrap_or("");
	
	// Determine the logo to use.
	println!(""); // For a blank line before output.
	if logo == "true" {
		if logofile != "" {
			let _res = printlogo(logofile.to_string());
		} else {
			print_defaultlogo()
		}
		println!(""); // print a newline
	}
	let format;
	
	// Determine if borders are used, and if they are, the style of the corners.
	if borders == "true" {
		if corners == "■" {
			format = format::FormatBuilder::new()
				.column_separator(' ')
				.borders('│')
				.separators(&[format::LinePosition::Top,
					format::LinePosition::Bottom],
					format::LineSeparator::new('─', '─', '■', '■'))
				.padding(1, 1)
				.build();
			table.set_format(format);
		} else if corners == "0" {
			format = format::FormatBuilder::new()
				.column_separator(' ')
				.borders('│')
				.separators(&[format::LinePosition::Top,
					format::LinePosition::Bottom],
					format::LineSeparator::new('─', '─', '0', '0'))
				.padding(1, 1)
				.build();
			table.set_format(format);
		}
	} else {
		format = format::FormatBuilder::new()
			.column_separator(' ')
			.borders(' ')
			.separators(&[format::LinePosition::Top,
				format::LinePosition::Bottom],
				format::LineSeparator::new(' ', ' ', ' ', ' '))
			.padding(1, 1)
			.build();
		table.set_format(format);
	}
	// Begin output. Data for variables will *only* be collected if the option for that specific output is turned on. Therefore making the program much more efficient.
	if user == "true" {
		let you = Command::new("/usr/bin/whoami")
				.output()
				.expect("failed to execute process");
		table = addrow(table, abold, caps, borders, "USER", &String::from_utf8_lossy(&you.stdout));
	}
	if host == "true" {
		let mut file = File::open("/sys/devices/virtual/dmi/id/product_name").expect("Unable to open the file");
		let mut contents = String::new();
		file.read_to_string(&mut contents).expect("Unable to read the file");
		let mut dev = contents.to_string();
		let len = dev.len();
		dev.truncate(len - 1);
		assert_eq!(dev, dev);
		table = addrow(table, abold, caps, borders, "HOST", &dev);
	}
	if uptime == "true" {
		let upt = Command::new("/usr/bin/bash")
				.arg("-c")
				.arg("uptime -p | sed 's/up //'")
				.output()
				.expect("failed to execute process");
		table = addrow(table, abold, caps, borders, "UPTIME", &String::from_utf8_lossy(&upt.stdout));
	}
	if distro == "true" {
		let mut file = File::open("/etc/os-release").expect("Unable to open the file");
		let mut contents = String::new();
		file.read_to_string(&mut contents).expect("Unable to read the file");
		let thefile = contents;
		let dist = &thefile[31..41];
		table = addrow(table, abold, caps, borders, "DISTRO", dist);
	}
	if kernel == "true" {
		let kern = Command::new("/usr/bin/uname")
				.arg("-r")
				.output()
				.expect("failed to execute process");
		table = addrow(table, abold, caps, borders, "KERNEL", &String::from_utf8_lossy(&kern.stdout));
	}
	if window_manager == "true" {
		let mut path: PathBuf = env::var("HOME").expect("$HOME not set").into();
		path.push(".xinitrc");
		let file = File::open(path).expect("unable to open file");
		let reader = BufReader::new(file);
		let last_line = reader.lines().last()
			.expect("no last line")
			.expect("io error reading file");
		let word = last_line.to_string();
		let start_bytes = word.find(" ").unwrap();
		let result = &word[start_bytes..];
		let mut wm = result.to_string();
		assert_eq!(wm.remove(0), ' ');
		table = addrow(table, abold, caps, borders, "WINDOW MANAGER", &wm);
	}
	if editor == "true" {
		let ed = Command::new("/usr/bin/bash")
				.arg("-c")
				.arg("echo $EDITOR")
				.output()
				.expect("failed to execute process");
		table = addrow(table, abold, caps, borders, "EDITOR", &String::from_utf8_lossy(&ed.stdout));
	}
	if shell == "true" {
		let mut file = File::open("/etc/passwd").expect("Unable to open the file");
		let mut contents = String::new();
		file.read_to_string(&mut contents).expect("Unable to read the file");
		let thefile = contents;
		let sh = &thefile[692..701];
		table = addrow(table, abold, caps, borders, "SHELL", sh);
	}
	if ip_address == "true" {
		let ip = Command::new("/usr/bin/bash")
				.arg("-c")
				.arg("curl --silent http://ipecho.net/plain")
				.output()
				.expect("failed to execute process");
		table = addrow(table, abold, caps, borders, "IP ADDRESS", &String::from_utf8_lossy(&ip.stdout));
	}
	if packages == "true" {
		let mut pkgs = 0;
		let a = Command::new("/usr/bin/pacman")
					.arg("-Q")
					.output()
					.expect("failed to execute process");
		let b = &String::from_utf8_lossy(&a.stdout).to_string();
		fs::write("/tmp/packages", b).expect("Unable to write file");
		let file = BufReader::new(File::open("/tmp/packages").unwrap());
		for _line in file.lines() {
			pkgs = pkgs + 1;
		}
		let pkg = format!("{}", pkgs);
		table = addrow(table, abold, caps, borders, "PACKAGES", &pkg);
	}
	if music == "mpd" {
		let a = Command::new("/usr/bin/mpc")
					.arg("current")
					.arg("-f")
					.arg("%artist% - (%date%) %album% - %title%")
					.output()
					.expect("failed to execute process");
		let mus = &mut String::from_utf8_lossy(&a.stdout).to_string();
		let len = mus.len();
		mus.truncate(len - 1);
		assert_eq!(mus, mus);
		table = addrow(table, abold, caps, borders, "MUSIC (MPD)", mus);
	}
	// After collecting data for variables and adding the rows, print the final output into a custom table.
	table.printstd();;
	println!(""); // For a blank line after output.
}
