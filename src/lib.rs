#[cfg(test)]
mod tests {
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
    
    #[test]
    fn test_get_focused_window() -> Result<()> {
        let json_string = r#"[
            {
                "id": 1,
                "num": 1,
                "name": "1",
                "visible": false,
                "focused": false,
                "output": "DP-1",
                "urgent": false
            },
            {
                "id": 2,
                "num": 2,
                "name": "2",
                "visible": true,
                "focused": true,
                "output": "DP-1",
                "urgent": false
            },
            {
                "id": 3,
                "num": 3,
                "name": "3",
                "visible": true,
                "focused": false,
                "output": "DP-1",
                "urgent": false
            }
        ]"#;
        
        let windows: Vec<Window> = serde_json::from_str(json_string)?;
        
        assert_eq!(windows.len(), 3);
        
        let focused_window = windows.iter().find(|w| w.focused);
        assert!(focused_window.is_some());
        
        let focused_window = focused_window.unwrap();
        assert_eq!(focused_window.id, 2);
        assert_eq!(focused_window.num, 2);
        assert_eq!(focused_window.name, "2");
        assert_eq!(focused_window.visible, true);
        assert_eq!(focused_window.focused, true);
        assert_eq!(focused_window.output, "DP-1");
        assert_eq!(focused_window.urgent, false);
        
        Ok(())
    }
}