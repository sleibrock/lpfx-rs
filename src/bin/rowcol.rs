// rowcol.rs

extern crate lpfx;

use lpfx::prelude::*;

fn main() -> Err {
    let mut lp = Launchpad::new("Launchpad MIDI 1")?;
    return play(&mut lp);
}

fn play<D: Grid>(lp: &mut D) -> Err {

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
