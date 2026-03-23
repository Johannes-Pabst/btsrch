use std::process::Stdio;

use tokio::{io::AsyncReadExt, process::Command};

/// returns Some(()) iff it fails
pub async fn run_in_terminal(command: String) -> Option<()> {
    if let Ok(s) = std::env::var("TERMINAL") {
        try_exec(&command, s).await?;
    }
    let mut s = String::new();
    if let Ok(c1) = Command::new("gsettings")
        .arg("get")
        .arg("org.gnome.desktop.default-applications.terminal")
        .arg("exec")
        .stdout(Stdio::piped())
        .spawn()
    {
        if let Some(mut c2) = c1.stdout {
            if let Ok(_) = c2.read_to_string(&mut s).await {
                if !s.is_empty() {
                    try_exec(&command, s).await?;
                }
            }
        }
    }
    let mut s = String::new();
    if let Ok(c1) = Command::new("gsettings")
        .arg("get")
        .arg("org.cinnamon.desktop.default-applications.terminal")
        .arg("exec")
        .stdout(Stdio::piped())
        .spawn()
    {
        if let Some(mut c2) = c1.stdout {
            if let Ok(_) = c2.read_to_string(&mut s).await {
                if !s.is_empty() {
                    try_exec(&command, s).await?;
                }
            }
        }
    }
    Some(())
}
pub async fn try_exec(command: &String, mut terminal: String) -> Option<()> {
    terminal=terminal.trim().to_string();
    if terminal.starts_with("'")&&terminal.ends_with("'"){
        terminal=terminal[1..terminal.len()-1].to_string();
    }
    println!("{command} {:?}", terminal);
    let spawn = Command::new(terminal).arg("--").arg(command).spawn();
    if spawn.is_ok() {
        spawn.unwrap().wait().await.unwrap();
        None
    } else {
        // println!("{:?}", spawn.unwrap_err());
        Some(())
    }
}
