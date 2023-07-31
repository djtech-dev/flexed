use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub enum X11WindowManager {
    TwoBWM,   // 2bwm
    BlackBox,
    Compiz,
    Cwm,
    Enlightment,
    Evilwm,
    Fluxbox,
    IceWM,
    Kwin,
    Metacity,
    Openbox,
    Dwm,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum X11TaskBar {
    Eww,
    CairoDock, // cairo-dock
    Lemonbar,
    Plank,
    Polybar,
    Tint2,
    Yambar,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum X11StatusBar {
    Polybar,
    Tint2,
    Eww,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum X11SystemTray {
    Trayer,
    Stalonetray,
    AllTray,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum X11Notificator {
    Dunst,
    NotifyOSD,
    LxQT, // lxqt-notificationd
}
#[derive(Serialize, Deserialize, Debug)]
pub enum X11ScreenLocker {
    Sflock,
    Physlock,
    Mate, // mate-screensaver
}
#[derive(Serialize, Deserialize, Debug)]
pub enum X11AudioControl {
    Emixer,
    Qastools,
    MateMedia, // mate-media
}
#[derive(Serialize, Deserialize, Debug)]
pub enum X11AppLauncher {
    Xlunch, // https://xlunch.org/
    Dmenu,
    Xmobar
}
#[derive(Serialize, Deserialize, Debug)]
pub enum X11WallpaperUI {
    Nitrogen
}
#[derive(Serialize, Deserialize, Debug)]
pub enum X11ScreenCapture {
    Flameshot,
    LxQT,
    Ksnip,
    Shotgun
}
#[derive(Serialize, Deserialize, Debug)]
pub enum X11PowerManager {
    LxQT
    Cbatticon
}
#[derive(Serialize, Deserialize, Debug)]
pub enum X11BacklightUI {
    Acpilight,       // Classic
    Clight,          // Support for both keybindings and ambient brightness
    MacbookLighter,  // macbook-lighter (AUR); Support for both keybindings and ambient brightness
    Wluma,           // Support for both keybindings and ambient brightness
}
#[derive(Serialize, Deserialize, Debug)]
pub enum X11LogoutUI {
    Clearine,
    Oblogout,
    LxQT, // [Launch `lxqt-leave`]
}
#[derive(Serialize, Deserialize, Debug)]
pub enum X11Polkit {
    PolkitDumbAgent, // polkit-dump-agent-git (AUR)
    LxQT,            // lxqt-policykit
}
#[derive(Serialize, Deserialize, Debug)]
pub enum X11ClipboardManager {
    CopyQ,
    Clipcat,
    Xclipper,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum X11AppLauncher {
    Launchy,
    Rofi,
    Rlaunch,
    Synapse,
    Kupfer,
}
