// rowcol.rs

extern crate lpfx;

use std::thread;
use std::time::Duration;

use lpfx::launchpad::*;
use lpfx::colors::*;


fn main() {

    let mut lp = Launchpad::from_name("Launchpad MIDI 1")
	.expect("Failed to create a Launchpad instance");

    play(&mut lp);
}

fn play(mut lp: &mut Launchpad) {

    lp.clear();

    loop {
	
	for i in 0..8 {
	    lp.row_on(i, 3);
	    thread::sleep(Duration::from_millis(100));
	    lp.row_off(i);
	}

	for i in 0..8 {
	    lp.column_on(i, 3);
	    thread::sleep(Duration::from_millis(100));
	    lp.column_off(i);
	}

    }

}
