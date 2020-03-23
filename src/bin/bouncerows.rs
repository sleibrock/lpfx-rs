// shiftyrows.rs

extern crate lpfx;

use lpfx::launchpad::*;
use lpfx::utils::*;
use crate::Bounce::*;

const B : u8 = 6;

fn main() {
    let mut lp = get_lp_from_name("Launchpad MIDI 1");

    play(&mut lp);
}

#[derive(Clone)]
pub enum Bounce {
    Left(u8, u8, u8, u8, u8, u8, u8, u8, u8),
    Right(u8, u8, u8, u8, u8, u8, u8, u8, u8),
}


impl Bounce {
    pub fn update(&self) -> Bounce {
	match *self {
	    Left(0,a,b,c,d,e,f,g,h)  => Right(B,h,a,b,c,d,e,f,g),
	    Right(0,a,b,c,d,e,f,g,h) => Left(B,b,c,d,e,f,g,h,a),
	    Left(n,a,b,c,d,e,f,g,h)  => Left(n-1,b,c,d,e,f,g,h,a),
	    Right(n,a,b,c,d,e,f,g,h) => Right(n-1,h,a,b,c,d,e,f,g),
	}
    }
}

fn draw_row(row: u8, lp: &mut Launchpad, r: &Bounce) {
    match *r {
	Left(_,a,b,c,d,e,f,g,h) => {
	    let values : Vec<u8> = vec![a,b,c,d,e,f,g,h];
	    for x in 0..8 {
		lp.led_on(x as u8, row, values[x as usize]);
	    }
	},
	Right(n,a,b,c,d,e,f,g,h) => {
	    let values : Vec<u8> = vec![a,b,c,d,e,f,g,h];
	    for x in 0..8 {
		lp.led_on(x as u8, row, values[x as usize]);
	    }
	},
    }

}


fn play(lp: &mut Launchpad) -> LPErr {

    let mut rows : Vec<Bounce> = vec![
	Left(3, 0,0,0,3,0,0,0,0),
	Left(2, 0,0,3,0,0,0,0,0),
	Left(1, 0,3,0,0,0,0,0,0),
	Left(2, 0,0,3,0,0,0,0,0),
	Left(3, 0,0,0,3,0,0,0,0),
	Left(4, 0,0,0,0,3,0,0,0),
	Left(5, 0,0,0,0,0,3,0,0),
	Left(6, 0,0,0,0,0,0,3,0),
    ];

    lp.clear()?;
    loop {

	for r in 0..8 {
	    let bounce = &rows[r];

	    draw_row(r as u8, lp, bounce);

	    rows[r] = bounce.update();
	}

	sleep_millis(200);
    }

    Ok(())
}

// end shiftyrows.rs
