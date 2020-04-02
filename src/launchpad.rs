// launchpad.rs

extern crate portmidi as pm;


/// Layout of the Launchpad
pub struct Launchpad {
    pub input:  pm::InputPort,
    pub output: pm::OutputPort,
    pub midi:   pm::PortMidi,
}

pub type LPErr = Result<(), String>;


pub fn get_lp_from_name(target: &str) -> Launchpad {
    return Launchpad::from_name(target)
	.expect("Failed to create a Launchpad instance");
}

impl Launchpad {

    // not implemented yet
    //pub fn get_launchpads() -> Vec<Launchpad> {
	//return vec![];
    //}


    /// Create a Launchpad instance from a given name
    ///
    /// ```
    /// let lp = Launchpad::from_name("Launchpad MIDI 1")?;
    /// ```
    pub fn from_name(target: &str) -> Result<Launchpad, String> {
	let midi = pm::PortMidi::new()
	    .expect("Failed to create PortMidi instance");
	let all_devices = midi.devices()
	    .expect("Failed to get all devices");

	let mut output_id : Option<i32> = None;
	let mut input_id  : Option<i32> = None;

	for device in all_devices {
	    println!("Device: {}, id: {}", device.name(), device.id());

	    if device.name().to_owned() == target {
		if device.is_output() {
		    output_id = Some(device.id() as i32);
		}

		if device.is_input() {
		    input_id = Some(device.id() as i32);
		}
	    }
	}

	if output_id.is_some() && input_id.is_some() {
	    let out_port = midi
		.device(output_id.expect("Failed to get Output ID"))
		.and_then(|dev| midi.output_port(dev, 1024))
		.expect("Failed to open an output port".into());

	    let in_port = midi
		.device(input_id.expect("Failed to get an Input ID"))
		.and_then(|dev| midi.input_port(dev, 1024))
		.expect("Failed to open an input port".into());

	    return  Ok(Launchpad {
		input: in_port,
		output: out_port,
		midi: midi,
	    });
	}

	return Err("Failed to create Launchpad".into());
    }


    /// Write a message in the format according to the Launchpad user manual.
    ///
    /// Message type       Hex format       Decimal Format
    /// Note off           80h, key, vel    128, key, vel
    /// Note on            90h, key, vel    144, key, vel
    /// Controller change  B0h, key, vel    176, Controller, data
    ///
    /// Controller change is usually meant to controller specific on-board
    /// settings or options, or are used for quick-setting/clearing LEDs.
    ///
    /// This function serves as the sub-function used by many convenience
    /// functions, and all Launchpads with a Launchpad-like Trait should
    /// have this method to build an API on top of.
    ///
    /// ```
    /// lp.write(0x80, 37, 33)?;
    /// ```
    pub fn write(&mut self, mtype: u8, note: u8, vel: u8) -> LPErr {
	let v = (*self).output.write_message(pm::MidiMessage {
	    status: mtype,
	    data1: note,
	    data2: vel,
	});

	return match v {
	    Ok(_) => Ok(()),
	    _  => Err("Failed to write message".into()),
	};
    }

    /// Clear all the LED statuses
    pub fn clear(&mut self) -> LPErr {
	self.write(0xB0, 0, 0)
    }


    /// Turn a specific LED on
    ///
    /// ```
    /// lp.led_on(3, 4)?;
    /// ```
    pub fn led_on(&mut self, x: u8, y: u8, v: u8) -> LPErr {
	if x > 7 { return Err("x too large for grid".into()); }
	if y > 7 { return Err("y too large for grid".into()); }

	self.write(0x90, x + (y * 16), v)
    }


    /// Turn off a specific LED
    ///
    /// ```
    /// lp.led_off(3, 4)?;
    /// ```
    pub fn led_off(&mut self, x: u8, y: u8) -> LPErr {
	if x > 7 { return Err("x too large for grid".into()); }
	if y > 7 { return Err("y too large for grid".into()); }

	self.write(0x80, x + (y * 16), 0)
    }


    /// Set a note's status on with a given velocity
    ///
    /// ```
    /// lp.note_on(33, 51)?;
    /// ```
    pub fn note_on(&mut self, note: u8, vel: u8) -> LPErr {
	self.write(0x90, note, vel)
    }


    /// Set a note's status to off
    ///
    /// ```
    /// lp.note_off(33)?;
    /// ```
    pub fn note_off(&mut self, note: u8) -> LPErr {
	self.write(0x80, note, 0)
    }


    /// Turn a row on with a given velocity
    ///
    /// ```
    /// lp.row_on(3, 34)?;
    /// ```
    pub fn row_on(&mut self, row: u8, vel: u8) -> LPErr {
	if row > 7 { return Err("row too large for grid".into()); }

	for i in 0..8 {
	    self.write(0x90, (row*16) + i, vel)?;
	}

	Ok(())
    }

    
    /// Turn a row off
    ///
    /// ```
    /// lp.row_off(3)?;
    /// ```
    pub fn row_off(&mut self, row: u8) -> LPErr {
	if row > 7 { return Err("row too large for grid".into()); }

	for i in 0..8 {
	    self.write(0x80, (row*16) + i, 0)?;
	}

	Ok(())
    }


    /// Turn a column on with a given velocity
    ///
    /// ```
    /// lp.column_on(4, 19)?;
    /// ```
    pub fn column_on(&mut self, col: u8, vel: u8) -> LPErr {
	if col > 7 { return Err("col too large for grid".into()); }

	for i in 0..8 {
	    self.write(0x90, col + (i*16), vel)?;
	}

	Ok(())
    }
    

    /// Turn a column off
    ///
    /// ```
    /// lp.column_off(4)?;
    /// ```
    pub fn column_off(&mut self, col: u8) -> LPErr {
	if col > 7 { return Err("col too large for grid".into()); }

	for i in 0..8 {
	    self.write(0x80, col + (i*16), 0)?;
	}

	Ok(())
    }


    pub fn color_all(&mut self, vel: u8) -> LPErr {

	// we need to execute a message 40 times
	for _ in 0..39 {
	    self.write(0x92, vel, vel)?;
	}
	self.write(0xB0, 1, 1)?;

	return Ok(());
    }

}


// end launchpad.rs

