use serde_json::json;
use std::env;
use std::process;
use swayipc::{Connection, EventType};

fn handle_workspace_change(output: &str) {
    let mut connection = Connection::new().unwrap();
    let workspaces = connection.get_workspaces().unwrap();

    let mut workspaces_array = Vec::new();

    for workspace in workspaces {
        if workspace.output == output {
            let name = workspace.name;
            let id: i32 = name
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse()
                .unwrap_or(0);
            let active = workspace.focused;
            let urgent = workspace.urgent;
            let visible = workspace.visible;

            let mut class = format!("workspace-button w{}", id);
            if active && visible && !urgent {
                class = format!("{} workspace-active wa{}", class, id);
            }
            if visible && !active && !urgent {
                class = format!("{} workspace-visible wv{}", class, id);
            }
            if urgent {
                class = format!("{} workspace-urgent wu{}", class, id);
            }

            let workspace_json = json!({
                "name": name,
                "id": id,
                "active": active,
                "urgent": urgent,
                "visible": visible,
                "class": class
            });

            workspaces_array.push(workspace_json);
        }
    }

    let result = json!(workspaces_array);
    println!("{}", result.to_string());
}

fn display_workspaces(output: String) {
    handle_workspace_change(&output);

    let mut connection = Connection::new().unwrap();
    let events = connection.subscribe(&[EventType::Workspace]).unwrap();

    for event in events {
        match event.unwrap() {
            swayipc::Event::Workspace(_) => handle_workspace_change(&output),
            _ => {}
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <output>", args[0]);
        process::exit(1);
    }

    let output = args[1].clone();
    display_workspaces(output);
}
