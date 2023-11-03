use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about = "Mount a LUKS encrypted filesystem", long_about = None)]
pub(crate) struct Cli {
    /// The mount directory (eg. /mnt)
    pub(crate) mnt: String,
    /// Try to remove the mount directory
    #[arg(short, long)]
    pub(crate) rmdir: bool,
}
