// shiftyrows.rs

extern crate lpfx;

use lpfx::prelude::*;
use crate::Bounce::*; // shortcutting the enum

// default bounce count
const B : u8 = 6;

fn main() -> Err {
    let mut lp = Launchpad::new("Launchpad MIDI 1")?;
    return play(&mut lp);
}


// A more copmlicated version of Shifty Rows
// There are now 9 values
// The first value represents how many times we bounce
// When the first value hits zero, we switch it to it's opposite
#[derive(Clone)]
pub enum Bounce {
    Left(u8, u8, u8, u8, u8, u8, u8, u8, u8),
    Right(u8, u8, u8, u8, u8, u8, u8, u8, u8),
}


// We need 4 match rules for each type; n=0, n!=0
// When n == 0, flip right -> left and vice versa
// The direction we shift values over is based on the type
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


// Easily draw a row by referencing the Bounce type and the Launchpad
fn draw_row<D: Grid>(row: u8, lp: &mut D, r: &Bounce) -> Err {
    match *r {
	Left(_,a,b,c,d,e,f,g,h) => {
	    let values : Vec<u8> = vec![a,b,c,d,e,f,g,h];
	    for x in 0..8 {
		lp.led_on(x as u8, row, values[x as usize])?;
	    }
	},
	Right(_,a,b,c,d,e,f,g,h) => {
	    let values : Vec<u8> = vec![a,b,c,d,e,f,g,h];
	    for x in 0..8 {
		lp.led_on(x as u8, row, values[x as usize])?;
	    }
	},
    }
    Ok(())
}

// Play the whole thing now
fn play<D: Grid>(lp: &mut D) -> Err {

    // Store an initial array of Bounce types
    // Modify the left-most column to change time before impacts
    // Modify the 8 other columns to light the Launchpad
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

    // init clear
    lp.clear()?;

    loop {
	// iter through all rows and draw
	for r in 0..8 {
	    let bounce = &rows[r];
	    draw_row(r as u8, lp, bounce)?;
	    rows[r] = bounce.update();
	}

	// sleep before next cycle
	sleep_millis(200);
    }
}

// end bouncerows.rs
