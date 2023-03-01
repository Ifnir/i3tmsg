use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::{str, env};
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
#[derive(Debug, PartialEq, Eq)]
struct Arg {
    number: u8,
    icon: String
}

fn get_args(len: u8)-> Result<Vec<Arg>> {
    let mut args_vec = Vec::new();
    for i in 1..=len {
        let arg = env::args().nth(i.into()).unwrap_or_default();
        let icon = if arg.is_empty() { i.to_string() } else { arg };
        args_vec.push(Arg {number: i, icon: icon });
    }
    Ok(args_vec)
}
/**
 * Return Result of String
 */
fn workspace() -> Result<String> {
    let output = Command::new("i3-msg").args(["-t", "get_workspaces"]).output().expect("failed to execute process"); // i3 workspaces output

    // Get stdout in bytes and convert them from utf8
    let s = match str::from_utf8(&output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    // Create json format from string to vector Window
    let windows: Vec<Window> = serde_json::from_str(s)?;

    let args = get_args(windows.len().try_into().unwrap())?;
    println!("{:?}", args);
 
    // Make empty mutable vector for strings
    let mut works_vec = Vec::new();

    for window in windows {
        if let Some(arg) = args.iter().find(|a| a.number == window.name.parse::<u8>().unwrap_or(0)) {
            let icon = if arg.icon.is_empty() {
                format!("{}", window.name)
            } else {
                arg.icon.clone()
            };
            // use the icon variable here
            if !window.visible {
                works_vec.push(format!("(button :onclick \"i3-msg 'workspace {}'\" :class \"{}\" \"{}\")", window.name, "unoccupied", icon));
            }
            if window.visible && !window.focused {
                works_vec.push(format!("(button :onclick \"i3-msg 'workspace {}'\" :class \"{}\" \"{}\")", window.name, "occupied", icon));
            }
            if window.focused {
                works_vec.push(format!("(button :onclick \"i3-msg 'workspace {}'\" :class \"{}\" \"{}\")", window.name,"focused", icon));
            }
        }
        

    }
    // Put "sep" between every button for eww to use expression replace, and then format the vector to a string and return it
    let works_str = works_vec.join("sep");
    let rust_str = format!("{}", works_str);

    Ok(rust_str)
}

fn get_workspace() {
    workspace().map(|msg| println!("(box :class \"works\" :orientation \"h\" :halign \"start\" :space-evenly false {})", msg)).ok();
}

fn main() -> std::io::Result<()> {
    // Execute workspace() at beginning of program, then we subscribe to window and workspace
    get_workspace(); // Separate this into a new fn to avoid duplicate
    let i3_msg_process = Command::new("i3-msg")
    .args(["-t", "subscribe", "-m", r#"["window", "workspace"]"#])
    .stdout(Stdio::piped())
    .spawn()?;

    // Read the buffer
    let reader = BufReader::new(i3_msg_process.stdout.expect("failed to get stdout"));

    // Watch any change
    for _line in reader.lines() {
        get_workspace()
    }

    Ok(())
}
