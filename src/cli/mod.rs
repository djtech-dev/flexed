pub mod binary;
pub mod config;
pub mod profile;
pub mod start;
pub mod wallpaper;
pub mod dm;
pub mod shortcuts;

use clap::{Parser, Subcommand};
use crate::cli::binary::*;
use crate::cli::config::*;
use crate::cli::profile::*;
use crate::cli::start::*;
use crate::cli::wallpaper::*;
use crate::cli::dm::*;
use crate::cli::shortcuts::*;


#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manage Flexed installation
    #[clap(subcommand)]
    Binary(BinaryCli),
    /// Manage Flexed Packaged Configurations
    #[clap(subcommand)]
    Config(ConfigCli),
    /// Manage Flexed Desktop Profile
    #[clap(subcommand)]
    Profile(ProfileCli),
    /// Download/set/upload wallpaper(s)
    #[clap(subcommand)]
    Wallpaper(WallpaperCli),
    /// Start the DE
    Start(StartCli),
    /// Kill all running Components (will restart most of the DE)
    Reboot(RebootCli),
    /// List/edit/reset/sync keyboard shortcuts
    #[clap(subcommand)]
    Shortcuts(ShortcutsCli),
    /// Set the Display Manager
    #[clap(subcommand)]
    DM(DmCli),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}
