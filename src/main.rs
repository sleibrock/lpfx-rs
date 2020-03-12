extern crate portmidi;

//use portmidi;
use std::thread;
use std::time::Duration;

use portmidi::MidiMessage;

mod launchpad;
use launchpad::*;

mod colors;
use colors::*;

fn main() {

    // Start an instance of a Launchpad from a given ID 
    let mut launchpad = Launchpad::from_name("Launchpad MIDI 1")
	.expect("Failed to create a Launchpad instance");

    play(&mut launchpad)
}

fn play(mut lp: &mut Launchpad) {
    println!("Playing something");

    lp.leds_off();

    let values = [
	0, 1,  2,  3,  2,   1,
	0, 16, 17, 18, 19, 18, 17, 16,
	0, 32, 33, 34, 35, 34, 33, 32,
	0, 48, 49, 50, 51, 50, 49, 48,
    ];

    for &v in values.iter().cycle() {
	println!("color: {}", v);
	lp.color_all(v as u8);
	thread::sleep(Duration::from_millis(100));
    }
}



// end
