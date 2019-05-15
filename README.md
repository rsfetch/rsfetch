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

Here's a detailed benchmark, with rsfetch versus Neofetch, ScreenFetch, ufetch, kfetch, aurafetch, and pfetch:
```
+kiedtl ~ (master +?) % hyperfine 'pfetch' 'kfetch' 'ufetch' 'rsfetch' 'aura' 'neofetch' 'screenfetch'
Benchmark #1: pfetch
  Time (mean ± σ):       4.7 ms ±   0.5 ms    [User: 3.3 ms, System: 1.9 ms]
  Range (min … max):     4.2 ms …   8.9 ms    426 runs

Benchmark #2: kfetch
  Time (mean ± σ):      14.7 ms ±   1.3 ms    [User: 10.3 ms, System: 5.2 ms]
  Range (min … max):    13.4 ms …  26.6 ms    170 runs

Benchmark #3: ufetch
  Time (mean ± σ):      22.9 ms ±   1.1 ms    [User: 19.9 ms, System: 6.3 ms]
  Range (min … max):    21.3 ms …  27.7 ms    100 runs

Benchmark #4: rsfetch
  Time (mean ± σ):       1.0 ms ±   0.3 ms    [User: 1.0 ms, System: 0.9 ms]
  Range (min … max):     0.8 ms …   2.8 ms    822 runs

Benchmark #5: aura
  Time (mean ± σ):      70.0 ms ±  43.8 ms    [User: 47.9 ms, System: 19.6 ms]
  Range (min … max):    54.7 ms … 215.5 ms    13 runs
  
Benchmark #6: neofetch
  Time (mean ± σ):     154.5 ms ±   4.2 ms    [User: 113.2 ms, System: 55.2 ms]
  Range (min … max):   149.3 ms … 168.7 ms    17 runs

Benchmark #7: screenfetch
  Time (mean ± σ):     318.3 ms ±  81.8 ms    [User: 211.9 ms, System: 119.5 ms]
  Range (min … max):   287.8 ms … 550.9 ms    10 runs

Summary
  'rsfetch' ran
    4.47 ± 1.33 times faster than 'pfetch'
   14.08 ± 4.06 times faster than 'kfetch'
   21.87 ± 6.11 times faster than 'ufetch'
   66.86 ± 45.68 times faster than 'aura'
  147.56 ± 40.79 times faster than 'neofetch'
  304.07 ± 114.47 times faster than 'screenfetch'
```
As you can see, rsfetch is the clear winner with pfetch and kfetch trailing behind.  

And yes, you saw right. Execution time was 1ms on average! Crazy fast.

### License

- License has been set to `Unlicense`. Anybody can do anything with this repository. [View license](https://raw.githubusercontent.com/rsfetch/rsfetch/master/LICENSE)
