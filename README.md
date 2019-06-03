<h3 align="center"><img src="https://raw.githubusercontent.com/rsfetch/rsfetch/master/Screenshots/logo.jpg" alt="logo" height="100px"></h3>
<p align="center">Fast (<1ms execution time) and somewhat(?) minimal fetch program written in Rust.</p>

<p align="center">
<img alt="GitHub code size in bytes" src="https://img.shields.io/github/languages/code-size/rsfetch/rsfetch.svg">
<img alt="GitHub" src="https://img.shields.io/github/license/rsfetch/rsfetch.svg">
<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/rsfetch/rsfetch.svg">
<img alt="GitHub issues" src="https://img.shields.io/github/issues/rsfetch/rsfetch.svg?color=gren">
</p>

### Table of Contents
* [Preface](#preface)
* [Things to Know](#things-to-know)
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

1. If you plan to help, note that indentation is done with 4 spaces. It used to be tabs, but I have been told that spaces are the preferred indentation in the Rust community. Any new changes should be recorded in [CHANGELOG.md](CHANGELOG.md).

2. These are the current package managers supported. I missed any, please report it in [this](https://github.com/rsfetch/rsfetch/issues/28) issue.
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

1. Install rust and cargo.
2. Clone the repository.
3. `cd rsfetch; make; sudo make install`

Uninstall with `sudo make uninstall`.

#### AUR package
Optionally, if you use a Arch-based distro, you can use either of the following AUR packages:
- [rsfetch-git](https://aur.archlinux.org/packages/rsfetch-git/) (This requires Rust as it builds from source.)
- [rsfetch-bin](https://aur.archlinux.org/packages/rsfetch-bin/) (This grabs the latest binary from releases.)

#### ebuild
I just switched over to Gentoo a few days ago, so whenever I figure out how to make (and share) ebuilds, there will be one released. :)

### Screenshots

**Help**
![Help](Screenshots/help.png?raw=true "Help")

**Default Fetch**
![Default](Screenshots/default.png?raw=true "Default")

**Default (Minimal Mode) Fetch**
![Default](Screenshots/default-minimal.png?raw=true "Default")

**My Preference of Options**
![Default](Screenshots/preference.png?raw=true "Preference")

### Benchmarks

Here's a detailed benchmark, with rsfetch versus Neofetch and ScreenFetch:

Note: All programs are using default options, no flags or config files were used.

$ `hyperfine "rsfetch" "neofetch" "screenfetch"`

```
Benchmark #1: rsfetch
  Time (mean ± σ):       1.3 ms ±   0.0 ms    [User: 0.6 ms, System: 0.9 ms]
  Range (min … max):     1.2 ms …   1.6 ms    1314 runs
 
Benchmark #2: neofetch
  Time (mean ± σ):     125.1 ms ±   2.2 ms    [User: 84.7 ms, System: 45.8 ms]
  Range (min … max):   123.4 ms … 134.4 ms    22 runs
 
Benchmark #3: screenfetch
  Time (mean ± σ):     141.4 ms ±   1.3 ms    [User: 87.3 ms, System: 65.0 ms]
  Range (min … max):   139.2 ms … 145.2 ms    21 runs
 
Summary
  'rsfetch' ran
   96.09 ± 3.77 times faster than 'neofetch'
  108.56 ± 3.93 times faster than 'screenfetch'
```
As you can see, `rsfetch` is the clear winner.  

And yes, you saw right. Execution time was <1ms on average! Crazy fast.

### License

- License has been set to "MIT". Anybody can do anything with this repository, provided that they include the license if they choose to redistribute. [View license](https://raw.githubusercontent.com/rsfetch/rsfetch/master/LICENSE)
