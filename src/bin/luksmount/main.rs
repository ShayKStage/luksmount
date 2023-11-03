use clap::Parser;
use std::process::Command;

mod cli;

fn main() {
    let cli = cli::Cli::parse();
    dbg!(cli.clone());

    let device_mapper = "luksmount-".to_string() + cli.mnt.replace("/", "__").as_str();
    dbg!(device_mapper.clone());

    match Command::new("cryptsetup")
        .arg("open")
        .arg(cli.dev.clone())
        .arg(device_mapper.clone())
        .status()
    {
        Ok(status) => {
            if !status.success() {
                eprintln!("Failed to open encrypted volume {}", cli.dev);
                std::process::exit(status.code().unwrap_or(-1))
            }
        }
        Err(error) => {
            eprintln!("Command \"cryptsetup\" failed with error: {}", error);
            std::process::exit(-1)
        }
    }

    let mut mount_cmd = Command::new("mount");
    mount_cmd
        .arg("-t")
        .arg(cli.fstype)
        .arg(format!("/dev/mapper/{}", device_mapper.clone()))
        .arg(cli.mnt.clone());

    if cli.mkdir {
        mount_cmd.arg("--mkdir");
    }

    match mount_cmd.status() {
        Ok(status) => {
            if !status.success() {
                eprintln!(
                    "Failed to mount volume {} at {} via mapper volume {}",
                    cli.dev, cli.mnt, device_mapper
                );
                std::process::exit(status.code().unwrap_or(-1))
            }
        }
        Err(error) => {
            eprintln!("Command \"mount\" failed with error: {}", error);
            std::process::exit(-1)
        }
    }
}
