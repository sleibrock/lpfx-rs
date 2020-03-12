// launchpad.rs

extern crate portmidi as pm;

use crate::colors::*;


pub struct Launchpad {
    pub input:  pm::InputPort,
    pub output: pm::OutputPort,
    pub midi:   pm::PortMidi,
}


impl Launchpad {

    // not implemented yet
    //pub fn get_launchpads() -> Vec<Launchpad> {
	//return vec![];
    //}

    pub fn from_name(target: &str) -> Result<Launchpad, &str> {
	let midi = pm::PortMidi::new()
	    .expect("Failed to create PortMidi instance");
	let all_devices = midi.devices()
	    .expect("Failed to get all devices");

	let mut output_id : Option<i32> = None;
	let mut input_id : Option<i32> = None;

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
	    let mut out_port = midi
		.device(output_id.expect("Failed to get Output ID"))
		.and_then(|dev| midi.output_port(dev, 1024))
		.expect("Failed to open an output port");

	    let mut in_port = midi
		.device(input_id.expect("Failed to get an Input ID"))
		.and_then(|dev| midi.input_port(dev, 1024))
		.expect("Failed to open an input port");

	    return  Ok(Launchpad {
		input: in_port,
		output: out_port,
		midi: midi,
	    });
	}

	return Err("Failed to create Launchpad");
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
    pub fn write(&mut self, mtype: u8, note: u8, vel: u8) -> Result<(), &str> {
	let v = (*self).output.write_message(pm::MidiMessage {
	    status: mtype,
	    data1: note,
	    data2: vel,
	});

	return match v {
	    Ok(_) => Ok(()),
	    _  => Err("Failed to write message"),
	};
    }

    pub fn leds_off(&mut self) -> Result<(), &str> {
	return self.write(0xB0, 0, 0);
    }


    pub fn led_on(&mut self, x: u8, y: u8, v: u8) -> Result<(), &str> {
	if x > 7 { return Err("x too large for grid"); }
	if y > 7 { return Err("y too large for grid"); }

	return self.write(0x90, x + (y * 8), v);
    }

    pub fn led_off(&mut self, x: u8, y: u8) -> Result<(), &str> {
	if x > 7 { return Err("x too large for grid"); }
	if y > 7 { return Err("y too large for grid"); }

	return self.write(0x80, 0, 0);
    }

    pub fn note_on(&mut self, note: u8, vel: u8) -> Result<(), &str> {
	return self.write(0x90, note, vel);
    }

    pub fn note_off(&mut self, note: u8) -> Result<(), &str> {
	return self.write(0x80, note, 0);
    }


    pub fn color_all(&mut self, vel: u8) -> Result<(), &str> {

	// we need to execute a message 40 times
	for i in 0..39 {
	    self.write(0x92, vel, vel);
	}
	self.write(0xB0, 1, 1);

	return Ok(());
    }

}
