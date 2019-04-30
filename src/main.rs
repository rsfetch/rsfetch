// Crates
#[macro_use] extern crate prettytable;
extern crate clap;

// use commands
use std::char;
use std::process::Command;
use prettytable::format;
use prettytable::Table;
use clap::{Arg, App};

// escape character (U+001B)
const E: char = 0x1B as char;

fn bold(text: &str) -> String {
	format!("{}[1m{}{}[0m", E, text, E)
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
					.version("0.9.3")
					.about("\nFetches system info. Somewhat(?) minimalistic.\nAll \"BOOL\" options default to \"true\", and \"SOURCE\" defaults to mpd.")
					.arg(Arg::with_name("user")
						.short("U")
						.long("user")
						.value_name("BOOL")
						.help("Turn user name on or off. Supported options are \"true\" and \"false\".")
						.takes_value(true))
					.arg(Arg::with_name("ip_address")
						.short("i")
						.long("ip_address")
						.value_name("BOOL")
						.help("Turn ip address display on or off. Supported options are \"true\" and \"false\".")
						.takes_value(true))
					.arg(Arg::with_name("shell")
						.short("s")
						.long("shell")
						.value_name("BOOL")
						.help("Turn default shell name on or off. Supported options are \"true\" and \"false\".")
						.takes_value(true))
					.arg(Arg::with_name("terminal")
						.short("t")
						.long("terminal")
						.value_name("BOOL")
						.help("Turn terminal display on or off. Supported options are \"true\" and \"false\".")
						.takes_value(true))
					.arg(Arg::with_name("window_manager")
						.short("w")
						.long("window_manager")
						.value_name("BOOL")
						.help("Turn window manager name on or off. Supported options are \"true\" and \"false\".")
						.takes_value(true))
					.arg(Arg::with_name("distro")
						.short("d")
						.long("distro")
						.value_name("BOOL")
						.help("Turn distro name on or off. Supported options are \"true\" and \"false\".")
						.takes_value(true))
					.arg(Arg::with_name("kernel")
						.short("k")
						.long("kernel")
						.value_name("BOOL")
						.help("Turn kernel info on or off. Supported options are \"true\" and \"false\".")
						.takes_value(true))
					.arg(Arg::with_name("uptime")
						.short("u")
						.long("uptime")
						.value_name("BOOL")
						.help("Turn uptime info on or off. Supported options are \"true\" and \"false\".")
						.takes_value(true))
					.arg(Arg::with_name("packages")
						.short("p")
						.long("packages")
						.value_name("BOOL")
						.help("Turn package counts on or off. Supported options are \"true\" and \"false\".")
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
						.help("Turn the logo (VALLEY) on or off. Supported options are \"true\" and \"false\".")
						.takes_value(true))
					.get_matches();
	let user = matches.value_of("user").unwrap_or("true");
	let ip_address = matches.value_of("ip_address").unwrap_or("true");
	let shell = matches.value_of("shell").unwrap_or("true");
	let terminal = matches.value_of("terminal").unwrap_or("true");
	let window_manager = matches.value_of("window_manager").unwrap_or("true");
	let distro = matches.value_of("distro").unwrap_or("true");
	let kernel = matches.value_of("kernel").unwrap_or("true");
	let uptime = matches.value_of("uptime").unwrap_or("true");
	let packages = matches.value_of("packages").unwrap_or("true");
	let music = matches.value_of("music").unwrap_or("mpd");
	let logo = matches.value_of("logo").unwrap_or("true");
	let you = Command::new("/usr/bin/whoami")
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
	let pkgs = Command::new("/usr/bin/bash")
					.arg("-c")
					.arg("echo \"$(pacman -Q | wc -l) (total) | $(paclist core | wc -l) (core), $(paclist extra | wc -l) (extra), $(paclist community | wc -l) (community), $(pacman -Qm | wc -l) (aur)\"")
					.output()
					.expect("failed to execute process");
	let ip = Command::new("/usr/bin/bash")
					.arg("-c")
					.arg("curl --silent http://ipecho.net/plain")
					.output()
					.expect("failed to execute process");
	let term = Command::new("/usr/bin/bash")
					.arg("-c")
					.arg("./term") // Yes, I cheated. I used a bash script to find the name of the term. I feel deeply saddened. :(
					.output()
					.expect("failed to execute process");
	// Output
	println!("");
	if logo == "true" {
		println!("{}", bold(" \\    / /\\   |    |    |--- \\   /"));
		println!("{}", bold("  \\  / /__\\  |    |    |---  \\ /"));
		println!("{}", bold("   \\/ /----\\ |___ |___ |---   |"));
	}
	if user == "true" {
		table.add_row(row![bold("USER"), "=", String::from_utf8_lossy(&you.stdout)]);
	}
	if ip_address == "true" {
		table.add_row(row![bold("IP ADDRESS"), "=", String::from_utf8_lossy(&ip.stdout)]);
	}
	if shell == "true" {
		table.add_row(row![bold("SHELL"), "=", String::from_utf8_lossy(&sh.stdout)]);
	}
	if terminal == "true" {
		table.add_row(row![bold("TERMINAL"), "=", String::from_utf8_lossy(&term.stdout)]);
	}
	if window_manager == "true" {
		table.add_row(row![bold("WINDOW MANAGER"), "=", String::from_utf8_lossy(&wm.stdout)]);
	}
	if distro == "true" {
		table.add_row(row![bold("DISTRO"), "=", String::from_utf8_lossy(&dist.stdout)]);
	}
	if kernel == "true" {
		table.add_row(row![bold("KERNEL"), "=", String::from_utf8_lossy(&kern.stdout)]);
	}
	if uptime == "true" {
		table.add_row(row![bold("UPTIME"), "=", String::from_utf8_lossy(&upt.stdout)]);
	}
	if packages == "true" {
		table.add_row(row![bold("PACKAGES"), "=", String::from_utf8_lossy(&pkgs.stdout)]);
	}
	if music == "mpd" {
		let mus = Command::new("/usr/bin/bash")
					.arg("-c")
					.arg("mpc -f \"%artist% - (%date%) %album% - %title%\" | head -n1")
					.output()
					.expect("failed to execute process");
		table.add_row(row![bold("MUSIC (MPD)"), "=", String::from_utf8_lossy(&mus.stdout)]);
	}
	table.printstd();;
	println!("");
}
