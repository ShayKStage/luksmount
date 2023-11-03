use clap::Parser;
use std::process::Command;

mod cli;

fn main() {
    let cli = cli::Cli::parse();
    dbg!(cli.clone());

    let device_mapper = format!("luksmount-{}", cli.mnt.replace("/", "__"));
    dbg!(device_mapper.clone());

    match Command::new("umount").arg(cli.mnt.clone()).status() {
        Ok(status) => {
            if !status.success() {
                eprintln!("Failed to unmount {}", cli.mnt);
                std::process::exit(status.code().unwrap_or(-1))
            }
        }
        Err(error) => {
            eprintln!("Command \"umount\" failed with error: {}", error);
            std::process::exit(-1)
        }
    }

    match Command::new("cryptsetup")
        .arg("close")
        .arg(device_mapper.clone())
        .status()
    {
        Ok(status) => {
            if !status.success() {
                eprintln!("Failed to close encrypted mapper volume {}", device_mapper);
                std::process::exit(status.code().unwrap_or(-1))
            }
        }
        Err(error) => {
            eprintln!("Command \"cryptsetup\" failed with error: {}", error);
            std::process::exit(-1)
        }
    }

    if !cli.rmdir {
        std::process::exit(0)
    }

    match Command::new("rm").arg("-d").arg(cli.mnt.clone()).status() {
        Ok(status) => {
            if !status.success() {
                eprintln!("Failed to remove {}", cli.mnt);
                std::process::exit(status.code().unwrap_or(-1))
            }
        }
        Err(error) => {
            eprintln!("Command \"rm\" failed with error: {}", error);
            std::process::exit(-1)
        }
    }
}
