use serde::{Serialize, Deserialize};
pub use crate::wayland::*;
pub use crate::xorg::*;

/// Your own X11 Desktop Environment
#[derive(Serialize, Deserialize, Debug)]
pub struct X11Desktop {
    window_manager: X11WindowManager,
    taskbar: Option<X11TaskBar>,
    statusbar: Option<X11StatusBar>,
    notificator: Option<X11Notificator>,
    screen_locker: Option<X11ScreenLocker>,
    audio_control: Option<X11AudioControl>,

    enable_mousetweaks: bool, // Requires `mousetweaks`
    enable_screenkey: bool,   // Requires `screenkey`
}
/// Your own Wayland Desktop Environment
#[derive(Serialize, Deserialize, Debug)]
pub struct WaylandDesktop {

    enable_showmethekey: bool, // Requires `showmethekey`
}

impl X11Desktop {
    pub fn launch(self) {}
    pub fn get_arch_deps(self) {}
}
impl WaylandDesktop {
    pub fn launch(self) {}
    pub fn get_arch_deps(self) {}
}

/// Profile that implements Desktop for X11/Wayland/Both
#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub author: String,
    pub version: String,
    pub url: String,

    pub x11: Option<X11Desktop>,
    pub wayland: Option<WaylandDesktop>,

    pub favourite_dm: Option<DisplayManager>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DisplayManager {
    Ly,
    Gdm, // `gdm`
    LightDM,
    SDDM,
    GreetdGTK, // `greetd`; NOTE: Login demon, not DM; for more see https://wiki.archlinux.org/title/Greetd. When required from here will use GTK
    GreetdQT, // `greetd`; NOTE: Login demon, not DM; for more see https://wiki.archlinux.org/title/Greetd. When required from here will use Qt
    GreetdWayland, // `greetd`; NOTE: Login demon, not DM; for more see https://wiki.archlinux.org/title/Greetd. When required from here will be compatible with Wayland
}
