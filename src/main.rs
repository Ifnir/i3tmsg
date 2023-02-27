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

fn workspace() -> Result<()> {
    let output = Command::new("i3-msg").args(["-t", "get_workspaces"]).output().expect("failed to execute process");

    let s = match str::from_utf8(&output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let windows: Vec<Window> = serde_json::from_str(s)?;

    let mut works_vec = Vec::new();
    for window in windows {
        if !window.visible {
            works_vec.push(format!("(button :onclick \"i3-msg 'workspace {}'\" :class \"works\" \"{}\")", window.name, "unoccupied"));
        }
        if window.visible {
            works_vec.push(format!("(button :onclick \"i3-msg 'workspace {}'\" :class \"works\" \"{}\")", window.name, "occupied"));
        }
        if window.focused {
            works_vec.push(format!("(button :onclick \"i3-msg 'workspace {}'\" :class \"works\" \"{}\")", window.name,"focused"));
        }
    }

    let works_str = works_vec.join("\n\t\t\t");
    let rust_str = format!("(box :class \"works\" :orientation \"h\" :halign \"center\" :space-evenly \"false\" :spacing \"12\"\n\t{}\n)", works_str);

    println!("{}", rust_str);

    Ok(())
}

fn main() -> std::io::Result<()> {
    let i3_msg_process = Command::new("i3-msg")
    .args(["-t", "subscribe", "-m", r#"["window", "workspace"]"#])
    .stdout(Stdio::piped())
    .spawn()?;

    let reader = BufReader::new(i3_msg_process.stdout.expect("failed to get stdout"));

    for _line in reader.lines() {
        workspace().map_err(|err| println!("{:?}", err)).ok();
    }

    Ok(())
}
