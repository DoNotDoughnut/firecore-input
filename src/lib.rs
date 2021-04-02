use macroquad::prelude::KeyCode;
use serde::{Serialize, Deserialize};
use ahash::AHashSet as HashSet;
use ahash::AHashMap as HashMap;

pub mod keyboard;

pub mod touchscreen;

//pub mod controller;

pub type KeySet = HashSet<KeyCode>;
pub type KeyMap = HashMap<Control, HashSet<KeyCode>>;

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

pub fn pressed(control: Control) -> bool {
    if keyboard::pressed(&control) {
        return true;
    }
    if let Some(controls) = unsafe{touchscreen::TOUCHSCREEN.as_ref()} {
        if controls.pressed(&control) {
            return true;
        }
    }
    // if touchscreen::TOUCH_CONTROLS.pressed(&control) {
    //     return true;
    // }
    return false;
}

pub fn down(control: Control) -> bool {
    if keyboard::down(&control) {
        return true;
    }
    if let Some(controls) = unsafe{touchscreen::TOUCHSCREEN.as_ref()} {
        if controls.down(&control) {
            return true;
        }
    }
    return false;
}