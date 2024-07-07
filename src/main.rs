mod cli;

#[cfg(target_os = "linux")]
#[path = "linux.rs"]
mod platform;

#[cfg(not(target_os = "linux"))]
#[path = "linux_not.rs"]
mod platform;

fn main() {
    let args = cli::get_args();

    if let Some(pid) = args.pid {
        println!("PID: {}", pid);
    } else if let Some(app_name) = args.app_name {
        println!("App Name: {}", app_name);
    } else {
        println!("No arguments provided.");
    }

    platform::print_separator();
    platform::print_inotify_limits();
    platform::print_separator();
}
