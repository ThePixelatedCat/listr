A barebones command-line list manager written in Rust.

Usage: listr 
That's it, no arguments or flags

Internal Commands
_________________
rm [NAME]: delete the specified list or item. will prompt for confirmation
mk [NAME]: make a new list or item with the specified name
name [NAME]: set the name of the specified list or item. will prompt for new name input
open [NAME]: open the specified list, or view the description of the specified item
desc [NAME]: set the description of the specified item. will prompt for new description (item level only)
help: view this list of commands
exit: save your changes and quit the application (list level), return to list level (item level)

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Listr by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>