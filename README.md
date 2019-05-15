<h3 align="center"><img src="https://raw.githubusercontent.com/rsfetch/rsfetch/master/Screenshots/logo.jpg" alt="logo" height="100px"></h3>
<p align="center">Fast (1ms execution time) and somewhat(?) minimal fetch program written in Rust.</p>

<p align="center">
<img alt="GitHub code size in bytes" src="https://img.shields.io/github/languages/code-size/rsfetch/rsfetch.svg">
<img alt="GitHub" src="https://img.shields.io/github/license/rsfetch/rsfetch.svg">
<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/rsfetch/rsfetch.svg">
<img alt="GitHub issues" src="https://img.shields.io/github/issues/rsfetch/rsfetch.svg?color=gren">
</p>

### Table of Contents
* [Preface](#preface)
* [Things to Know](#things-to-know)
* [Requirements](#requirements)
* [Installation](#installation)
* [Screenshots](#screenshots)
* [Benchmarks](#benchmarks)
* [License](#license)

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

### Things to Know

1. ~~The packages section only works for Arch (or Arch-based) distros as it uses pacman.~~ Supports Arch-based (pacman), Debian/Ubuntu-based (apt), Void (xbps), Fedora (dnf), and Python (pip).

2. If you plan to help, note that indentation is done with 4 spaces. It used to be tabs, but I have been told that spaces are the preferred indentation in the Rust community. Any new changes should be recorded in [CHANGELOG.md](CHANGELOG.md).

### Requirements
- `mpd + mpc` for the music info. (Completely optional, as music info is turned off by default.)

### Installation
I have prebuilt binaries in the releases tab for people who don't want to build from source, otherwise you can do this.

1. Install rust and cargo.
2. Clone the repository.
3. `cd rsfetch; make; sudo make install`

Uninstall with `sudo make uninstall`.

#### AUR package
Optionally, if you use a Arch-based distro, you can use either of the following AUR packages:
- [rsfetch-git](https://aur.archlinux.org/packages/rsfetch-git/) (This requires Rust as it builds from source.)
- [rsfetch-bin](https://aur.archlinux.org/packages/rsfetch-bin/) (This grabs the latest binary from releases.)

### Screenshots

**Help**
![Help](Screenshots/help.png?raw=true "Help")

**Default Fetch**
![Default](Screenshots/default.png?raw=true "Default")

**My Preference of Options**
![Default](Screenshots/preference.png?raw=true "Preference")

### Benchmarks

Here's a detailed benchmark, with rsfetch versus Neofetch, ScreenFetch, and ufetch:

$ `hyperfine "./target/release/rsfetch" "neofetch" "screenfetch" "ufetch"`

```
Benchmark #1: ./target/release/rsfetch
  Time (mean ± σ):       1.7 ms ±   0.5 ms    [User: 1.2 ms, System: 1.3 ms]
  Range (min … max):     1.3 ms …   4.6 ms    728 runs
 
Benchmark #2: neofetch
  Time (mean ± σ):     200.7 ms ±   7.0 ms    [User: 143.1 ms, System: 63.5 ms]
  Range (min … max):   191.7 ms … 219.8 ms    13 runs
 
Benchmark #3: screenfetch
  Time (mean ± σ):     416.4 ms ±  12.6 ms    [User: 254.6 ms, System: 186.3 ms]
  Range (min … max):   402.9 ms … 446.0 ms    10 runs
 
Benchmark #4: ufetch
  Time (mean ± σ):      32.1 ms ±   2.6 ms    [User: 25.9 ms, System: 9.5 ms]
  Range (min … max):    24.4 ms …  38.5 ms    79 runs
 
Summary
  './target/release/rsfetch' ran
   19.23 ± 5.77 times faster than 'ufetch'
  120.23 ± 34.97 times faster than 'neofetch'
  249.48 ± 72.45 times faster than 'screenfetch'
```
As you can see, `rsfetch` is the clear winner with `ufetch` trailing slightly behind.  

And yes, you saw right. Execution time was 1ms on average! Crazy fast.

### License

- License has been set to "The Unlicense". Anybody can do anything with this repository. [View license](https://raw.githubusercontent.com/rsfetch/rsfetch/master/LICENSE)
