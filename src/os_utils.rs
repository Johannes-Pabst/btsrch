use std::process::Stdio;

use tokio::{io::AsyncReadExt, process::Command};

/// returns Some(()) iff it fails
pub async fn run_in_terminal(command: String, keep_open: bool) -> Option<()> {
    if let Ok(s) = std::env::var("TERMINAL") {
        try_exec(&command, s, keep_open).await?;
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
                    try_exec(&command, s, keep_open).await?;
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
                    try_exec(&command, s, keep_open).await?;
                }
            }
        }
    }
    Some(())
}
pub async fn try_exec(command: &String, mut terminal: String, keep_open: bool) -> Option<()> {
    // if !exists_command(&terminal).await{
    //     return Some(())
    // }
    terminal = terminal.trim().to_string();
    if terminal.starts_with("'") && terminal.ends_with("'") {
        terminal = terminal[1..terminal.len() - 1].to_string();
    }
    println!("trying to execute {} in {}", command, terminal);
    let spawn = if keep_open {
        Command::new(terminal)
            .arg("--")
            .arg("bash")
            .arg("-c")
            .arg(command)
            .spawn()
    } else {
        Command::new(terminal).arg("--").arg(command).spawn()
    };
    if spawn.is_ok() {
        // spawn.unwrap().wait().await.unwrap();
        None
    } else {
        // println!("{:?}", spawn.unwrap_err());
        Some(())
    }
}
pub async fn exists_command(command: &String) -> bool {
    Command::new("bash")
        .arg("-c")
        .arg(format!("command -pv \"{}\"", command.split_whitespace().next().unwrap_or("")))
        .spawn()
        .unwrap()
        .wait()
        .await
        .unwrap()
        .success()
    // Command::new("which")
    //     .arg(command.split_whitespace().next().unwrap_or(""))
    //     .spawn()
    //     .unwrap()
    //     .wait()
    //     .await
    //     .unwrap()
    //     .success()
}
