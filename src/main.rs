// crates
#[macro_use] extern crate prettytable;
extern crate clap;

// use commands
use std::char;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::io::prelude::*;
use std::process::Command;
use prettytable::format;
use prettytable::Table;
use clap::{Arg, App};
use std::path::PathBuf;
use std::env;

// escape character (U+001B)
const E: char = 0x1B as char;

fn makebold(text: &str) -> String {
	format!("{}[1m{}{}[0m", E, text, E)
}

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

fn printlogo(file: String) -> Result<()> {
    let fs = File::open(file)?;
    for line in BufReader::new(fs).lines() {
        println!("{}", makebold(&line?));
    }
	Ok(())
}

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
					.version("1.2.1")
					.about("\nFetches system info. Somewhat(?) minimalistic.\nAll \"BOOL\" options default to \"true\" (with the exception of separate package counts and editor), and \"SOURCE\" defaults to no.\n\nNote: If you set -P to \"true\", make sure to set -p to \"false\".")
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
					.arg(Arg::with_name("package_counts")
						.short("P")
						.long("package_counts")
						.value_name("BOOL")
						.help("Turn separate package counts on or off.")
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
	let package_counts = matches.value_of("package_counts").unwrap_or("false");
	let music = matches.value_of("music").unwrap_or("no");
	let logo = matches.value_of("logo").unwrap_or("true");
	let logofile = matches.value_of("logofile").unwrap_or("");
	// Output
	println!("");
	if logo == "true" {
		if logofile != "" {
			let _res = printlogo(logofile.to_string());
		} else {
			print_defaultlogo()
		}
		println!(""); // print a newline
	}
	let format;
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
			let pkgs = Command::new("/usr/bin/bash")
					.arg("-c")
					.arg("echo \"$(pacman -Q | wc -l)\"")
					.output()
					.expect("failed to execute process");
			table = addrow(table, abold, caps, borders, "PACKAGES", &String::from_utf8_lossy(&pkgs.stdout));
		} else if package_counts == "true" {
			let pkgs = Command::new("/usr/bin/bash")
					.arg("-c")
					.arg("echo \"$(pacman -Q | wc -l) (total) | $(paclist core | wc -l) (core), $(paclist extra | wc -l) (extra), $(paclist community | wc -l) (community), $(pacman -Qm | wc -l) (aur)\"")
					.output()
					.expect("failed to execute process");
			table = addrow(table, abold, caps, borders, "PACKAGES", &String::from_utf8_lossy(&pkgs.stdout));
		}
		if music == "mpd" {
			let mus = Command::new("/usr/bin/bash")
						.arg("-c")
						.arg("mpc -f \"%artist% - (%date%) %album% - %title%\" | head -n1")
						.output()
						.expect("failed to execute process");
			table = addrow(table, abold, caps, borders, "MUSIC (MPD)", &String::from_utf8_lossy(&mus.stdout));
		}
		table.printstd();;
	println!("");
}
