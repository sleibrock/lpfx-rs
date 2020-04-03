extern crate lpfx;

use lpfx::prelude::*;


fn main() -> Err {

    // Start an instance of a Launchpad from a given ID 
    let mut launchpad = Launchpad::new("Launchpad MIDI 1")?;
    return play(&mut launchpad);
}

fn play<D: Grid>(lp: &mut D) -> Err {
    lp.clear()?;

    let values = [
	0, 1,  2,  3,  2,   1,
	0, 16, 17, 18, 19, 18, 17, 16,
	0, 32, 33, 34, 35, 34, 33, 32,
	0, 48, 49, 50, 51, 50, 49, 48,
    ];

    for &v in values.iter().cycle() {
	lp.color_all(v as u8)?;
	sleep_millis(150);
    }

    // a return is needed here because of no infinite loop
    Ok(())
}

// end allglow.rs
