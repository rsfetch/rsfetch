<h3 align="center"><img src="https://raw.githubusercontent.com/rsfetch/rsfetch/master/Screenshots/logo.jpg" alt="logo" height="100px"></h3>
<p align="center">Fast (<1ms execution time) and somewhat(?) minimal fetch program written in Rust.</p>

<p align="center">
<img alt="crates.io version" src="https://img.shields.io/badge/crates.io-v1.9.0-blue.svg">
<img alt="GitHub code size in bytes" src="https://img.shields.io/github/languages/code-size/rsfetch/rsfetch.svg">
<img alt="GitHub" src="https://img.shields.io/github/license/rsfetch/rsfetch.svg">
<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/rsfetch/rsfetch.svg">
<img alt="GitHub issues" src="https://img.shields.io/github/issues/rsfetch/rsfetch.svg?color=gren">
</p>

### Table of Contents
* [Installation](#installation)
* [Screenshots](#screenshots)
* [Benchmarks](#benchmarks)
* [License](#license)
* [Contributing](#contributing)
* [Changelog](#changelog)

### Installation
#### Binaries
Binaries are available in the [releases](https://github.com/rsfetch/rsfetch/releases) for those who do not 
wish to build from source.

#### Building from source
**Prerequisites**
1. the Rust compiler toolchain
2. GNU Make (`bmake` untested)

Retrieve the repository source:
```
$ wget https://github.com/rsfetch/rsfetch/archive/master.tar.gz
$ tar xvf master.tar.gz && cd rsfetch
```
Build:
```
$ make
```
Install:
```
# make install
```

You may uninstall with `sudo make uninstall`.

#### AUR package
Optionally, if you use a Arch-based distro, you can use either of the following AUR packages:
- [rsfetch-git](https://aur.archlinux.org/packages/rsfetch-git/) (This requires Rust as it builds from source.)
- [rsfetch-bin](https://aur.archlinux.org/packages/rsfetch-bin/) (This grabs the latest binary from releases.)

#### Cargo
If you already have `rust` and `cargo` setup, you can install it with `cargo install rsfetch`.

### Screenshots

**Default Fetch**
![Default](Screenshots/default.png?raw=true "Default")

**Default (Minimal Mode) Fetch**
![Default](Screenshots/default-minimal.png?raw=true "Default")

**My Preference of Options**
![Default](Screenshots/preference.png?raw=true "Preference")

### Benchmarks

Here's a detailed benchmark, with rsfetch versus Neofetch and ScreenFetch:

Note: All programs are using default options, no flags or config files were used.

$ `hyperfine "\rsfetch" "pfetch" "neofetch --config none" "screenfetch"`

```
Benchmark #1: \rsfetch
  Time (mean ± σ):       1.4 ms ±   0.3 ms    [User: 0.6 ms, System: 1.0 ms]
  Range (min … max):     1.2 ms …   3.2 ms    1084 runs
 
Benchmark #2: pfetch
  Time (mean ± σ):      41.4 ms ±   2.8 ms    [User: 27.2 ms, System: 18.6 ms]
  Range (min … max):    36.3 ms …  47.4 ms    75 runs
 
Benchmark #3: neofetch --config none
  Time (mean ± σ):     228.1 ms ±   4.6 ms    [User: 124.7 ms, System: 114.9 ms]
  Range (min … max):   220.7 ms … 238.7 ms    12 runs
 
Benchmark #4: screenfetch
  Time (mean ± σ):     392.0 ms ±   8.4 ms    [User: 170.3 ms, System: 250.6 ms]
  Range (min … max):   382.3 ms … 406.9 ms    10 runs
 
Summary
  '\rsfetch' ran
   28.58 ± 5.69 times faster than 'pfetch'
  157.65 ± 29.70 times faster than 'neofetch --config none'
  270.96 ± 51.09 times faster than 'screenfetch'
```

As you can see, `rsfetch` is the clear winner.  

And yes, you saw right. Execution time was <1ms on average! Crazy fast.

### License

- License has been set to "MIT". Anybody can do anything with this repository, provided that they include the license if they choose to redistribute. [View license](https://raw.githubusercontent.com/rsfetch/rsfetch/master/LICENSE)


### Contributing

1. If you plan to help, note that indentation is done with 4 spaces. It used to be tabs, but I have been told that spaces are the preferred indentation in the Rust community. ~~Any new changes should be recorded in [CHANGELOG.md](CHANGELOG.md).~~

### Changelog

- Changelog was generated (because I'm too lazy) by [github_changelog_generator](https://github.com/skywinder/Github-Changelog-Generator). [View changelog](https://raw.githubusercontent.com/rsfetch/rsfetch/master/CHANGELOG.md)
