extern crate lpfx;

use lpfx::launchpad::*;
use lpfx::utils::*;


fn main() {

    // Start an instance of a Launchpad from a given ID 
    let mut launchpad = get_lp_from_name("Launchpad MIDI 1");

    play(&mut launchpad);
}

fn play(lp: &mut Launchpad) -> LPErr {
    println!("Playing something");

    lp.clear()?;

    let values = [
	0, 1,  2,  3,  2,   1,
	0, 16, 17, 18, 19, 18, 17, 16,
	0, 32, 33, 34, 35, 34, 33, 32,
	0, 48, 49, 50, 51, 50, 49, 48,
    ];

    for &v in values.iter().cycle() {
	println!("color: {}", v);
	lp.color_all(v as u8)?;
	sleep_millis(150);
    }

    Ok(())
}

// end allglow.rs
