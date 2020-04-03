// traits.rs

extern crate portmidi as pm;

use super::aliases::*;

pub trait MusicDevice {
    fn new(name: &str) -> Result<Box<Self>, String>;
}

pub trait Lightable {
    fn clear(&mut self) -> Err;
    fn led_on(&mut self, x: u8, y: u8, vel: u8) -> Err;
    fn led_off(&mut self, x: u8, y: u8) -> Err;
    fn note_on(&mut self, note: u8, vel: u8) -> Err;
    fn note_off(&mut self, note: u8) -> Err;
}

pub trait Grid : Lightable {
    fn clear(&mut self) -> Err;
    fn row_on(&mut self) -> Err;
    fn row_off(&mut self) -> Err;
    fn column_on(&mut self) -> Err;
    fn column_off(&mut self) -> Err;
}


// end traits.rs
