// rowcol.rs

extern crate lpfx;

use lpfx::launchpad::*;
use lpfx::utils::*;


fn main() -> LPErr {
    let mut lp = get_lp_from_name("Launchpad MIDI 1");
    play(&mut lp)?;
    Ok(())
}

fn play(lp: &mut Launchpad) -> LPErr {

    lp.clear()?;

    loop {
	
	for i in 0..8 {
	    lp.row_on(i, 3)?;
	    sleep_millis(100);
	    lp.row_off(i)?;
	}

	for i in 0..8 {
	    lp.column_on(i, 3)?;
	    sleep_millis(100);
	    lp.column_off(i)?;
	}
    }
}

// end rowcol.rs
