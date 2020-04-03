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


impl Grid for Launchpad {
    fn row_on(&mut self, row: u8, vel: u8) -> Err {
	if row > 7 { return Err("row too large for grid".into()); }

	for i in 0..8 {
	    self.device.write(0x90, (row * 16) + i, vel)?;
	}

	Ok(())
    }

    fn row_off(&mut self, row: u8) -> Err {
	if row > 7 { return Err("row too large for grid".into()); }

	for i in 0..8 {
	    self.device.write(0x80, (row * 16) + i, 0)?;
	}

	Ok(())
    }

    fn column_on(&mut self, col: u8, vel: u8) -> Err {
	if col > 7 { return Err("col too large for grid".into()); }

	for i in 0..8 {
	    self.device.write(0x90, col + (i * 16), vel)?;
	}

	Ok(())
    }

    fn column_off(&mut self, col: u8) -> Err {
	if col > 7 { return Err("col too large for grid".into()); }

	for i in 0..8 {
	    self.device.write(0x80, col + (i * 16), 0)?;
	}

	Ok(())
    }

    fn color_all(&mut self, vel: u8) -> Err {
	for _ in 0..39 {
	    self.device.write(0x92, vel, vel)?;
	}
	self.device.write(0xB0, 1, 1)?;
	Ok(())
    }
}



// end launchpad.rs
