pub mod desktop;
pub mod xorg;
pub mod wayland;
pub mod cli;

pub use crate::cli::Cli;
use clap::Parser;

pub fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
}
