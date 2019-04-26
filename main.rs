use std::process::Command;

// Main function
fn main() {
	// Variables
	let user = Command::new("/usr/bin/whoami")
					.output()
					.expect("failed to execute process");
	let shell = Command::new("/usr/bin/bash")
					.arg("-c")
					.arg("grep $USER /etc/passwd | sed 's/.*://'")
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
	// Variables for package counts.
	let pkgs = Command::new("/usr/bin/bash")
					.arg("-c")
					.arg("echo \"$(pacman -Q | wc -l) (total) | $(paclist core | wc -l) (core), $(paclist extra | wc -l) (extra), $(paclist community | wc -l) (community), $(pacman -Qm | wc -l) (aur)\"")
					.output()
					.expect("failed to execute process");
	// Output
	println!("\\    / /\\   |    |    |--- \\   /     |");
	println!(" \\  / /__\\  |    |    |---  \\ /      |");
	println!("  \\/ /----\\ |___ |___ |---   |       |");
	println!("--------------------------------------");
	print!("USER             =   {}", String::from_utf8_lossy(&user.stdout));
	print!("SHELL            =   {}", String::from_utf8_lossy(&shell.stdout));
	print!("WINDOW MANAGER   =   {}", String::from_utf8_lossy(&wm.stdout));
	println!("DISTRO           =   {}", String::from_utf8_lossy(&distro.stdout));
	print!("KERNEL           =   {}", String::from_utf8_lossy(&kernel.stdout));
	print!("PACKAGES         =   {}", String::from_utf8_lossy(&pkgs.stdout));
}
