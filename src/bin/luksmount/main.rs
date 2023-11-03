use clap::Parser;

mod cli;

fn main() {
    let cli = cli::Cli::parse();
    dbg!(cli.clone());

    let device_mapper = format!("luksmount-{}", cli.mnt.replace("/", "__"));
    dbg!(device_mapper.clone());

    luxutil::run_command(
        "cryptsetup",
        ["open", cli.dev.as_str(), device_mapper.as_str()],
        format!("Failed to open encrypted volume {}", cli.dev).as_str(),
        luxutil::QuitOn::Error,
    );

    let device_mapper = format!("/dev/mapper/{}", device_mapper);

    let mut mount_args = vec![
        "-t",
        cli.fstype.as_str(),
        device_mapper.as_str(),
        cli.mnt.as_str(),
    ];

    if cli.mkdir {
        mount_args.append(&mut vec!["--mkdir"]);
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
