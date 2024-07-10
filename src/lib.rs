use std::fs::File;
use std::io::{self, Read};
use std::iter;
use std::path::PathBuf;

use colored::Colorize;

const INOTIFY_DIR: &str = "/proc/sys/fs/inotify";

fn print_separator() {
    println!(
        "{}",
        iter::repeat('-').take(78).collect::<String>().yellow(),
    );
}

fn get_inotify_procfs_value(inotify_dir: &str, proc_name: &str) -> io::Result<String> {
    let filename = PathBuf::from(inotify_dir).join(proc_name);
    let mut file = File::open(filename)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    Ok(buf.trim().to_string())
}

fn print_inotify_limits() {
    let proc_names = vec![
        "max_queued_events",
        "max_user_instances",
        "max_user_watches",
    ];

    println!("{}", "INotify Limits:".cyan());
    for &proc_name in &proc_names {
        match get_inotify_procfs_value(&INOTIFY_DIR, proc_name) {
            Ok(val) => println!("  {:<20} {}", proc_name, val.green()),
            Err(e) => println!("  {:<20} {:<10} - {}", proc_name, "N/A".red(), e),
        }
    }
}

pub fn greet() {
    print_separator();
    print_inotify_limits();
    print_separator();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn test_get_inotify_procfs_value() {
        // Create a temporary directory
        let dir = tempdir().unwrap();
        let temp_dir_path = dir.path().to_str().unwrap().to_string();

        // Write a test value to the temporary file
        let file_path = Path::new(&temp_dir_path).join("max_user_watches");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "8192").unwrap();

        // Test the function with the temporary file
        let result = get_inotify_procfs_value(&temp_dir_path, "max_user_watches");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "8192");
    }

    #[test]
    fn test_get_inotify_procfs_value_fails_when_file_missing() {
        // Create a temporary directory
        let dir = tempdir().unwrap();
        let temp_dir_path = dir.path().to_str().unwrap().to_string();

        // Test the function with the temporary file
        let result = get_inotify_procfs_value(&temp_dir_path, "invalid");
        assert!(result.is_err());
        assert!(!result.is_ok());
    }
}
