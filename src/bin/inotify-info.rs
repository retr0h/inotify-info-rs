// #[cfg(target_os = "linux")]
// mod linux;

// #[cfg(not(target_os = "linux"))]
// mod linux_not;

// #[cfg(target_os = "linux")]
// fn main() {
//     linux::main()
// }

// #[cfg(not(target_os = "linux"))]
// fn main() {
//     linux_not::main()
// }

use clap::Parser;

use inotify_info_rs::greet;

#[derive(Parser, Debug)]
#[command(
    name = "inotify-info-rs",
    about = r#"

        ,  ._            ._
*._  _ -+-*|,  . ___ *._ |, _  ___ ._. __
|[ )(_) | || \_|     |[ )| (_)     [  _)
             ._|

Linux inotify info reporting rust app.

https://github.com/retr0h/gilt
"#
)]
struct Cli {
    /// Specify the PID filter
    #[arg(long)]
    pid: Option<String>,

    /// Specify the application name filter
    #[arg(long = "app-name")]
    app_name: Option<String>,
}

fn get_args() -> Cli {
    Cli::parse()
}

fn main() {
    let args = get_args();

    if let Some(pid) = args.pid {
        println!("PID: {}", pid);
    } else if let Some(app_name) = args.app_name {
        println!("App Name: {}", app_name);
    } else {
        println!("No arguments provided.");
    }

    greet();
}
