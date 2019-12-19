// TODO: refactor this hairy mess into multiple files,
// with structures and `impl`s.
// ALSO TODO: replace reqwest with a lighter crate :(
// -- kiedtl

mod cpu;
mod uptime;
mod device;

use clap::{App, Arg};
use log::error;
use prettytable::{cell, format, row, Table};
use snafu::{OptionExt, ResultExt, Snafu};
use std::env;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::process::Command;
use std::result;

use crate::cpu::*;
use crate::uptime::*;
use crate::device::*;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Could not retrieve device name: {}", source))]
    DeviceName { source: io::Error },
    #[snafu(display("Could not read the OS release: {}", source))]
    OsRelease { source: io::Error },
    #[snafu(display("Could not read the kernel version: {}", source))]
    KernelVersion { source: io::Error },
    #[snafu(display("Could not read the logo file: {}", source))]
    ReadLogo { source: io::Error },
    #[snafu(display("Could not format uptime: {}", source))]
    FormatUptime { source: fmt::Error },
    #[snafu(display("Could not determine home directory"))]
    HomeDir,
    #[snafu(display("Could not open .xinitrc: {}", source))]
    OpenXInitRc { source: io::Error },
    #[snafu(display("Empty .xinitrc"))]
    EmptyXInitRc,
    #[snafu(display("Could not read .xinitrc: {}", source))]
    ReadXInitRc { source: io::Error },
    #[snafu(display("Could not guess window manager"))]
    GuessWm,
    #[snafu(display("Could not determine editor"))]
    Editor { source: env::VarError },
    #[snafu(display("Could not retrieve IP address: {}", source))]
    Reqwest { source: reqwest::Error },
    #[snafu(display(
        "Could not retrieve package count. Perhaps you input the wrong package manager?"
    ))]
    Pkgcount { source: io::Error },
    #[snafu(display("Could not run mpc"))]
    Mpc { source: io::Error },
}

type Result<T, E = Error> = result::Result<T, E>;

// escape character (U+001B)
const E: char = '\x1B';

// Function for making bold text.
fn make_bold(text: &str) -> String {
    format!("{}[1m{}{}[0m", E, text, E)
}

// Function for adding rows to the table.
fn add_row(table: &mut Table, bold: bool, caps: bool, border: bool, title: &str, value: &str) {
    let mut title_str = title.to_string();
    if !caps {
        title_str = title_str.to_lowercase();
    }
    if bold {
        title_str = make_bold(&title_str);
    }
    if !border {
        table.add_row(row![title_str, value]);
    } else {
        table.add_row(row![title_str, "=", value]);
    }
}

// For custom art.
fn print_logo(file: &str) -> Result<()> {
    let fs = File::open(file).context(ReadLogo)?;
    for line in BufReader::new(fs).lines() {
        println!("{}", make_bold(&line.context(ReadLogo)?));
    }
    Ok(())
}

// Default art.
fn print_default_logo() {
    println!("{}", make_bold(" \\    / /\\   |    |    |--- \\   /"));
    println!("{}", make_bold("  \\  / /__\\  |    |    |---  \\ /"));
    println!("{}", make_bold("   \\/ /----\\ |___ |___ |---   |"));
}

fn get_value(key: &str, line: &str) -> Option<String> {
    if line.starts_with(key) {
        Some(
            line[key.len() + 1..line.len() - 1]
                .trim_matches('"')
                .to_string(),
        )
    } else {
        None
    }
}

fn get_os_release() -> Result<Option<String>> {
    let file = File::open("/etc/os-release").context(OsRelease)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut name = None;
    let mut pretty_name = None;
    while reader.read_line(&mut line).context(OsRelease)? > 0 {
        if let Some(val) = get_value("NAME", &line) {
            name = Some(val);
        } else if let Some(val) = get_value("PRETTY_NAME", &line) {
            pretty_name = Some(val);
            break;
        }
        line.clear();
    }
    Ok(pretty_name.or(name))
}

fn get_kernel_version() -> Result<String> {
    let contents = fs::read_to_string("/proc/sys/kernel/osrelease").context(KernelVersion)?;
    let kern = contents.trim_end().to_string();
    Ok(kern)
}

fn count_lines(data: Vec<u8>) -> usize {
    let mut count: usize = 0;

    // convert srcs from Vec<u8> to String
    let mut src = "".to_owned();
    for byte in data {
        src = format!("{}{}", src, byte as char);
    }

    let _ = src.split("\n").map(|_| count += 1).collect::<()>();
    count
}

fn get_window_manager() -> Result<String> {
    if let Some(de) = env::var_os("XDG_DESKTOP_SESSION")
        .or_else(|| env::var_os("XDG_CURRENT_DESKTOP"))
        .or_else(|| env::var_os("DESKTOP_SESSION"))
        .map(|s| s.to_string_lossy().into_owned())
    {
        return Ok(de);
    }

    let mut path = dirs::home_dir().context(HomeDir)?;
    path.push(".xinitrc");
    let file = File::open(path).context(OpenXInitRc)?;
    let reader = BufReader::new(file);
    let last_line = reader
        .lines()
        .last()
        .context(EmptyXInitRc)?
        .context(ReadXInitRc)?;
    let space = last_line.find(' ').context(GuessWm)?;
    let wm = last_line[space + 1..].to_string();
    Ok(wm)
}

fn get_editor() -> Result<String> {
    let ed = env::var("EDITOR").context(Editor)?.to_string();
    Ok(ed)
}

fn get_ip_address() -> Result<String> {
    let ip = reqwest::get("https://ipecho.net/plain")
        .context(Reqwest)?
        .text()
        .context(Reqwest)?;
    Ok(ip)
}

fn get_package_count_arch_based() -> Result<String> {
    let pacman = Command::new("pacman")
        .arg("-Qq")
        .output()
        .context(Pkgcount)?;
    let pkgs = count_lines(pacman.stdout);
    let pkg = format!("{}", pkgs);
    Ok(pkg)
}

fn get_package_count_debian_based() -> Result<String> {
    let apt = Command::new("apt").arg("list").output().context(Pkgcount)?;
    let pkgs = count_lines(apt.stdout);
    let pkg = format!("{}", pkgs);
    Ok(pkg)
}

fn get_package_count_void() -> Result<String> {
    let xbps = Command::new("xbps-query")
        .arg("-l")
        .output()
        .context(Pkgcount)?;
    let pkgs = count_lines(xbps.stdout);
    let pkg = format!("{}", pkgs);
    Ok(pkg)
}

fn get_package_count_fedora() -> Result<String> {
    let dnf = Command::new("dnf")
        .arg("list")
        .arg("--installed")
        .output()
        .context(Pkgcount)?;
    let pkgs = count_lines(dnf.stdout);
    let pkgs = pkgs - 1;
    let pkg = format!("{}", pkgs);
    Ok(pkg)
}

fn get_package_count_bsd() -> Result<String> {
    let bpkg = Command::new("pkg").arg("info").output().context(Pkgcount)?;
    let pkgs = count_lines(bpkg.stdout);
    let pkg = format!("{}", pkgs);
    Ok(pkg)
}

fn get_package_count_solus() -> Result<String> {
    let eopkg = Command::new("eopkg")
        .arg("list-installed")
        .output()
        .context(Pkgcount)?;
    let pkgs = count_lines(eopkg.stdout);
    let pkg = format!("{}", pkgs);
    Ok(pkg)
}

fn get_package_count_suse() -> Result<String> {
    let rpm = Command::new("rpm").arg("-qa").output().context(Pkgcount)?;
    let pkgs = count_lines(rpm.stdout);
    let pkg = format!("{}", pkgs);
    Ok(pkg)
}

fn get_package_count_alpine() -> Result<String> {
    let apk = Command::new("apk").arg("info").output().context(Pkgcount)?;
    let pkgs = count_lines(apk.stdout);
    let pkg = format!("{}", pkgs);
    Ok(pkg)
}

fn get_package_count_gentoo() -> Result<String> {
    let qlist = Command::new("qlist").arg("-I").output().context(Pkgcount)?;
    let pkgs = count_lines(qlist.stdout);
    let pkg = format!("{}", pkgs);
    Ok(pkg)
}

fn get_package_count_pip() -> Result<String> {
    let pip = Command::new("pip").arg("list").output().context(Pkgcount)?;
    let pkgs = count_lines(pip.stdout);
    let pkgs = pkgs - 2;
    let pkg = format!("{}", pkgs);
    Ok(pkg)
}

fn get_package_count_cargo() -> Result<String> {
    let cargo = Command::new("cargo")
        .arg("list")
        .output()
        .context(Pkgcount)?;
    let pkgs = count_lines(cargo.stdout);
    let pkgs = pkgs - 1;
    let pkg = format!("{}", pkgs);
    Ok(pkg)
}

fn get_mpd_song() -> Result<String> {
    let mpc = Command::new("mpc")
        .arg("current")
        .arg("-f")
        .arg("%artist% - (%date%) %album% - %title%")
        .output()
        .context(Mpc)?;
    let mut mus = String::from_utf8_lossy(&mpc.stdout).into_owned();
    mus.pop();
    Ok(mus)
}

//fn format_duration(duration: Duration) -> Result<String> {
//    let mut duration = duration.as_secs();
//    if duration < 60 {
//       let s = if duration == 1 {
//            String::from("1 second")
//        } else {
//            format!("{} seconds", duration)
//        };
//        return Ok(s);
//    }
//
//    duration /= 60;
//    let minutes = duration % 60;
//    duration /= 60;
//    let hours = duration % 24;
//    duration /= 24;
//    let days = duration % 7;
//    let weeks = (duration / 7) % 52;
//    duration /= 365;
//    let years = duration % 10;
//    let decades = duration / 10;
//
//    let mut s = String::new();
//    let mut comma = false;
//    fn add_part(comma: &mut bool, mut s: &mut String, name: &str, value: u64) -> Result<()> {
//        if value > 0 {
//            if *comma {
//                s.push_str(", ");
//            }
//            itoa::fmt(&mut s, value).context(FormatUptime)?;
//            s.push(' ');
//            s.push_str(name);
//            if value > 1 {
//                s.push('s');
//            }
//            *comma = true;
//        }
//        Ok(())
//    }
//    add_part(&mut comma, &mut s, "decade", decades)?;
//    add_part(&mut comma, &mut s, "year", years)?;
//    add_part(&mut comma, &mut s, "week", weeks)?;
//    add_part(&mut comma, &mut s, "day", days)?;
//    add_part(&mut comma, &mut s, "hour", hours)?;
//    add_part(&mut comma, &mut s, "minute", minutes)?;
//    Ok(s)
//}

fn get_packages(packages: &str) -> Result<String> {
    match packages {
        "pacman" => get_package_count_arch_based(),
        "apt" => get_package_count_debian_based(),
        "xbps" => get_package_count_void(),
        "dnf" => get_package_count_fedora(),
        "pkg" => get_package_count_bsd(),
        "eopkg" => get_package_count_solus(),
        "rpm" => get_package_count_suse(),
        "apk" => get_package_count_alpine(),
        "portage" => get_package_count_gentoo(),
        "pip" => get_package_count_pip(),
        "cargo" => get_package_count_cargo(),
        _ => unreachable!(),
    }
}

// Main function
fn main() {
    pretty_env_logger::init();
    // Variables
    let mut table = Table::new();
    let matches = App::new("rsfetch")
                    .version("1.9.0")
                    .about("\nMy info fetch tool for Linux. Fast (1ms execution time) and somewhat(?) minimal.\n\nAll options are on (with the exception of package count, editor, window manager, and ip address). Music info is turned off by default.\n\nAccepted values for the package manager are \"pacman\", \"apt\", \"xbps\", \"dnf\", \"pkg\", \"eopkg\", \"rpm\", \"apk\", \"pip\", \"portage\", and \"cargo\".")
                    .arg(Arg::with_name("credits")
                        .long("credits")
                        .value_name(" ")
                        .help("Links to those who helped make this, and thanks to others who've helped me.")
                        .takes_value(false))
                    .arg(Arg::with_name("no-bold")
                        .short("b")
                        .long("no-bold")
                        .help("Turn bold for field titles off.")
                        .takes_value(false))
                    .arg(Arg::with_name("no-borders")
                        .short("B")
                        .long("no-borders")
                        .help("Turn borders off.")
                        .takes_value(false))
                    .arg(Arg::with_name("no-caps")
                        .short("c")
                        .long("no-caps")
                        .help("Turn all caps off.")
                        .takes_value(false))
                    .arg(Arg::with_name("cpu")
                         .long("cpu")
                         .help("Turn CPU information (model, frequency, and processor count) on.")
                         .takes_value(false))
                    .arg(Arg::with_name("no-user")
                        .short("U")
                        .long("no-user")
                        .help("Turn user name off.")
                        .takes_value(false))
                    .arg(Arg::with_name("no-host")
                        .short("h")
                        .long("no-host")
                        .help("Turn device name off.")
                        .takes_value(false))
                    .arg(Arg::with_name("ip_address")
                        .short("i")
                        .long("ip_address")
                        .help("Turn ip address display on.")
                        .takes_value(false))
                    .arg(Arg::with_name("editor")
                        .short("e")
                        .long("editor")
                        .help("Turn default editor name on. (Must have $EDITOR variable set.).")
                        .takes_value(false))
                    .arg(Arg::with_name("no-shell")
                        .short("s")
                        .long("no-shell")
                        .help("Turn default shell name off.")
                        .takes_value(false))
                    .arg(Arg::with_name("no-wm-de")
                        .short("w")
                        .long("no-wm-de")
                        .help("Turn window manager or desktop environment name off.")
                        .takes_value(false))
                    .arg(Arg::with_name("no-distro")
                        .short("d")
                        .long("no-distro")
                        .help("Turn distro name off.")
                        .takes_value(false))
                    .arg(Arg::with_name("no-kernel")
                        .short("k")
                        .long("no-kernel")
                        .help("Turn kernel version off.")
                        .takes_value(false))
                    .arg(Arg::with_name("no-uptime")
                        .short("u")
                        .long("no-uptime")
                        .help("Turn uptime info off.")
                        .takes_value(false))
                    .arg(Arg::with_name("minimal")
                        .short("M")
                        .long("minimal")
                        .help("Turn minimal mode on.")
                        .takes_value(false))
                    .arg(Arg::with_name("packages")
                        .short("p")
                        .long("packages")
                        .value_name("PKG MNGR")
                        .help("Turn total package count on.")
                        .takes_value(true))
                    .arg(Arg::with_name("music")
                        .short("m")
                        .long("music")
                        .value_name("SOURCE")
                        .help("Choose where to get music info. Supported options are \"mpd\" (mpc) and no (none).\n")
                        .takes_value(true))
                    .arg(Arg::with_name("no-logo")
                        .short("l")
                        .long("no-logo")
                        .help("Turn the logo or ascii art off.")
                        .takes_value(false))
                    .arg(Arg::with_name("logofile")
                        .short("L")
                        .long("logofile")
                        .value_name("FILE")
                        .help("Specify the file from which to read a custom ASCII logo.")
                        .takes_value(true))
                    .arg(Arg::with_name("corners")
                        .short("C")
                        .long("corners")
                        .value_name("CHARACTER")
                        .help("Specify the corner style. Choose either \"■\" or \"0\". Only used when borders are enabled.")
                        .takes_value(true))
                    .get_matches();
    if matches.is_present("credits") {
        println!();
        println!("Main Developer:   valley  (Reddit: /u/Valley6660) (Github: Phate6660)");
        println!("Contributor:      kiedtl  (Reddit: /u/kiedtl)     (Github: kiedtl)");
        println!("Contributor:      lnicola                         (Github: lnicola)\n");
        println!("With thanks to:   \"/r/rust\", \"/u/tablair\", \"/u/kabocha_\", \"/u/DebuggingPanda\" for all the help they gave; and the tool \"neofetch\" for giving me the inspiration to make this.");
        println!();
        return;
    }
    let current_user = if !matches.is_present("no-user") || !matches.is_present("no-shell") {
        Some(env::var("USER")) //Passwd::current_user()
    } else {
        None
    };
    let bold = !matches.is_present("no-bold");
    let caps = !matches.is_present("no-caps");
    let borders = !matches.is_present("no-borders");
    // For the options that require bools or other input.
    let corners = matches.value_of("corners").unwrap_or("■");
    let music = matches.value_of("music").unwrap_or("no");
    let logofile = matches.value_of("logofile").unwrap_or("");
    let packages = matches.value_of("packages");
    let format;
    // Determine if borders are used, and if they are, the style of the corners.
    if matches.is_present("minimal") {
        format = format::FormatBuilder::new()
            .column_separator(' ')
            .borders(' ')
            .separators(
                &[format::LinePosition::Top, format::LinePosition::Bottom],
                format::LineSeparator::new(' ', ' ', ' ', ' '),
            )
            .padding(0, 0)
            .build();
        table.set_format(format);
    } else if borders {
        if corners == "■" {
            format = format::FormatBuilder::new()
                .column_separator(' ')
                .borders('│')
                .separators(
                    &[format::LinePosition::Top, format::LinePosition::Bottom],
                    format::LineSeparator::new('─', '─', '■', '■'),
                )
                .padding(1, 1)
                .build();
            table.set_format(format);
        } else if corners == "0" {
            format = format::FormatBuilder::new()
                .column_separator(' ')
                .borders('│')
                .separators(
                    &[format::LinePosition::Top, format::LinePosition::Bottom],
                    format::LineSeparator::new('─', '─', '0', '0'),
                )
                .padding(1, 1)
                .build();
            table.set_format(format);
        }
    }
    // Begin output. Data for variables will *only* be collected if the option for that specific output is turned on. Therefore making the program much more efficient.
    println!(); // Print blank line before output.
                // Determine the logo to use.
    if !matches.is_present("minimal") {
        if !matches.is_present("no-logo") {
            if !logofile.is_empty() {
                if let Err(e) = print_logo(logofile) {
                    error!("{}", e);
                }
            } else {
                print_default_logo()
            }
            println!(); // print a newline
        }
    }
    if !matches.is_present("no-user") {
        if matches.is_present("minimal") {
            if let Some(ref user) = current_user {
                println!("{}", user.as_ref().unwrap());
            }
        } else {
            if let Some(ref user) = current_user {
                add_row(&mut table, bold, caps, borders, "USER", &user.as_ref().unwrap());
            }
        }
    }
    if !matches.is_present("no-host") {
        let device = DeviceInfo::new();
        match device.get() {
            Ok(()) => if !matches.is_present("minimal") {
                println!("{}", device.format());
            } else {
                add_row(&mut table, bold, caps, borders, "HOST", &device.format());
            },
            Err(e) => error!("{}", e),
        }
    }
    if !matches.is_present("no-uptime") {
        let mut uptime = UptimeInfo::new();
        match uptime.get() {
            Ok(()) => if matches.is_present("minimal") {
                println!("{}", uptime.format());
            } else {
                add_row(&mut table, bold, caps, borders, "UPTIME", &uptime.format());
            },
            Err(e) => error!("{}", e),
        }
    }
    if !matches.is_present("no-distro") {
        if matches.is_present("minimal") {
            if let Ok(Some(dist)) = get_os_release() {
                println!("{}", &dist);
            }
        } else {
            if let Ok(Some(dist)) = get_os_release() {
                add_row(&mut table, bold, caps, borders, "DISTRO", &dist);
            }
        }
    }
    if !matches.is_present("no-kernel") {
        if matches.is_present("minimal") {
            match get_kernel_version() {
                Ok(kern) => println!("{}", &kern),
                Err(e) => error!("{}", e),
            }
        } else {
            match get_kernel_version() {
                Ok(kern) => add_row(&mut table, bold, caps, borders, "KERNEL", &kern),
                Err(e) => error!("{}", e),
            }
        }
    }
    if !matches.is_present("no-wm-de") {
        if matches.is_present("minimal") {
            match get_window_manager() {
                Ok(wm) => println!("{}", &wm),
                Err(e) => error!("{}", e),
            }
        } else {
            match get_window_manager() {
                Ok(wm) => add_row(&mut table, bold, caps, borders, "WM/DE", &wm),
                Err(e) => error!("{}", e),
            }
        }
    }
    if matches.is_present("editor") {
        if matches.is_present("minimal") {
            match get_editor() {
                Ok(ed) => println!("{}", &ed),
                Err(e) => error!("{}", e),
            }
        } else {
            match get_editor() {
                Ok(ed) => add_row(&mut table, bold, caps, borders, "EDITOR", &ed),
                Err(e) => error!("{}", e),
            }
        }
    }
    if !matches.is_present("no-shell") {
        if matches.is_present("minimal") {
                if let Some(shell) = Path::new(&env::var("SHELL").unwrap()).file_name() {
                    println!("{}", shell.to_string_lossy().as_ref());
                }
        } else {
                if let Some(shell) = Path::new(&env::var("SHELL").unwrap()).file_name() {
                    add_row(
                        &mut table,
                        bold,
                        caps,
                        borders,
                        "SHELL",
                        shell.to_string_lossy().as_ref(),
                    );
                }
        }
    }
    if matches.is_present("cpu") {
        let mut cpu = CPUInfo::new();
        match cpu.get() {
            Ok(()) => if matches.is_present("minimal") {
                println!("{}", cpu.format());
            } else {
                add_row(&mut table, bold, caps, borders, "CPU", &cpu.format());
            },
            Err(e) => error!("{}", e),
        }
    }
    if matches.is_present("ip_address") {
        if matches.is_present("minimal") {
            match get_ip_address() {
                Ok(ip) => println!("{}", &ip),
                Err(e) => error!("{}", e),
            }
        } else {
            match get_ip_address() {
                Ok(ip) => add_row(&mut table, bold, caps, borders, "IP ADDRESS", &ip),
                Err(e) => error!("{}", e),
            }
        }
    }
    if let Some(packages) = packages {
        match get_packages(packages) {
            Ok(pkg) => {
                if matches.is_present("minimal") {
                    println!("{}", &pkg);
                } else {
                    add_row(
                        &mut table,
                        bold,
                        caps,
                        borders,
                        &format!("PACKAGES ({})", packages.to_ascii_uppercase()),
                        &pkg,
                    );
                }
            }
            Err(e) => error!("{}", e),
        }
    }
    if music == "mpd" {
        if matches.is_present("minimal") {
            match get_mpd_song() {
                Ok(mus) => println!("{}", &mus),
                Err(e) => error!("{}", e),
            }
        } else {
            match get_mpd_song() {
                Ok(mus) => add_row(&mut table, bold, caps, borders, "MUSIC (MPD)", &mus),
                Err(e) => error!("{}", e),
            }
        }
    }
    if !matches.is_present("minimal") {
        table.printstd(); // After collecting data for variables and adding the rows, print the final output into a custom table.
    }
    println!(); // Print blank line after output.
}
