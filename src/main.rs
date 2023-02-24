use std::process::{Command};
use std::str;
use serde::{Serialize, Deserialize};
use serde_json::{Result};

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
struct Window {
    id: i64,
    num: i64,
    name: String,
    visible: bool,
    focused: bool,
    output: String,
    urgent: bool,
}



fn main() -> Result<()> {
    let output = Command::new("i3-msg").args(["-t", "get_workspaces"]).output().expect("failed to execute process");

    let s = match str::from_utf8(&output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let windows: Vec<Window> = serde_json::from_str(s)?;

    for window in windows {
        if window.focused {
            println!("focused_window: {}", window.name);
        }
    }

    Ok(())
}
