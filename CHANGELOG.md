# Changelog

## [Unreleased](https://github.com/rsfetch/rsfetch/tree/HEAD)

[Full Changelog](https://github.com/rsfetch/rsfetch/compare/2.0.0...HEAD)

**Implemented enhancements:**

- Detect terminal font. [\#75](https://github.com/rsfetch/rsfetch/issues/75)
- Manpage [\#58](https://github.com/rsfetch/rsfetch/issues/58)
- New logo [\#50](https://github.com/rsfetch/rsfetch/issues/50)

**Fixed bugs:**

- Fix Artix detection. [\#76](https://github.com/rsfetch/rsfetch/issues/76)

**Closed issues:**

- `sudo make install` fails with: [\#68](https://github.com/rsfetch/rsfetch/issues/68)
- Discussion: rsfetch 2.0 [\#51](https://github.com/rsfetch/rsfetch/issues/51)

**Merged pull requests:**

- fix artix detection by checking for /usr/lib/os-release [\#77](https://github.com/rsfetch/rsfetch/pull/77) ([Phate6660](https://github.com/Phate6660))
- Update LICENSE [\#72](https://github.com/rsfetch/rsfetch/pull/72) ([kiedtl](https://github.com/kiedtl))
- fix for music info [\#70](https://github.com/rsfetch/rsfetch/pull/70) ([Phate6660](https://github.com/Phate6660))
- replace/remove screenshots, update benchmarks [\#69](https://github.com/rsfetch/rsfetch/pull/69) ([Phate6660](https://github.com/Phate6660))

## [2.0.0](https://github.com/rsfetch/rsfetch/tree/2.0.0) (2020-01-05)

[Full Changelog](https://github.com/rsfetch/rsfetch/compare/1.9.0...2.0.0)

**Implemented enhancements:**

- Change the way package counts work? [\#49](https://github.com/rsfetch/rsfetch/issues/49)
- add basic manpage [\#67](https://github.com/rsfetch/rsfetch/pull/67) ([kiedtl](https://github.com/kiedtl))
- Add Terminal information field [\#65](https://github.com/rsfetch/rsfetch/pull/65) ([kiedtl](https://github.com/kiedtl))
- Misc improvements [\#64](https://github.com/rsfetch/rsfetch/pull/64) ([kiedtl](https://github.com/kiedtl))
- Add memory field [\#63](https://github.com/rsfetch/rsfetch/pull/63) ([kiedtl](https://github.com/kiedtl))
- Fix CPU and OS detection [\#60](https://github.com/rsfetch/rsfetch/pull/60) ([kiedtl](https://github.com/kiedtl))
- Neofetch-style output [\#59](https://github.com/rsfetch/rsfetch/pull/59) ([kiedtl](https://github.com/kiedtl))
- refactor [\#57](https://github.com/rsfetch/rsfetch/pull/57) ([kiedtl](https://github.com/kiedtl))
- remove superfluous dependencies [\#54](https://github.com/rsfetch/rsfetch/pull/54) ([kiedtl](https://github.com/kiedtl))

**Closed issues:**

- rsfetch capitalization [\#47](https://github.com/rsfetch/rsfetch/issues/47)
- Crop output from WM variable? [\#40](https://github.com/rsfetch/rsfetch/issues/40)
- Add more formatting/customization options. [\#30](https://github.com/rsfetch/rsfetch/issues/30)
- Change crate for handling CLI arguments. [\#12](https://github.com/rsfetch/rsfetch/issues/12)
- AUR package. [\#8](https://github.com/rsfetch/rsfetch/issues/8)

**Merged pull requests:**

- Change version to 2.0.0 [\#66](https://github.com/rsfetch/rsfetch/pull/66) ([Phate6660](https://github.com/Phate6660))
- update prerequisite [\#62](https://github.com/rsfetch/rsfetch/pull/62) ([kiedtl](https://github.com/kiedtl))
- use new logo [\#61](https://github.com/rsfetch/rsfetch/pull/61) ([kiedtl](https://github.com/kiedtl))
- Add CPU information field [\#55](https://github.com/rsfetch/rsfetch/pull/55) ([kiedtl](https://github.com/kiedtl))
- Update 2.md [\#52](https://github.com/rsfetch/rsfetch/pull/52) ([Phate6660](https://github.com/Phate6660))
- Add gentoo \(portage\) support. [\#48](https://github.com/rsfetch/rsfetch/pull/48) ([Phate6660](https://github.com/Phate6660))
- Avoid some duplication in get\_packages [\#46](https://github.com/rsfetch/rsfetch/pull/46) ([lnicola](https://github.com/lnicola))

## [1.9.0](https://github.com/rsfetch/rsfetch/tree/1.9.0) (2019-05-27)

[Full Changelog](https://github.com/rsfetch/rsfetch/compare/1.8.0...1.9.0)

**Implemented enhancements:**

- Reduce cognitive complexity of main function. [\#43](https://github.com/rsfetch/rsfetch/issues/43)
- Create a minimal flag. [\#42](https://github.com/rsfetch/rsfetch/issues/42)

**Merged pull requests:**

- Added a minimal flag, added support for Alpine Linux \(apk\). [\#45](https://github.com/rsfetch/rsfetch/pull/45) ([Phate6660](https://github.com/Phate6660))
- Added cargo \(Rust\) support, various tweaks suggested by Clippy. [\#44](https://github.com/rsfetch/rsfetch/pull/44) ([Phate6660](https://github.com/Phate6660))

## [1.8.0](https://github.com/rsfetch/rsfetch/tree/1.8.0) (2019-05-23)

[Full Changelog](https://github.com/rsfetch/rsfetch/compare/1.7.0...1.8.0)

**Merged pull requests:**

- Added support for BSD, Solus, and OpenSUSE. [\#39](https://github.com/rsfetch/rsfetch/pull/39) ([Phate6660](https://github.com/Phate6660))

## [1.7.0](https://github.com/rsfetch/rsfetch/tree/1.7.0) (2019-05-15)

[Full Changelog](https://github.com/rsfetch/rsfetch/compare/1.6.6...1.7.0)

**Closed issues:**

- Use a proper web request crate. [\#31](https://github.com/rsfetch/rsfetch/issues/31)
- Use `switch` instead of `bool` for args [\#26](https://github.com/rsfetch/rsfetch/issues/26)

**Merged pull requests:**

- Add Python \(pip\), Void \(xbps\), and Fedora \(dnf\) support to package counts. [\#37](https://github.com/rsfetch/rsfetch/pull/37) ([Phate6660](https://github.com/Phate6660))
- Add debian/ubuntu \(apt\) support for package count. [\#36](https://github.com/rsfetch/rsfetch/pull/36) ([Phate6660](https://github.com/Phate6660))
- Add detailed benchmarks [\#35](https://github.com/rsfetch/rsfetch/pull/35) ([kiedtl](https://github.com/kiedtl))
- Change WM to include DE too. [\#34](https://github.com/rsfetch/rsfetch/pull/34) ([Phate6660](https://github.com/Phate6660))
- Use reqwest for the IP address [\#33](https://github.com/rsfetch/rsfetch/pull/33) ([lnicola](https://github.com/lnicola))
- Add DE support [\#32](https://github.com/rsfetch/rsfetch/pull/32) ([lnicola](https://github.com/lnicola))

## [1.6.6](https://github.com/rsfetch/rsfetch/tree/1.6.6) (2019-05-12)

[Full Changelog](https://github.com/rsfetch/rsfetch/compare/1...1.6.6)

**Merged pull requests:**

- Changed the way options work. [\#27](https://github.com/rsfetch/rsfetch/pull/27) ([Phate6660](https://github.com/Phate6660))

## [1](https://github.com/rsfetch/rsfetch/tree/1) (2019-05-07)

[Full Changelog](https://github.com/rsfetch/rsfetch/compare/1.5.0...1)

## [1.5.0](https://github.com/rsfetch/rsfetch/tree/1.5.0) (2019-05-07)

[Full Changelog](https://github.com/rsfetch/rsfetch/compare/1.4.0...1.5.0)

**Implemented enhancements:**

- add link to aur package from readme [\#19](https://github.com/rsfetch/rsfetch/pull/19) ([kiedtl](https://github.com/kiedtl))

**Closed issues:**

- Update description [\#24](https://github.com/rsfetch/rsfetch/issues/24)
- move this repo to organization [\#18](https://github.com/rsfetch/rsfetch/issues/18)
- Stop going the easy way. [\#13](https://github.com/rsfetch/rsfetch/issues/13)
- Use proper error handling. [\#11](https://github.com/rsfetch/rsfetch/issues/11)
- make the term script included in the main file [\#4](https://github.com/rsfetch/rsfetch/issues/4)

**Merged pull requests:**

- Improve error handling [\#25](https://github.com/rsfetch/rsfetch/pull/25) ([lnicola](https://github.com/lnicola))
- Misc cleanups [\#23](https://github.com/rsfetch/rsfetch/pull/23) ([lnicola](https://github.com/lnicola))
- Add logo && badges to readme [\#22](https://github.com/rsfetch/rsfetch/pull/22) ([kiedtl](https://github.com/kiedtl))
- Optimize uptime retrieval [\#21](https://github.com/rsfetch/rsfetch/pull/21) ([lnicola](https://github.com/lnicola))
- change fetch to rsfetch in makefile [\#17](https://github.com/rsfetch/rsfetch/pull/17) ([kiedtl](https://github.com/kiedtl))

## [1.4.0](https://github.com/rsfetch/rsfetch/tree/1.4.0) (2019-05-07)

[Full Changelog](https://github.com/rsfetch/rsfetch/compare/1.3.8...1.4.0)

**Closed issues:**

- Vote For Name Here [\#10](https://github.com/rsfetch/rsfetch/issues/10)

**Merged pull requests:**

- Optimize package counting [\#16](https://github.com/rsfetch/rsfetch/pull/16) ([lnicola](https://github.com/lnicola))
- Fix /etc/os-release parsing [\#15](https://github.com/rsfetch/rsfetch/pull/15) ([lnicola](https://github.com/lnicola))
- Fix shell detection [\#14](https://github.com/rsfetch/rsfetch/pull/14) ([lnicola](https://github.com/lnicola))

## [1.3.8](https://github.com/rsfetch/rsfetch/tree/1.3.8) (2019-05-06)

[Full Changelog](https://github.com/rsfetch/rsfetch/compare/1.3.0...1.3.8)

**Merged pull requests:**

- update makefile [\#9](https://github.com/rsfetch/rsfetch/pull/9) ([kiedtl](https://github.com/kiedtl))

## [1.3.0](https://github.com/rsfetch/rsfetch/tree/1.3.0) (2019-05-03)

[Full Changelog](https://github.com/rsfetch/rsfetch/compare/1.2.1...1.3.0)

## [1.2.1](https://github.com/rsfetch/rsfetch/tree/1.2.1) (2019-05-03)

[Full Changelog](https://github.com/rsfetch/rsfetch/compare/1.2.0...1.2.1)

## [1.2.0](https://github.com/rsfetch/rsfetch/tree/1.2.0) (2019-05-03)

[Full Changelog](https://github.com/rsfetch/rsfetch/compare/1.1.1...1.2.0)

## [1.1.1](https://github.com/rsfetch/rsfetch/tree/1.1.1) (2019-05-03)

[Full Changelog](https://github.com/rsfetch/rsfetch/compare/1.1.0...1.1.1)

**Closed issues:**

- discuss [\#2](https://github.com/rsfetch/rsfetch/issues/2)

## [1.1.0](https://github.com/rsfetch/rsfetch/tree/1.1.0) (2019-05-01)

[Full Changelog](https://github.com/rsfetch/rsfetch/compare/1.0.0...1.1.0)

**Merged pull requests:**

- Update Makefile [\#7](https://github.com/rsfetch/rsfetch/pull/7) ([kiedtl](https://github.com/kiedtl))
- Update README.md [\#6](https://github.com/rsfetch/rsfetch/pull/6) ([kiedtl](https://github.com/kiedtl))
- Major and minor improvements [\#5](https://github.com/rsfetch/rsfetch/pull/5) ([kiedtl](https://github.com/kiedtl))

## [1.0.0](https://github.com/rsfetch/rsfetch/tree/1.0.0) (2019-04-30)

[Full Changelog](https://github.com/rsfetch/rsfetch/compare/0.9.8...1.0.0)

**Merged pull requests:**

- Make title/logo bold [\#3](https://github.com/rsfetch/rsfetch/pull/3) ([kiedtl](https://github.com/kiedtl))

## [0.9.8](https://github.com/rsfetch/rsfetch/tree/0.9.8) (2019-04-30)

[Full Changelog](https://github.com/rsfetch/rsfetch/compare/0.9.7...0.9.8)

## [0.9.7](https://github.com/rsfetch/rsfetch/tree/0.9.7) (2019-04-30)

[Full Changelog](https://github.com/rsfetch/rsfetch/compare/0.9.5...0.9.7)

**Closed issues:**

- How to install? [\#1](https://github.com/rsfetch/rsfetch/issues/1)

## [0.9.5](https://github.com/rsfetch/rsfetch/tree/0.9.5) (2019-04-29)

[Full Changelog](https://github.com/rsfetch/rsfetch/compare/0.9.3...0.9.5)

## [0.9.3](https://github.com/rsfetch/rsfetch/tree/0.9.3) (2019-04-29)

[Full Changelog](https://github.com/rsfetch/rsfetch/compare/0.9.1...0.9.3)

## [0.9.1](https://github.com/rsfetch/rsfetch/tree/0.9.1) (2019-04-28)

[Full Changelog](https://github.com/rsfetch/rsfetch/compare/54ee164690375e857f48d58ec2509d06759af3b5...0.9.1)



\* *This Changelog was automatically generated by [github_changelog_generator](https://github.com/github-changelog-generator/github-changelog-generator)*
