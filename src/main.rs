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
        "cryptsetup",
        ["open", &cli.dev, &device_mapper],
        format!("Failed to open encrypted volume {}", cli.dev).as_str(),
        luksmount_internal::QuitOn::Error,
    );

    if cli.mkdir {
        'mkdir: loop {
            match fs::create_dir_all(&cli.mnt) {
                Ok(()) => break 'mkdir,
                Err(error) => {
                    eprintln!(
                        "Failed to create mount directory {} with error: {error}",
                        cli.mnt
                    );

                    let retry = Confirm::new("Retry?").with_default(false).prompt();
                    match retry {
                        Ok(true) => continue 'mkdir,
                        Ok(false) => std::process::exit(-1),
                        Err(error) => {
                            eprintln!(
                                "Failed to get info from user with error: {error}, not retrying."
                            );
                            println!("To create the mount directory manually run \"mkdir {}\" (without the quotes)", cli.mnt);
                            std::process::exit(-1)
                        }
                    }
                }
            }
        }
    }

    let device_mapper = format!("/dev/mapper/{device_mapper}");

    luksmount_internal::run_command(
        "mount",
        ["-t", &cli.fstype, &device_mapper, &cli.mnt],
        format!(
            "Failed to mount volume {} at {} via mapper volume {}",
            cli.dev, cli.mnt, device_mapper
        )
        .as_str(),
        luksmount_internal::QuitOn::Error,
    );
}
