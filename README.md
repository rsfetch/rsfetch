<h3 align="center"><img src="https://raw.githubusercontent.com/rsfetch/rsfetch/master/Screenshots/new-logo.jpg" alt="logo" height="100px"></h3>
<p align="center">Fast (<1ms execution time) and somewhat(?) minimal fetch program written in Rust.</p>

<p align="center">
<img alt="crates.io version" src="https://img.shields.io/badge/crates.io-v1.9.0-blue.svg">
<img alt="GitHub code size in bytes" src="https://img.shields.io/github/languages/code-size/rsfetch/rsfetch.svg">
<img alt="GitHub" src="https://img.shields.io/github/license/rsfetch/rsfetch.svg">
<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/rsfetch/rsfetch.svg">
<img alt="GitHub issues" src="https://img.shields.io/github/issues/rsfetch/rsfetch.svg?color=gren">
</p>

### Table of Contents
* [Preface](#preface)
* [Things to Know](#things-to-know)
* [Installation](#installation)
* [Usage](#usage)
* [Screenshots](#screenshots)
* [Benchmarks](#benchmarks)
* [License](#license)
* [Changelog](#changelog)

### Preface

**Why I originally made it:** I used to use neofetch, ufetch, and 
aurafetch a lot, but then that got me thinking. I have the skill to make
 my own script, so why not? At first, it was just a simple BASH script. 
Then I decided I wanted to try my hand at Rust, as it has interested me 
for a while. So... here's the end result. I am open to any criticisms 
you have. After all, I wish to make this better (as well as improve my 
Rust skills).

**Why I want to continue improving this:** When this first started out, it 
was a simple pet project for learning how to code in Rust. Now that I have
people helping me with this project, I would love to see this turn into a
worthy alternative to neofetch/ufetch/screenfetch. Please, if there's anything 
that you feel is missing from this, open an issue. I would love to hear people's
thoughts on how this is.

**Another notice:** If you show off `rsfetch` anywhere, please let [/u/Valley6660](https://www.reddit.com/user/Valley6660/) know or open up an issue here.
He'd absolutely love to see `rsfetch` used by someone other than himself.
And if you share, there's a very good possibility of your screenshot being included in the README!
(I'm totally not trying to bribe people into using it, shhhhhhh.)

### Things to Know

1. If you plan to help, note that indentation is done with 4 spaces. It used to be tabs, but I have been told that spaces are the preferred indentation in the Rust community. ~~Any new changes should be recorded in [CHANGELOG.md](CHANGELOG.md).~~

2. These are the current package managers supported. If I missed any, please report it in [this](https://github.com/rsfetch/rsfetch/issues/28) issue.
    - pacman
    - apt
    - xbps
    - dnf
    - eopkg
    - pkg
    - rpm
    - apk
    - portage (note: requires portage-utils to be installed as it uses qlist)
    - cargo
    - pip

3. The music info obtains the info from a locally running `mpd` server using `mpc`. If you want more music sources supported, let me know by opening up an issue.

### Installation
I have prebuilt binaries in the releases tab for people who don't want to build from source, otherwise you can do this.

#### Prerequisites
- the Rust compiler toolchain
- **GNU** Make (makefile not compatible with `bmake`!)

You have two methods to build from source:

##### make
(Choose either 3 or 4, but don't do both.)
1. Clone the repository
2. `cd rsfetch`
3. `sudo make install`
4. Or use `make DESTDIR="/home/$USER" PREFIX=".cargo/" install` to install to `$HOME/.cargo/bin`

Uninstall with `sudo make uninstall`.

##### cargo
1. Clone the repository
2. `cd rsfetch`
3. `cargo install --path .`

Uninstall with `cargo uninstall rsfetch`

#### AUR package
Optionally, if you use a Arch-based distro, you can use either of the following AUR packages:
- [rsfetch-git](https://aur.archlinux.org/packages/rsfetch-git/) (This requires Rust as it builds from source.)
- [rsfetch-bin](https://aur.archlinux.org/packages/rsfetch-bin/) (This grabs the latest binary from releases.)

#### ebuild
~~I just switched over to Gentoo a few days ago, so whenever I figure out how to make (and share) ebuilds, there will be one released. :)~~ So it turns out that not only am I terrible at pkgbuilds, but also ebuilds. If anyone could make one for me, that would be amazing.

#### Cargo
If you already have `rust` and `cargo` setup, you can install it with:<br>
`cargo install --git https://github.com/rsfetch/rsfetch.git`


### Usage

```
rsfetch 2.0.0

An fetch tool for Linux. Fast (~1ms execution time) and somewhat(?) minimal.

All options are off by default. 

Accepted values for the package manager are "pacman", "apt", "xbps", "dnf", "pkg", "eopkg", "rpm", "apk", "pip",
"portage", and "cargo".

USAGE:
    rsfetch [FLAGS] [OPTIONS]

FLAGS:
    -P, --cpu           Turn CPU information on.
        --credits       List of past and current contributors for this project.
    -d, --distro        Turn distro name on.
    -e, --editor        Turn default editor name on. (Must have $EDITOR/$VISUAL variable set.)
        --help          Prints help information
    -h, --host          Turn device name on.
    -H, --hostname      Turn hostname on.
    -i, --ip-address    Turn ip address display on.
    -k, --kernel        Turn kernel version on.
    -l, --logo          Turn the logo or ascii art on.
    -r, --memory        Turn memory information on.
    -M, --minimal       Turn minimal-style output mode on.
    -N, --neofetch      Turn neofetch-style output mode on.
    -b, --no-bold       Turn bold for field titles off.
    -B, --no-borders    Turn borders off.
    -c, --no-caps       Turn all caps off.
    -s, --shell         Turn default shell name on.
    -t, --terminal      Turn terminal name on.
    -u, --uptime        Turn uptime info on.
    -U, --user          Turn user name on.
    -@, --userat        Turn 'user@hostname' style on (only applicable if both 'user' and 'hostname' are enabled!).
    -V, --version       Prints version information
    -w, --wm            Turn WM or DE name on.

OPTIONS:
    -C, --corners <CHARACTER>    Specify the corner character. Only used when borders are enabled.
    -L, --logofile <FILE>        Specify the file from which to read a custom ASCII logo.
    -m, --music <SOURCE>         Choose where to get music info. The only supported options is "mpd".
    -p, --packages <PKG MNGR>    Turn total package count on.
```

### Screenshots

**rsfetch-style output**<br>
![rsfetch-style](Screenshots/rsfetch.png?raw=true "rsfetch-style")

**neofetch-style output**<br>
![neofetch-style](Screenshots/neofetch.png?raw=true "neofetch-style")

### Benchmarks

Here's a detailed benchmark; with rsfetch versus ufetch, pfetch, aurafetch, Neofetch, and ScreenFetch:

```
Benchmark #1: target/release/rsfetch -NcldkuUH@swp apt
  Time (mean ± σ):      20.5 ms ±  20.2 ms    [User: 10.8 ms, System: 5.3 ms]
  Range (min … max):    14.4 ms … 117.2 ms    25 runs
 
  Warning: The first benchmarking run for this command was significantly slower than the rest (117.2 ms). This could be caused by (filesystem) caches that were not filled until after the first run. You should consider using the '--warmup' option to fill those caches before the actual benchmark. Alternatively, use the '--prepare' option to clear the caches before each timing run.
 
Benchmark #2: ufetch
  Time (mean ± σ):     180.4 ms ±  10.4 ms    [User: 154.4 ms, System: 24.9 ms]
  Range (min … max):   171.4 ms … 211.9 ms    14 runs
 
Benchmark #3: pfetch
  Time (mean ± σ):     146.2 ms ±   3.7 ms    [User: 123.5 ms, System: 19.1 ms]
  Range (min … max):   141.4 ms … 155.9 ms    19 runs
 
Benchmark #4: aura -n "term"
  Time (mean ± σ):     138.4 ms ±   3.5 ms    [User: 109.6 ms, System: 26.0 ms]
  Range (min … max):   132.2 ms … 145.8 ms    21 runs
 
Benchmark #5: neofetch --disable resolution --disable theme --disable icons --disable term --disable cpu --disable memory
  Time (mean ± σ):     454.2 ms ±  23.6 ms    [User: 327.3 ms, System: 101.6 ms]
  Range (min … max):   427.6 ms … 507.6 ms    10 runs
 
Benchmark #6: screenfetch -d "-gtk;-res;-disk;-mem;-cpu"
  Time (mean ± σ):     647.4 ms ±  33.1 ms    [User: 480.3 ms, System: 133.6 ms]
  Range (min … max):   622.5 ms … 731.6 ms    10 runs
 
Summary
  'target/release/rsfetch -NcldkuUH@swp apt' ran
    6.76 ± 6.67 times faster than 'aura -n "term"'
    7.14 ± 7.05 times faster than 'pfetch'
    8.81 ± 8.71 times faster than 'ufetch'
   22.19 ± 21.93 times faster than 'neofetch --disable resolution --disable theme --disable icons --disable term --disable cpu --disable memory'
   31.63 ± 31.25 times faster than 'screenfetch -d "-gtk;-res;-disk;-mem;-cpu"'
```

### License

- License has been set to "MIT". Anybody can do anything with this repository, provided that they include the license if they choose to redistribute. [View license](https://raw.githubusercontent.com/rsfetch/rsfetch/master/LICENSE)

### Changelog

- Changelog was generated (because I'm too lazy) by [github_changelog_generator](https://github.com/skywinder/Github-Changelog-Generator). [View changelog](https://raw.githubusercontent.com/rsfetch/rsfetch/master/CHANGELOG.md)
