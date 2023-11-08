#![deny(clippy::pedantic)]

use clap::Parser;

mod cli;

fn main() {
    let cli = cli::Cli::parse();

    let device_mapper = format!("luksmount-{}", cli.mnt.replace('/', "__"));

    luksmount::run_command(
        "umount",
        [&cli.mnt],
        format!("Failed to unmount {}", cli.mnt).as_str(),
        luksmount::QuitOn::Error,
    );

    luksmount::run_command(
        "cryptsetup",
        ["close", &device_mapper],
        format!("Failed to close encrypted mapper volume {device_mapper}").as_str(),
        luksmount::QuitOn::Error,
    );

    if !cli.rmdir {
        std::process::exit(0)
    }

    luksmount::run_command(
        "rm",
        ["-d", &cli.mnt],
        format!("Failed to remove {}", cli.mnt).as_str(),
        luksmount::QuitOn::Error,
    );
}
