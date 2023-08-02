pub use crate::desktop::X11Desktop;
pub use crate::xorg::defs::*;

impl X11Desktop {
    pub fn execute(self) {
        match self.window_manager {
            X11WindowManager::TwoBWM => { track("twobwm")},
            X11WindowManager::BlackBox => { track("blackbox") },
            X11WindowManager::Compiz => { track("compiz --replace") },
            X11WindowManager::Cwm => { track("cwm") },
            X11WindowManager::Enlightment => { track("enlightenment_start") // Might be `startx /usr/bin/enlightment_start` },
            X11WindowManager::Evilwm => { track("evilwm") },
            X11WindowManager::Fluxbox => { track("startfluxbox") },
            X11WindowManager::IceWM => { track("icewm") },
            X11WindowManager::Kwin => { track("kwin") },
            X11WindowManager::Metacity => { track("metacity") },
            X11WindowManager::Openbox => { track("openbox") },
            X11WindowManager::Dwm => { track("dwm") },
        }

        /*
         * How to implement the launcher of a Component [NOT ALL COMPONENTS NEEDS TO BE LAUCNHED]:
        if self._.is_some() {
            match self._.unwrap() {
                X11_::_ => { track("") },
            }
        }
        */
    }
}
