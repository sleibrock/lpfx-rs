// device.rs

extern crate portmidi as pm;

use crate::aliases::*;

pub struct DeviceT {
    pub input: pm::InputPort,
    pub output: pm::OutputPort,
    pub midi: pm::PortMidi,
}

impl DeviceT {
    pub fn new(name: &str) -> Result<DeviceT, String> {
	let midi = pm::PortMidi::new()
	    .expect("Failed to create PortMidi instance");
	let all_devices = midi.devices()
	    .expect("Failed to get all devices");

	let mut output_id : Option<i32> = None;
	let mut input_id  : Option<i32> = None;

	for device in all_devices {
	    println!("Device: {}, id: {}", device.name(), device.id());

	    if device.name().to_owned() == name {
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

	    return Ok(DeviceT {
		input: in_port,
		output: out_port,
		midi: midi,
	    });
	}
	Err("Failed to create device instance".into())
    }

    pub fn write(&mut self, mtype: u8, data: u8, vel: u8) -> Err {
	let v = (*self).output.write_message(pm::MidiMessage {
	    status: mtype,
	    data1: data,
	    data2: vel,
	});
	
	return match v {
	    Ok(_) => Ok(()),
	    _  => Err("Failed to write message".into()),
	};
    }
}

// end device.rs
