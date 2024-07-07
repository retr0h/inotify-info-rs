use clap::Parser;

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
pub struct Cli {
    /// Specify the PID filter
    #[arg(long)]
    pub pid: Option<String>,

    /// Specify the application name filter
    #[arg(long = "app-name")]
    pub app_name: Option<String>,
}

pub fn get_args() -> Cli {
    Cli::parse()
}
