# i3tmsg
To handle workspaces in i3 with eww widget.

### Description

This is first project in Rust, I created because I like to use i3 and https://github.com/elkowar/eww.
However, eww and i3 didn't work out of box together, I wanted to avoid using bash jq and start my journey into Rust.

### Install Guide
The idea is to set all usable workspaces in eww and use the i3tmsg to print out with awk the focused window.
```
git clone git@github.com:Ifnir/i3tmsg.git
cd i3tsmg
cargo build --release
cd target/release
./i3tmsg | awk '{print $2}'
```

