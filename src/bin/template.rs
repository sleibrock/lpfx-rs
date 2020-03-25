// template.rs
// only to be used as a skeleton when making other programs

extern crate lpfx;

use lpfx::launchpad::*;
use lpfx::utils::*;


fn main() {

    // Start an instance of a Launchpad from a given ID 
    let mut launchpad = get_lp_from_name("Launchpad MIDI 1");

    play(&mut launchpad);
}

fn play(lp: &mut Launchpad) -> LPErr {
    lp.clear()?;


    loop {

    }

    Ok(())
}

// end template.rs 
