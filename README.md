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
copy i3tmsg to your script folder
```

#### Eww Widget

Add this to your .config/eww/scripts folder

```
(deflisten workspace "scripts/i3tmsg") 
(defwidget _workspaces []
  (literal :content {replace(workspace, "sep", "
   ")})
)
```

i3tmsg can take arguments like icons

```
(deflisten workspace "scripts/i3tmsg  s a") 

For the eww.scss file
```
.works {
   
}

.focused {
  color: #61AAD6;
}
.urgent {
  color: #f00a0a;
}
.unoccupied {
  color: #ff00ea;
}
.occupied {
  color: #ffffff;
}
```
