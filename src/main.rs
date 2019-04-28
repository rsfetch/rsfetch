// Crates
#[macro_use] extern crate prettytable;
extern crate clap;

// Use commands
use std::process::Command;
use prettytable::format;
use prettytable::Table;
use clap::{Arg, App};

// Main function
fn main() {
	// Variables
	let matches = App::new("fetch")
					.version("0.9.1")
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
	let window_manager = matches.value_of("window_manager").unwrap_or("true");
	let distro = matches.value_of("distro").unwrap_or("true");
	let kernel = matches.value_of("kernel").unwrap_or("true");
	let uptime = matches.value_of("uptime").unwrap_or("true");
	let packages = matches.value_of("packages").unwrap_or("true");
	let music = matches.value_of("music").unwrap_or("mpd");
	let logo = matches.value_of("logo").unwrap_or("true");
    let mut table = Table::new();
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
	// Output
	println!("");
	if logo == "true" {
		println!(" \\    / /\\   |    |    |--- \\   /");
		println!("  \\  / /__\\  |    |    |---  \\ /");
		println!("   \\/ /----\\ |___ |___ |---   |");
	}
	table.set_format(*format::consts::FORMAT_BORDERS_ONLY);
	if user == "true" {
		table.add_row(row!["USER", "=", String::from_utf8_lossy(&you.stdout)]);
	}
	if ip_address == "true" {
		table.add_row(row!["IP ADDRESS", "=", String::from_utf8_lossy(&ip.stdout)]);
	}
	if shell == "true" {
		table.add_row(row!["SHELL", "=", String::from_utf8_lossy(&sh.stdout)]);
	}
	if window_manager == "true" {
		table.add_row(row!["WINDOW MANAGER", "=", String::from_utf8_lossy(&wm.stdout)]);
	}
	if distro == "true" {
		table.add_row(row!["DISTRO", "=", String::from_utf8_lossy(&dist.stdout)]);
	}
	if kernel == "true" {
		table.add_row(row!["KERNEL", "=", String::from_utf8_lossy(&kern.stdout)]);
	}
	if uptime == "true" {
		table.add_row(row!["UPTIME", "=", String::from_utf8_lossy(&upt.stdout)]);
	}
	if packages == "true" {
		table.add_row(row!["PACKAGES", "=", String::from_utf8_lossy(&pkgs.stdout)]);
	}
	if music == "mpd" {
		let mus = Command::new("/usr/bin/bash")
					.arg("-c")
					.arg("mpc -f \"%artist% - (%date%) %album% - %title%\" | head -n1")
					.output()
					.expect("failed to execute process");
		table.add_row(row!["MUSIC (MPD)", "=", String::from_utf8_lossy(&mus.stdout)]);
	}
	table.printstd();;
	println!("");
}
