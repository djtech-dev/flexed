use serde::{Serialize, Deserialize};
pub use crate::wayland::defs::*;
pub use crate::xorg::defs::*;

/// Your own X11 Desktop Environment
#[derive(Serialize, Deserialize, Debug)]
pub struct X11Desktop {
    window_manager: X11WindowManager,
    taskbar: Option<X11TaskBar>,
    statusbar: Option<X11StatusBar>,
    notificator: Option<X11Notificator>,
    screen_locker: Option<X11ScreenLocker>,
    audio_control: Option<X11AudioControl>,
    system_tray: Option<X11SystemTray>,
    wallpaper_ui: Option<X11WallpaperUI>,
    screen_capture: Option<X11ScreenCapture>,
    power_manager: Option<X11PowerManager>,
    backlight_ui: Option<X11BacklightUI>,
    logout_ui: Option<X11LogoutUI>,
    polkit_agent: Option<X11Polkit>,
    clipboard_manager: Option<X11ClipboardManager>,
    app_launcher: Option<X11AppLauncher>,

    enable_mousetweaks: bool, // Requires `mousetweaks`
    enable_screenkey: bool,   // Requires `screenkey`
    force_pipewire: bool,     // Check if `pipewire` is running, if not start it up.
}
/// Your own Wayland Desktop Environment
#[derive(Serialize, Deserialize, Debug)]
pub struct WaylandDesktop {
    window_manager: WLWindowManager,

    enable_showmethekey: bool, // Requires `showmethekey`
    force_pipewire: bool,     // Check if `pipewire` is running, if not start it up.
}

impl X11Desktop {
    pub fn new() -> X11Desktop {
        X11Desktop {
            window_manager: X11WindowManager::Openbox,
            taskbar: None,
            statusbar: None,
            notificator: None,
            screen_locker: None,
            audio_control: None,
            system_tray: None,
            wallpaper_ui: None,
            screen_capture: None,
            power_manager: None,
            backlight_ui: None,
            logout_ui: None,
            polkit_agent: None,
            clipboard_manager: None,
            app_launcher: None,

            enable_mousetweaks: false, // Requires `mousetweaks`
            enable_screenkey: false,   // Requires `screenkey`
            force_pipewire: true,     // Check if `pipewire` is running, if not start it up.
        }
    }
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
    GDM, // `gdm`
    LightDM,
    SDDM,
    GreetdGTK, // `greetd`; NOTE: Login demon, not DM; for more see https://wiki.archlinux.org/title/Greetd. When required from here will use GTK
    GreetdQT, // `greetd`; NOTE: Login demon, not DM; for more see https://wiki.archlinux.org/title/Greetd. When required from here will use Qt
    GreetdWayland, // `greetd`; NOTE: Login demon, not DM; for more see https://wiki.archlinux.org/title/Greetd. When required from here will be compatible with Wayland
}
