// conway.rs
// game of life simulation with launchpad

extern crate lpfx;

use lpfx::launchpad::*;
use lpfx::utils::*;

const ALIVE  : u8 = 1;
const DEAD   : u8 = 0;
const ROWMOD : u8 = 8;
const GAP : u8 = 7;

pub struct Game {
    pub cells: [[u8; 8]; 8],
}

impl Game {
    fn get_cell(&self, x: i32, y: i32) -> u8 {
	let fx : i32 = match (x < 0, x > 7) {
	    (true, false) => 7,
	    (false, true) => 0,
	    _ => x,
	};

	let fy : i32 = match (y < 0, y > 7) {
	    (true, false) => 7,
	    (false, true) => 0,
	    _ => y,
	};

	return self.cells[fy as usize][fx as usize];
    }

    fn count_neighbors(&self, x: i32, y: i32) -> (u8, u8) {
	let mut n : u8 = 0;

	n += self.get_cell(x-1, y);
	n += self.get_cell(x+1, y);
	n += self.get_cell(x, y-1);
	n += self.get_cell(x, y+1);
	n += self.get_cell(x-1, y-1);
	n += self.get_cell(x-1, y+1);
	n += self.get_cell(x+1, y-1);
	n += self.get_cell(x+1, y+1);

	return (self.get_cell(x,y), n);
    }
    
    fn update(&mut self) {
	let mut new_data : [[u8; 8]; 8] = [[0; 8]; 8];

	for y in 0..8 {
	    for x in 0..8 {
		let (v, c) = self.count_neighbors(x, y);

		match (v, c) {
		    (ALIVE, 2) | (ALIVE, 3)
			=> { new_data[y as usize][x as usize] = ALIVE; },
		    (DEAD, 3)
			=> { new_data[y as usize][x as usize] = ALIVE; },
		    _ => { new_data[y as usize][x as usize] = DEAD; },
		}
	    }
	}

	self.cells = new_data;
    }
}


fn main() {

    // Start an instance of a Launchpad from a given ID 
    let mut launchpad = get_lp_from_name("Launchpad MIDI 1");

    play(&mut launchpad);
}

fn play(lp: &mut Launchpad) -> LPErr {
    lp.clear()?;

    let mut world = Game {cells: [
	[0, 0, 0, 0, 0, 0, 0, 0],
	[0, 0, 0, 1, 0, 0, 0, 0],
	[0, 0, 0, 0, 1, 0, 0, 0],
	[0, 0, 1, 1, 1, 0, 0, 0],
	[0, 0, 0, 0, 0, 0, 0, 0],
	[0, 0, 1, 0, 0, 0, 0, 0],
	[0, 1, 0, 1, 0, 0, 1, 1],
	[0, 0, 1, 0, 1, 0, 1, 1],
    ]};

    loop {
	for y in 0..8 {
	    for x in 0..8 {
		let v = world.get_cell(x as i32, y as i32);

		match v {
		    ALIVE => { lp.led_on(x, y, 3)?; },
		    _ => { lp.led_off(x, y)?; },
		}
	    }
	}

	world.update();
	sleep_millis(250);
    }

    Ok(())
}

// end conway.rs
