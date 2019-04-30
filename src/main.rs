// Crates
#[macro_use] extern crate prettytable;
extern crate clap;

// use commands
use std::char;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::process::Command;
use prettytable::format;
use prettytable::Table;
use clap::{Arg, App};

// escape character (U+001B)
const E: char = 0x1B as char;

fn makebold(text: &str) -> String {
	format!("{}[1m{}{}[0m", E, text, E)
}

fn addrow(
		mut table: Table, 
		abold: &str, 
		caps: &str, 
		title: &str, 
		value: &str
	) -> Table {
	let mut title_str: String = title.to_string();
	if caps == "false" {
		title_str = title_str.to_lowercase();
	}
	if abold != "false" {
		title_str = makebold(&title_str);
	}
	table.add_row(row![title_str, "=", value]);
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
	let format = format::FormatBuilder::new()
						.column_separator(' ')
						.borders('│')
						.separators(&[format::LinePosition::Top,
							format::LinePosition::Bottom],
							format::LineSeparator::new('─', '─', '0', '0'))
						.padding(1, 1)
						.build();
	table.set_format(format);
	let matches = App::new("fetch")
					.version("1.0.0")
					.about("\nFetches system info. Somewhat(?) minimalistic.\nAll \"BOOL\" options default to \"true\" (with the exception of separate package counts and editor), and \"SOURCE\" defaults to no.\n\nNote: If you set -P to \"true\", make sure to set -p to \"false\".")
					.arg(Arg::with_name("bold")
						.short("b")
						.long("bold")
						.value_name("BOOL")
						.help("Turn bold for field titles on or off.")
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
					.arg(Arg::with_name("terminal")
						.short("t")
						.long("terminal")
						.value_name("BOOL")
						.help("Turn terminal display on or off. Requires \"term\", and you must be in the same directory as both fetch and term.")
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
						.value_name("STRING")
						.help("Specifies the file from which to read a custom ASCII logo.")
						.takes_value(true))
					.get_matches();
	let caps = matches.value_of("caps").unwrap_or("true");
	let abold = matches.value_of("bold").unwrap_or("true");
	let user = matches.value_of("user").unwrap_or("true");
	let host = matches.value_of("host").unwrap_or("true");
	let ip_address = matches.value_of("ip_address").unwrap_or("true");
	let editor = matches.value_of("editor").unwrap_or("false");
	let shell = matches.value_of("shell").unwrap_or("true");
	let terminal = matches.value_of("terminal").unwrap_or("true");
	let window_manager = matches.value_of("window_manager").unwrap_or("true");
	let distro = matches.value_of("distro").unwrap_or("true");
	let kernel = matches.value_of("kernel").unwrap_or("true");
	let uptime = matches.value_of("uptime").unwrap_or("true");
	let packages = matches.value_of("packages").unwrap_or("true");
	let package_counts = matches.value_of("package_counts").unwrap_or("false");
	let music = matches.value_of("music").unwrap_or("no");
	let logo = matches.value_of("logo").unwrap_or("true");
	let logofile = matches.value_of("logofile").unwrap_or("");
	let you = Command::new("/usr/bin/whoami")
					.output()
					.expect("failed to execute process");
	let dev = Command::new("/usr/bin/bash")
					.arg("-c")
					.arg("echo $(< /sys/devices/virtual/dmi/id/product_name)")
					.output()
					.expect("failed to execute process");
	let ed = Command::new("/usr/bin/bash")
					.arg("-c")
					.arg("echo $EDITOR")
					.output()
					.expect("failed to execute process");
	let sh = Command::new("/usr/bin/bash")
					.arg("-c")
					.arg("grep $USER /etc/passwd | sed 's/.*://'")
					.output()
					.expect("failed to execute process");
	let upt = Command::new("/usr/bin/bash")
					.arg("-c")
					.arg("uptime -p | sed 's/up //'")
					.output()
					.expect("failed to execute process");
	let dist = Command::new("/usr/bin/bash")
					.arg("-c")
					.arg("grep PRETTY /etc/os-release | grep -o '\".*\"' | sed 's/\"//g' | awk 'NR>1{print PREV} {PREV=$0} END{printf(\"%s\",$0)}'")
					.output()
					.expect("failed to execute process");
	let kern = Command::new("/usr/bin/uname")
					.arg("-r")
					.output()
					.expect("failed to execute process");
	let wm = Command::new("/usr/bin/bash")
					.arg("-c")
					.arg("tail -n 1 \"${HOME}/.xinitrc\" | cut -d ' ' -f 2")
					.output()
					.expect("failed to execute process");
	let ip = Command::new("/usr/bin/bash")
					.arg("-c")
					.arg("curl --silent http://ipecho.net/plain")
					.output()
					.expect("failed to execute process");
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
		if user == "true" {
			table = addrow(table, abold, caps, "USER", &String::from_utf8_lossy(&you.stdout));
		}
		if host == "true" {
			table = addrow(table, abold, caps, "HOST", &String::from_utf8_lossy(&dev.stdout));
		}
		if uptime == "true" {
			table = addrow(table, abold, caps, "UPTIME", &String::from_utf8_lossy(&upt.stdout));
		}
		if distro == "true" {
			table = addrow(table, abold, caps, "DISTRO", &String::from_utf8_lossy(&dist.stdout));
		}
		if kernel == "true" {
			table = addrow(table, abold, caps, "KERNEL", &String::from_utf8_lossy(&kern.stdout));
		}
		if window_manager == "true" {
			table = addrow(table, abold, caps, "WINDOW MANAGER", &String::from_utf8_lossy(&wm.stdout));
		}
		if editor == "true" {
			table = addrow(table, abold, caps, "EDITOR", &String::from_utf8_lossy(&ed.stdout));
		}
		if shell == "true" {
			table = addrow(table, abold, caps, "SHELL", &String::from_utf8_lossy(&sh.stdout));
		}
		if terminal == "true" {
			let term = Command::new("/usr/bin/bash")
					.arg("-c")
					.arg("./term") // Yes, I cheated. I used a bash script to find the name of the term. I feel deeply saddened. :(
					.output()
					.expect("failed to execute process");
			table = addrow(table, abold, caps, "TERMINAL", &String::from_utf8_lossy(&term.stdout));
		}
		if ip_address == "true" {
			table = addrow(table, abold, caps, "IP ADDRESS", &String::from_utf8_lossy(&ip.stdout));
		}
		if packages == "true" {
			let pkgs = Command::new("/usr/bin/bash")
					.arg("-c")
					.arg("echo \"$(pacman -Q | wc -l)\"")
					.output()
					.expect("failed to execute process");
			table = addrow(table, abold, caps, "PACKAGES", &String::from_utf8_lossy(&pkgs.stdout));
		} else if package_counts == "true" {
			let pkgs = Command::new("/usr/bin/bash")
					.arg("-c")
					.arg("echo \"$(pacman -Q | wc -l) (total) | $(paclist core | wc -l) (core), $(paclist extra | wc -l) (extra), $(paclist community | wc -l) (community), $(pacman -Qm | wc -l) (aur)\"")
					.output()
					.expect("failed to execute process");
			table = addrow(table, abold, caps, "PACKAGES", &String::from_utf8_lossy(&pkgs.stdout));
		}
		if music == "mpd" {
			let mus = Command::new("/usr/bin/bash")
						.arg("-c")
						.arg("mpc -f \"%artist% - (%date%) %album% - %title%\" | head -n1")
						.output()
						.expect("failed to execute process");
			table = addrow(table, abold, caps, "MUSIC (MPD)", &String::from_utf8_lossy(&mus.stdout));
		}
		table.printstd();;
	println!("");
}