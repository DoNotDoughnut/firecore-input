use serde::{Serialize, Deserialize};
use ahash::{AHashMap as HashMap, AHashSet as HashSet};

pub mod keyboard;
// pub mod touchscreen;
//pub mod controller;
#[cfg(feature = "input")]
pub type KeySet = HashSet<macroquad::prelude::KeyCode>;

pub type KeySetSerializable = HashSet<keyboard::serialization::KeySerializable>;

#[cfg(feature = "input")]
pub type KeyMap = HashMap<Control, HashSet<macroquad::prelude::KeyCode>>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum Control {

    A,
    B,
    Up,
    Down,
    Left,
    Right,
    Start,
    Select,
    //Escape,

}

#[cfg(feature = "input")]
pub fn pressed(control: Control) -> bool {
    if keyboard::pressed(&control) {
        return true;
    }
    // if touchscreen::TOUCH_CONTROLS.pressed(&control) {
    //     return true;
    // }
    return false;
}

#[cfg(feature = "input")]
pub fn down(control: Control) -> bool {
    if keyboard::down(&control) {
        return true;
    }
    // if touchscreen::TOUCH_CONTROLS.down(&control) {
    //     return true;
    // }
    return false;
}

#[cfg(feature = "input")]
pub fn load(key_map: KeyMap) {
    unsafe { keyboard::KEY_CONTROLS = Some(key_map); }
}