use clap::Parser;

#[derive(Parser)]
#[command(author, version, about = "Mount a LUKS encrypted filesystem", long_about = None)]
pub struct Cli {
    /// The block device to mount (eg. /dev/sdb1)
    pub(crate) dev: String,
    /// The directory to use as the mountpoint (eg. /mnt)
    pub(crate) mnt: String,
    /// The filesystem to mount it as (eg. ext4, btrfs, ntfs)
    #[arg(default_value = "ext4")]
    pub(crate) fstype: String,
    /// Try to create the mount directory
    #[arg(short, long)]
    pub(crate) mkdir: bool,
}
