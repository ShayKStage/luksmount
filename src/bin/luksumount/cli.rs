use clap::Parser;

#[derive(Parser)]
#[command(author, version, about = "Unmount a LUKS encrypted filesystem", long_about = None)]
pub struct Cli {
    /// The mount directory (eg. /mnt)
    pub(crate) mnt: String,
    /// Try to remove the mount directory
    #[arg(short, long)]
    pub(crate) rmdir: bool,
}
