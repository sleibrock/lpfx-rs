// conway.rs
// game of life simulation with launchpad

extern crate lpfx;

use lpfx::prelude::*;


const ALIVE  : u8 = 1;
const DEAD   : u8 = 0;
const LBOUND : i16 = 0;
const RBOUND : i16 = 7;


// Store an 8x8 array of state
// Use quick initializers of [T; N]
pub struct Game {
    pub cells: [[u8; 8]; 8],
}

// Game state implementation for getters/updaters
impl Game {

    // Check if supplied (x,y) falls within our 8x8 grid
    // we use i32 types so we can freely subtract into negatives
    fn get_cell(&self, x: i16, y: i16) -> u8 {
	let fx = match (x < LBOUND, x > RBOUND) {
	    (true, false) => 7,
	    (false, true) => 0,
	    _ => x as usize,
	};

	let fy = match (y < LBOUND, y > RBOUND) {
	    (true, false) => 7,
	    (false, true) => 0,
	    _ => y as usize,
	};

	return self.cells[fy][fx];
    }

    fn count_neighbors(&self, x: i16, y: i16) -> (u8, u8) {
	let mut n : u8 = 0;

	// count all neighbors (by using addition since ALIVE=1, DEAD=0)
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
		    // checks if neighbors == 2 or 3
		    (ALIVE, 2) | (ALIVE, 3)
			=> { new_data[y as usize][x as usize] = ALIVE; },

		    // check if dead cells == 3
		    (DEAD, 3)
			=> { new_data[y as usize][x as usize] = ALIVE; },

		    // overpopulation/other rules falls here
		    _ => { new_data[y as usize][x as usize] = DEAD; },
		}
	    }
	}

	// mutate the &self state to the new array
	// We allocate a new matrix so we don't overrite our
	// current state frame while counting neighbors and mutating
	// otherwise would result in wrong state transitions
	self.cells = new_data;
    }
}


fn main() -> Err {

    // Start an instance of a Launchpad from a given ID 
    let mut launchpad = Launchpad::new("Launchpad MIDI 1")?;

    return play(&mut launchpad);
}

fn play<D: Grid>(lp: &mut D) -> Err {
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
		let v = world.get_cell(x as i16, y as i16);

		match v {
		    ALIVE => { lp.led_on(x, y, 3)?; },
		    _ => { lp.led_off(x, y)?; },
		}
	    }
	}

	world.update();
	sleep_millis(250);
    }
}

// end conway.rs
