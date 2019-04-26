use std::process::Command;
use prettytable::format;

#[macro_use] extern crate prettytable;
use prettytable::{Table};

// Main function
fn main() {
	// Variables
	let mut table = Table::new();
	let user = Command::new("/usr/bin/whoami")
					.output()
					.expect("failed to execute process");
	let shell = Command::new("/usr/bin/bash")
					.arg("-c")
					.arg("grep $USER /etc/passwd | sed 's/.*://'")
					.output()
					.expect("failed to execute process");
	let uptime = Command::new("/usr/bin/bash")
					.arg("-c")
					.arg("uptime -p | sed 's/up //'")
					.output()
					.expect("failed to execute process");
	let distro = Command::new("/usr/bin/bash")
					.arg("-c")
					.arg("grep PRETTY /etc/os-release | grep -o '\".*\"' | sed 's/\"//g' | awk 'NR>1{print PREV} {PREV=$0} END{printf(\"%s\",$0)}'")
					.output()
					.expect("failed to execute process");
	let kernel = Command::new("/usr/bin/uname")
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
	println!(" \\    / /\\   |    |    |--- \\   /");
	println!("  \\  / /__\\  |    |    |---  \\ /");
	println!("   \\/ /----\\ |___ |___ |---   |");
	table.set_format(*format::consts::FORMAT_BORDERS_ONLY);
	table.add_row(row!["USER", "=", String::from_utf8_lossy(&user.stdout)]);
	table.add_row(row!["IP ADDRESS", "=", String::from_utf8_lossy(&ip.stdout)]);
	table.add_row(row!["SHELL", "=", String::from_utf8_lossy(&shell.stdout)]);
	table.add_row(row!["WINDOW MANAGER", "=", String::from_utf8_lossy(&wm.stdout)]);
	table.add_row(row!["DISTRO", "=", String::from_utf8_lossy(&distro.stdout)]);
	table.add_row(row!["KERNEL", "=", String::from_utf8_lossy(&kernel.stdout)]);
	table.add_row(row!["UPTIME", "=", String::from_utf8_lossy(&uptime.stdout)]);
	table.add_row(row!["PACKAGES", "=", String::from_utf8_lossy(&pkgs.stdout)]);
	table.printstd();;
	println!("");
}
