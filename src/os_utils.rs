use tokio::process::Command;

pub fn run_in_terminal(command:String){
    let terminal=std::env::var("TERMINAL").ok().or_else(||{
        Command::new("gsettings get org.gnome.desktop.default-applications.terminal exec").arg("get").arg("org.cinnamon.desktop.default-applications.terminal")
    })
}