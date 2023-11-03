use clap::Parser;

mod cli;

fn main() {
    let cli = cli::Cli::parse();
    dbg!(cli.clone());

    let device_mapper = format!("luksmount-{}", cli.mnt.replace("/", "__"));
    dbg!(device_mapper.clone());

    luxutil::run_command(
        "umount",
        [cli.mnt.clone()],
        format!("Failed to unmount {}", cli.mnt).as_str(),
        luxutil::QuitOn::Error,
    );

    luxutil::run_command(
        "cryptsetup",
        ["close", device_mapper.as_str()],
        format!("Failed to close encrypted mapper volume {}", device_mapper).as_str(),
        luxutil::QuitOn::Error,
    );

    if !cli.rmdir {
        std::process::exit(0)
    }

    luxutil::run_command(
        "rm",
        ["-d", cli.mnt.as_str()],
        format!("Failed to remove {}", cli.mnt).as_str(),
        luxutil::QuitOn::Error,
    );
}
