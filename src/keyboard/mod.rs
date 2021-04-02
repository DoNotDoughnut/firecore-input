use crate::KeySetSerializable;

use self::serialization::KeySerializable;

use ahash::{
    AHashMap as HashMap,
    AHashSet as HashSet,
};

use super::Control;

pub mod serialization;

#[cfg(feature = "input")]
pub static mut KEY_CONTROLS: Option<crate::KeyMap> = None;

#[cfg(feature = "input")]
pub fn load(key_map: crate::KeyMap) {
    unsafe { KEY_CONTROLS = Some(key_map); }
}

#[cfg(feature = "input")]
pub fn pressed(control: &Control) -> bool {
    if let Some(keys) = unsafe{KEY_CONTROLS.as_ref().unwrap().get(control)} {
        for key in keys {
            if macroquad::prelude::is_key_pressed(*key) {
                return true;
            }
        }
    }
    return false;
}

#[cfg(feature = "input")]
pub fn down(control: &Control) -> bool {
    if let Some(keys) = unsafe{KEY_CONTROLS.as_ref().unwrap().get(&control)} {
        for key in keys {
            if macroquad::prelude::is_key_down(*key) {
                return true;
            }
        }
    }
    return false;
}

pub fn default() -> HashMap<Control, KeySetSerializable> {

    #[cfg(not(feature = "input"))]
    use serialization::KeyCodeDef as KeyCode;

    #[cfg(feature = "input")]
    use macroquad::prelude::KeyCode;

    let mut controls = HashMap::new();
    controls.insert(Control::A, keyset(&[KeyCode::X]));
    controls.insert(Control::B, keyset(&[KeyCode::Z]));
    controls.insert(Control::Up, keyset(&[KeyCode::Up]));
    controls.insert(Control::Down, keyset(&[KeyCode::Down]));
    controls.insert(Control::Left, keyset(&[KeyCode::Left]));
    controls.insert(Control::Right, keyset(&[KeyCode::Right]));
    controls.insert(Control::Start, keyset(&[KeyCode::A]));
    controls.insert(Control::Select, keyset(&[KeyCode::S]));
    controls
}

fn keyset(
    #[cfg(not(feature = "input"))]
    codes: &[serialization::KeyCodeDef],
    #[cfg(feature = "input")]
    codes: &[macroquad::prelude::KeyCode]
) -> KeySetSerializable {
    let mut set = HashSet::new();
    for code in codes {
        set.insert(KeySerializable::new(*code));
    }    
    return set;
}