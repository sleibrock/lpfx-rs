// template.rs
// only to be used as a skeleton when making other programs

extern crate lpfx;

use lpfx::prelude::*;


fn main() -> Err {

    // Start an instance of a Launchpad from a given ID 
    let mut launchpad = Launchpad::new("Launchpad MIDI 1")?;

    return play(&mut launchpad);
}

fn play<D: Lightable>(lp: &mut D) -> Err {
    lp.clear()?;

    loop {

    }
}

// end template.rs 
