use clap::{App, Arg};
use log::error;
use prettytable::{cell, format, row, Table};
use pwd::Passwd;
use snafu::{OptionExt, ResultExt, Snafu};
use std::env;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::process::Command;
use std::result;
use std::time::Duration;

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
    #[snafu(display("Could not run curl"))]
    Curl { source: io::Error },
    #[snafu(display("Could not run pacman"))]
    Pacman { source: io::Error },
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
fn add_row(
    mut table: Table,
    abold: &str,
    caps: &str,
    border: &str,
    title: &str,
    value: &str,
) -> Table {
    let mut title_str = title.to_string();
    if caps != "true" {
        title_str = title_str.to_lowercase();
    }
    if abold != "false" {
        title_str = make_bold(&title_str);
    }
    if border != "true" {
        table.add_row(row![title_str, value]);
    } else {
        table.add_row(row![title_str, "=", value]);
    }
    return table;
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

fn get_device_name() -> Result<String> {
    let contents =
        fs::read_to_string("/sys/devices/virtual/dmi/id/product_name").context(DeviceName)?;
    let dev = contents.trim_end().to_string();
    Ok(dev)
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

fn get_window_manager() -> Result<String> {
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
    let curl = Command::new("curl")
        .arg("--silent")
        .arg("https://ipecho.net/plain")
        .output()
        .context(Curl)?;
    let ip = String::from_utf8_lossy(&curl.stdout).into_owned();
    Ok(ip)
}

fn get_package_count() -> Result<String> {
    let pacman = Command::new("pacman").arg("-Qq").output().context(Pacman)?;
    let pkgs = bytecount::count(&pacman.stdout, b'\n');
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

fn format_duration(duration: Duration) -> Result<String> {
    let mut duration = duration.as_secs();
    if duration < 60 {
        let s = if duration == 1 {
            String::from("1 second")
        } else {
            format!("{} seconds", duration)
        };
        return Ok(s);
    }

    duration /= 60;
    let minutes = duration % 60;
    duration /= 60;
    let hours = duration % 24;
    duration /= 24;
    let days = duration % 7;
    let weeks = (duration / 7) % 52;
    duration /= 365;
    let years = duration % 10;
    let decades = duration / 10;

    let mut s = String::new();
    let mut comma = false;
    fn add_part(comma: &mut bool, mut s: &mut String, name: &str, value: u64) -> Result<()> {
        if value > 0 {
            if *comma {
                s.push_str(", ");
            }
            itoa::fmt(&mut s, value).context(FormatUptime)?;
            s.push(' ');
            s.push_str(name);
            if value > 1 {
                s.push('s');
            }
            *comma = true;
        }
        Ok(())
    }
    add_part(&mut comma, &mut s, "decade", decades)?;
    add_part(&mut comma, &mut s, "year", years)?;
    add_part(&mut comma, &mut s, "week", weeks)?;
    add_part(&mut comma, &mut s, "day", days)?;
    add_part(&mut comma, &mut s, "hour", hours)?;
    add_part(&mut comma, &mut s, "minute", minutes)?;
    Ok(s)
}

// Main function
fn main() {
    pretty_env_logger::init();

    // Variables
    let mut table = Table::new();
    let matches = App::new("rsfetch")
                    .version("1.5.0")
                    .about("\nMy info fetch tool for Linux. Fast (0.01s - 0.2s execution time) and somewhat(?) minimal.\nAll \"BOOL\" options default to \"true\" (with the exception of editor, window manager, and ip address), and \"SOURCE\" defaults to no.")
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
        println!();
        println!("Main Developer:   valley  (Reddit: /u/Valley6660) (Github: Phate6660)");
        println!("Contributor:      kiedtl  (Reddit: /u/kiedtl)     (Github: kiedtl)");
        println!("Contributor:      lnicola                         (Github: lnicola)\n");
        println!("With thanks to:   \"/r/rust\", \"/u/tablair\", \"/u/kabocha_\", \"/u/DebuggingPanda\" for all the help they gave; and the tool \"neofetch\" for giving me the inspiration to make this.");
        println!();
        return;
    }
    let caps = matches.value_of("caps").unwrap_or("true");
    let abold = matches.value_of("bold").unwrap_or("true");
    let corners = matches.value_of("corners").unwrap_or("■");
    let borders = matches.value_of("borders").unwrap_or("true");
    let user = matches.value_of("user").unwrap_or("true");
    let host = matches.value_of("host").unwrap_or("true");
    let ip_address = matches.value_of("ip_address").unwrap_or("false");
    let editor = matches.value_of("editor").unwrap_or("false");
    let shell = matches.value_of("shell").unwrap_or("true");
    let window_manager = matches.value_of("window_manager").unwrap_or("false");
    let distro = matches.value_of("distro").unwrap_or("true");
    let kernel = matches.value_of("kernel").unwrap_or("true");
    let uptime = matches.value_of("uptime").unwrap_or("true");
    let packages = matches.value_of("packages").unwrap_or("true");
    let music = matches.value_of("music").unwrap_or("no");
    let logo = matches.value_of("logo").unwrap_or("true");
    let logofile = matches.value_of("logofile").unwrap_or("");

    // Determine the logo to use.
    println!(); // For a blank line before output.
    if logo == "true" {
        if !logofile.is_empty() {
            if let Err(e) = print_logo(logofile) {
                error!("{}", e);
            }
        } else {
            print_default_logo()
        }
        println!(); // print a newline
    }
    let format;

    // Determine if borders are used, and if they are, the style of the corners.
    if borders == "true" {
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
    } else {
        format = format::FormatBuilder::new()
            .column_separator(' ')
            .borders(' ')
            .separators(
                &[format::LinePosition::Top, format::LinePosition::Bottom],
                format::LineSeparator::new(' ', ' ', ' ', ' '),
            )
            .padding(1, 1)
            .build();
        table.set_format(format);
    }
    // Begin output. Data for variables will *only* be collected if the option for that specific output is turned on. Therefore making the program much more efficient.
    let current_user = if user == "true" || shell == "true" {
        Passwd::current_user()
    } else {
        None
    };
    if user == "true" {
        if let Some(ref user) = current_user {
            table = add_row(table, abold, caps, borders, "USER", &user.name);
        }
    }
    if host == "true" {
        match get_device_name() {
            Ok(dev) => table = add_row(table, abold, caps, borders, "HOST", &dev),
            Err(e) => error!("{}", e),
        }
    }
    if uptime == "true" {
        if let Some(uptime) = uptime_lib::get()
            .ok()
            .and_then(|uptime| uptime.to_std().ok())
        {
            match format_duration(uptime) {
                Ok(uptime) => table = add_row(table, abold, caps, borders, "UPTIME", &uptime),
                Err(e) => error!("{}", e),
            }
        };
    }
    if distro == "true" {
        if let Ok(Some(dist)) = get_os_release() {
            table = add_row(table, abold, caps, borders, "DISTRO", &dist);
        }
    }
    if kernel == "true" {
        match get_kernel_version() {
            Ok(kern) => table = add_row(table, abold, caps, borders, "KERNEL", &kern),
            Err(e) => error!("{}", e),
        }
    }
    if window_manager == "true" {
        match get_window_manager() {
            Ok(wm) => table = add_row(table, abold, caps, borders, "WINDOW MANAGER", &wm),
            Err(e) => error!("{}", e),
        }
    }
    if editor == "true" {
        match get_editor() {
            Ok(ed) => table = add_row(table, abold, caps, borders, "EDITOR", &ed),
            Err(e) => error!("{}", e),
        }
    }
    if shell == "true" {
        if let Some(ref user) = current_user {
            if let Some(shell) = Path::new(&user.shell).file_name() {
                table = add_row(
                    table,
                    abold,
                    caps,
                    borders,
                    "SHELL",
                    shell.to_string_lossy().as_ref(),
                );
            } else {
                table = add_row(table, abold, caps, borders, "SHELL", &user.shell);
            }
        }
    }
    if ip_address == "true" {
        match get_ip_address() {
            Ok(ip) => table = add_row(table, abold, caps, borders, "IP ADDRESS", &ip),
            Err(e) => error!("{}", e),
        }
    }
    if packages == "true" {
        match get_package_count() {
            Ok(pkg) => table = add_row(table, abold, caps, borders, "PACKAGES", &pkg),
            Err(e) => error!("{}", e),
        }
    }
    if music == "mpd" {
        match get_mpd_song() {
            Ok(mus) => table = add_row(table, abold, caps, borders, "MUSIC (MPD)", &mus),
            Err(e) => error!("{}", e),
        }
    }
    // After collecting data for variables and adding the rows, print the final output into a custom table.
    table.printstd();
    println!(); // For a blank line after output.
}
