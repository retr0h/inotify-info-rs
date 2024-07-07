use std::fs::File;
use std::io::{self, Read};
use std::iter;
use std::path::Path;

use colored::Colorize;

mod cli;

fn print_separator() {
    println!(
        "{}",
        iter::repeat('-').take(78).collect::<String>().yellow(),
    );
}

fn get_inotify_procfs_value(fname: &str) -> io::Result<String> {
    let filename = format!("/proc/sys/fs/inotify/{}", fname);
    let path = Path::new(&filename);

    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    Ok(buf.trim().to_string())
}

fn print_inotify_limits() {
    let filenames = vec![
        "invalid",
        "max_queued_events",
        "max_user_instances",
        "max_user_watches",
    ];

    println!("{}", "INotify Limits:".cyan());
    for &fname in &filenames {
        match get_inotify_procfs_value(fname) {
            Ok(val) => println!("  {:<20} {}", fname, val.green()),
            Err(e) => println!("  {:<20} {:<10} - {}", fname, "N/A".red(), e),
        }
    }
}

pub fn main() {
    let args = cli::get_args();

    if let Some(pid) = args.pid {
        println!("PID: {}", pid);
    } else if let Some(app_name) = args.app_name {
        println!("App Name: {}", app_name);
    } else {
        println!("No arguments provided.");
    }

    print_separator();
    print_inotify_limits();
    print_separator();
}
