use clap::{Subcommand, Args};
#[derive(Debug, Subcommand)]
pub enum BinaryCli {
    Install(InstallArgs),
    Uninstall(UninstallArgs),
    /// Update Local Source code from Github
    ///  | Saved at ~/.flexed/flexed or /usr/share/flexed/flexed
    SyncSrc,
    /// Build from Local Source Code
    BuildSrc(BuildArgs),
    /// Remove Local Source Code, while keeping default Profiles and Configs
    ///  | Will empty space in the disk, but re-building will take longer
    RmSrc,
}
#[derive(Debug, Args)]
pub struct InstallArgs {

}
#[derive(Debug, Args)]
pub struct UninstallArgs {

}
#[derive(Debug, Args)]
pub struct BuildArgs {

}
