// launchpad.rs
// rewrite from original to add support for Traits

use crate::traits::*;
use crate::devices::device::*;
use crate::aliases::*;

pub struct Launchpad {
    pub device: DeviceT,
}

impl Launchpad {
    pub fn new(name: &str) -> Result<Launchpad, String> {
	 match DeviceT::new(name) {
	    Err(e) => { return Err(e); }
	    Ok(d) => Ok(Launchpad {
		device: d,
	    })
	}
    }
}


impl Lightable for Launchpad {
    fn clear(&mut self) -> Err {
	self.device.write(0xB0, 0, 0)
    }

    fn led_on(&mut self, x: u8, y: u8, v: u8) -> Err {
	if x > 7 { return Err("x too large for grid".into()); }
	if y > 7 { return Err("y too large for grid".into()); }

	self.device.write(0x90, x + (y * 16), v)
    }

    fn led_off(&mut self, x: u8, y: u8) -> Err {
	if x > 7 { return Err("x too large for grid".into()); }
	if y > 7 { return Err("y too large for grid".into()); }

	self.device.write(0x80, x + (y * 16), 0)
    }

    fn note_on(&mut self, note: u8, vel: u8) -> Err {
	self.device.write(0x90, note, vel)
    }

    fn note_off(&mut self, note: u8) -> Err {
	self.device.write(0x80, note, 0)
    }
}




// end launchpad.rs
