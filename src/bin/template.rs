// template.rs
// only to be used as a skeleton when making other programs

extern crate lpfx;

use lpfx::launchpad::*;


fn main() -> LPErr {

    // Start an instance of a Launchpad from a given ID 
    let mut launchpad = get_lp_from_name("Target Device Name");

    play(&mut launchpad)?;

    Ok(())
}

fn play(lp: &mut Launchpad) -> LPErr {
    lp.clear()?;


    loop {

    }
}

// end template.rs 
