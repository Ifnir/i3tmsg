use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
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

fn workspace() -> Result<String> {
    let output = Command::new("i3-msg").args(["-t", "get_workspaces"]).output().expect("failed to execute process");

    let s = match str::from_utf8(&output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let windows: Vec<Window> = serde_json::from_str(s)?;

    let mut works_vec = Vec::new();
    for window in windows {
        if !window.visible {
            works_vec.push(format!("(button :onclick \"i3-msg 'workspace {}'\" :class \"{}\" \"{}\")", window.name, "unoccupied", window.name));
        }
        if window.visible && !window.focused {
            works_vec.push(format!("(button :onclick \"i3-msg 'workspace {}'\" :class \"{}\" \"{}\")", window.name, "occupied", window.name));
        }
        if window.focused {
            works_vec.push(format!("(button :onclick \"i3-msg 'workspace {}'\" :class \"{}\" \"{}\")", window.name,"focused", window.name));
        }
    }

    let works_str = works_vec.join("sep");
    let rust_str = format!("{}", works_str);

    Ok(rust_str)
}

fn main() -> std::io::Result<()> {
    workspace().map(|msg| println!("(box :class \"works\" :orientation \"h\" :halign \"start\" :space-evenly false {})", msg)).ok();
    let i3_msg_process = Command::new("i3-msg")
    .args(["-t", "subscribe", "-m", r#"["window", "workspace"]"#])
    .stdout(Stdio::piped())
    .spawn()?;

    let reader = BufReader::new(i3_msg_process.stdout.expect("failed to get stdout"));

    for _line in reader.lines() {
        workspace().map(|msg| println!("(box :class \"works\" :orientation \"h\" :halign \"start\" :space-evenly false {})", msg)).ok();
    }

    Ok(())
}
