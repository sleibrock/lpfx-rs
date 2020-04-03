// shiftyrows.rs

extern crate lpfx;

use lpfx::prelude::*;

fn main() -> Err{
    let mut lp = Launchpad::new("Launchpad MIDI 1")?;
    return play(&mut lp);
}

#[derive(Clone)]
pub enum EightV {
    Vals(u8, u8, u8, u8, u8, u8, u8, u8),
}

fn shiftr(v: EightV) -> EightV {
    match v {
	EightV::Vals(a,b,c,d,e,f,g,h) => EightV::Vals(h,a,b,c,d,e,f,g),
    }
}

fn play<D: Grid>(lp: &mut D) -> Err {

    lp.clear()?;

    let mut v = EightV::Vals(34, 51, 34, 0, 0, 34, 51, 34);

    loop {

	for r in 0..8 {
	    match v {
		EightV::Vals(a,b,c,d,e,f,g,h) => {
		    let v: Vec<u8> = vec![a,b,c,d,e,f,g,h];
		    
		    for i in 0..8 {
			let vel = v[i as usize];
			match vel {
			    0 => { lp.led_off(i, r)?; },
			    _ => { lp.led_on(i, r, vel)?; },
			}
		    }
		},
	    }
	    v = shiftr(v);
	}

	sleep_millis(200);
	v = shiftr(v);
    }
}

// end shiftyrows.rs
