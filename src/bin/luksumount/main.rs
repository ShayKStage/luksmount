#![deny(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::fs;

use clap::Parser;
use inquire::Confirm;

mod cli;

fn main() {
    let cli = cli::Cli::parse();

    let device_mapper = format!("luksmount_internal-{}", cli.mnt.replace('/', "__"));

    luksmount_internal::run_command(
        "umount",
        [&cli.mnt],
        format!("Failed to unmount {}", cli.mnt).as_str(),
        luksmount_internal::QuitOn::Error,
    );

    luksmount_internal::run_command(
        "cryptsetup",
        ["close", &device_mapper],
        format!("Failed to close encrypted mapper volume {device_mapper}").as_str(),
        luksmount_internal::QuitOn::Error,
    );

    if !cli.rmdir {
        std::process::exit(0)
    }

    'rmdir: loop {
        match fs::remove_dir(&cli.mnt) {
            Ok(()) => break 'rmdir,
            Err(error) => {
                eprintln!(
                    "Failed to remove mount directory {} with error: {error}",
                    cli.mnt
                );

                let retry = Confirm::new("Retry?").with_default(false).prompt();
                match retry {
                    Ok(true) => continue 'rmdir,
                    Ok(false) => break 'rmdir,
                    Err(error) => {
                        eprintln!(
                            "Failed to get info from user with error: {error}, not retrying."
                        );
                        println!("To remove the mount directory manually run \"rm -d {}\" (without the quotes)", cli.mnt);
                        break 'rmdir;
                    }
                }
            }
        }
    }
}
