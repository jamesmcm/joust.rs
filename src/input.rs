use crate::log;
pub static mut KEYSTATE: (bool, bool, bool) = (false, false, false);
// Left, Right, Space
// TODO: May need Mutex

pub fn keyup(event: web_sys::KeyboardEvent) {
    log(&format!("keyup {}", event.key_code()));
    unsafe {
        match event.key_code() {
            37 => KEYSTATE.0 = false, // Left
            39 => KEYSTATE.1 = false, // Right
            32 => KEYSTATE.2 = false, // Space
            _ => {}
        }
    }
}

pub fn keydown(event: web_sys::KeyboardEvent) {
    log(&format!("keydown {}", event.key_code()));
    unsafe {
        match event.key_code() {
            37 => KEYSTATE.0 = true, // Left
            39 => KEYSTATE.1 = true, // Right
            32 => KEYSTATE.2 = true, // Space - TODO space should trigger on keydown
            _ => {}
        }
    }
}

pub fn left_pressed() -> bool {
    unsafe { KEYSTATE.0 }
}
pub fn right_pressed() -> bool {
    unsafe { KEYSTATE.1 }
}
pub fn space_pressed() -> bool {
    unsafe { KEYSTATE.2 }
}
