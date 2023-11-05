use clap::Parser;

mod cli;

fn main() {
    let cli = cli::Cli::parse();

    let device_mapper = format!("luksmount-{}", cli.mnt.replace("/", "__"));

    luxutil::run_command(
        "cryptsetup",
        ["open", &cli.dev, &device_mapper],
        format!("Failed to open encrypted volume {}", cli.dev).as_str(),
        luxutil::QuitOn::Error,
    );

    let device_mapper = format!("/dev/mapper/{}", device_mapper);

    let mut mount_args = vec!["-t", &cli.fstype, &device_mapper, &cli.mnt];

    if cli.mkdir {
        mount_args.push("--mkdir");
    }

    luxutil::run_command(
        "mount",
        mount_args,
        format!(
            "Failed to mount volume {} at {} via mapper volume {}",
            cli.dev, cli.mnt, device_mapper
        )
        .as_str(),
        luxutil::QuitOn::Error,
    );
}
