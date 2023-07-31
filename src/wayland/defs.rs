use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub enum WaylandNotificator {
    Mako
}
#[derive(Serialize, Deserialize, Debug)]
pub enum WaylnadScreenLocker {
    Swaylock
}
#[derive(Serialize, Deserialize, Debug)]
pub enum WaylandScreenCapture {
    Grim
}
#[derive(Serialize, Deserialize, Debug)]
pub enum WaylandWallpaperUI {
    Swaybg
}
#[derive(Serialize, Deserialize, Debug)]
pub enum WaylandAppLauncher {
    Yofi,
    Wofi, // Inspired by `yofi`
}
