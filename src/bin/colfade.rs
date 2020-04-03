// colfade.rs

extern crate lpfx;

use lpfx::prelude::*;


fn main() -> Err {
    let mut lp = Launchpad::new("Launchpad MIDI 1")?;

    return play(&mut lp);
}

// Store an enumeration of eight values
pub enum EightV {
    Vals(u8, u8, u8, u8, u8, u8, u8, u8),
}


// Shift all values right over one place with a match expression
// This beats using any kind of array shifting loops/etc
fn shiftr(v: EightV) -> EightV {
    match v {
	EightV::Vals(a,b,c,d,e,f,g,h) => EightV::Vals(h,a,b,c,d,e,f,g),
    }
}


fn play<D: Grid>(lp: &mut D) -> Err {

    lp.clear()?;

    let mut v = EightV::Vals(0, 1, 2, 3, 2, 1, 0, 0);

    loop {
	match v {
	    // unpack the values every loop iteration and light LEDs
	    EightV::Vals(a,b,c,d,e,f,g,h) => {
		let v: Vec<u8> = vec![a,b,c,d,e,f,g,h];

		for i in 0..8 {
		    let vel = v[i as usize];
		    match vel {
			0 => { lp.column_off(i)?; },
			_ => { lp.column_on(i, vel)?; },
		    }
		}
	    }
	}

	sleep_millis(150);
	v = shiftr(v);
    }
}

// end colfade.rs
