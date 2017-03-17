rust-dwm-status
---------------

A status bar for dwm.
Displays system information such as battery status (on laptops), ram usage, cpu load, date and time.
Handles system notifications.

- Freely inspired from [gods](https://github.com/schachmat/gods).
- We use [systemstat](https://github.com/myfreeweb/systemstat) for obtaining necessary info from linux system files.
- We use [notify-rust](https://github.com/hoodie/notify-rust) for handling notifications.

![alt tag](https://github.com/pierrechevalier83/rust-dwm-status/blob/master/screenshots/demo.png)

Dependencies
------------
- xsetroot
- rust
- A font that supports unicode characters

Installation
------------
- On Arch Linux:
`yaourt -S rust-dwm-status`

- With cargo
`cargo install rust-dwm-status`

Usage
-----
Either run directly or add to your .xinitrc:
`rust-dwm-status &`
