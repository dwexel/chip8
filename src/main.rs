// total 4096 bytes (000-FFF)
// 
// first 512 are empty (000-1FF)
// 
// except for the font



// only 12 bytes are required to address the 4096 bytes of memory
// but in practice, 16-bit numbers are used as addresses








// a nibble is half a byte (0-F)
// an instruction is two bytes and thus four nibbles (0000-FFFF)


// instructions are separated 
// into broad catergories based on their first nibble (half octet)

// which by the first we mean most significant
// so 
// here, we're using big endian
// IBM logo program is stored in BE





// the main loop should execute at around 700 instructions per second

// the timer should loop independently and should decrement at
// 60 times a second




// also
// 2-byte program counter
// 16 one-byte registers (V0-VF)

#![feature(macro_metavar_expr)]



use std::{fmt, io::{self, stdout, Read}, time::{Duration, SystemTime}};
use rand;



mod rewrite{
	use std::collections::{HashMap, HashSet};

	type Symbols = HashMap<String, usize>;


	// pub type PCCompiler = usize;
	// pub type Program = [u8; PROGRAM_LEN];




	pub const PROGRAM_START: usize = 0x200;


	pub const PROGRAM_LEN: usize = 0x200;
	pub const DATA_SECTION: usize = (PROGRAM_LEN / 2);

	

	#[derive(Debug)]
	pub struct State {
		pub symbols: Symbols,
		pub program: [u8; PROGRAM_LEN],
		pub pcc: usize,
	}

	impl State {
		pub fn new() -> Self {



			Self {
				symbols: Symbols::new(),
				program: [0; PROGRAM_LEN],
				pcc: DATA_SECTION,

			}


		}
	}
}




// todo... it should be able to take any types...
// like hex codes..
// 
// and work even if a full row isn't filled out...



macro_rules! data {
	($state:expr, $i:expr, $($e:expr)*) => {{

		// uses big endian bytes


		// set symbol here (name plus current location of pcc)


		$state.symbols.insert(
			String::from($i), 
			 
			rewrite::PROGRAM_START + $state.pcc

			);


		let bytes: [ bool; ${count($e)} ] = [
			$(
				($e != 0),
			)*
		];

		let mut byte: u8 = 0;

		for i in 0..bytes.len() {
			let ii = i % 8;

			set_bit_8(&mut byte, ii as u8, bytes[i]);

			print!("{i:?} \n");
		
			if ii == 7 {
				println!("add one byte to Read-only");

				// set program to pcc and increment pcc
				
				$state.program[
					$state.pcc
				] = byte;

				$state.pcc += 1;

				println!("starting next byte...");

				byte = 0;
			}
		}

		$state

	}};
}

// todo
// make number of rows optional
// todo
// let it use literal data
// or even a variable register


macro_rules! draw {
	($state:expr, $name:expr, $x:expr, $y:expr) => {
		// parameters: name, location, number of rows 

		// fetch data from symbols

		let s = $state.symbols;

		if let Some(addr) = s.get($name) {
			// set I
			// call draw

			// we have a problem

		}
	};
}



macro_rules! print_program {
	($state:expr) => {
		println!("{:?}", $state.1);
	};
}






const DB: bool = true;





macro_rules! db {
	($expression:expr) => {
		if DB {
			println!($expression);

		}
	};

	($expression:expr, $( $v:expr ),* ) => {
		if DB {
			println!($expression,  $($v,)* );
		}
	};
}



fn test_draw_display(d: &[u64; 32]) {
	for line_64 in d {
		let s = format!("{:066b}", line_64)
			.replace('0', " ")
			.replace('1', "#");


		println!("{}", s);
	}
}

fn test_print_registers(r: &[u8; 16]) {
	print!("\n");
	for i in 0..16 {
		if i % 4 == 0 { print!("\n"); }

		print!("V{:#x} {:#x} // ", i, r[i]);
	}
	print!("\n");
}

fn test_print_slice(s: &[u8]) {
	for byte in s {
		print!("{:#x}, ", byte);
	}
	print!("\n");
}

fn test_print_slice_as_u16(s: &[u8]) {
	let mut i = 0;
	loop {
		// 
		if i >= (s.len() - 1) { break; }

		let byte = u16::from_be_bytes([s[i], s[i+1]]);
		print!("{:04x}, ", byte);

		i += 2;
	}
	print!("\n");
}

struct Stack {
	stack: [u16; 16],
	pointer: usize
}

impl Stack {
	fn new() -> Self { Self { stack: [0; 16], pointer: 0 } }
	fn push(&mut self, value: u16) {
		self.stack[self.pointer] = value;
		self.pointer += 1;
	}
	fn pop(&mut self) -> u16 {
		self.pointer -= 1;
		self.stack[self.pointer]
	}
}


// bits 0-63
// 0 = most significant
// 63 = least
// if x > 63, panics

fn check_bit_64(row: u64, x: u8) -> bool {
	let shl = 63 - x;

	(row >> shl & 1_u64) == 1
}

fn set_bit_64(row: &mut u64, x: u8, v: bool) {
	let mask = 1_u64 << (63 - x);

	if v {
		*row |= mask;
	} else {
		*row &= !mask;
	}
}

// bits 0-7
// 0 = most significant
// 7 = least
// if x > 7, panics

fn check_bit_8(row: u8, x: u8) -> bool {
	let shl = 7 - x;

	(row >> shl & 1_u8) == 1
}

fn set_bit_8(row: &mut u8, x: u8, v: bool) {
	let mask = 1_u8 << (7 - x);

	if v {
		*row |= mask;
	} else {
		*row &= !mask;
	}
}


fn from_bit_array(bits: &[bool]) -> u8 {
	let mut ret = 0_u8;

	for i in 0..8 {
		// grass

		set_bit_8(&mut ret, i, bits.get(i as usize).expect("past the end").to_owned());
	}

	ret
}


// the PC is at a location
// load two bytes from there, then increment PC by two bytes

fn fetch(pc: &mut u16, chunk: &[u8; 4096]) -> u16 {
	let _pc = *pc as usize;

	let instruction = u16::from_be_bytes([chunk[_pc], chunk[_pc+1]]);

	*pc += 2;

	instruction
}


// decode and execute after fetching

fn decode(instruction: u16, d: &mut[u64; 32], v: &mut [u8; 16], i: &mut u16, pc: &mut u16, chunk: &mut [u8], stack: &mut Stack) {
	// byte order big endian here

	let first_nibble = (instruction >> 12) & 0x000F;

	let x = ((instruction >> 8) & 0x000F) as usize; // 4 bits
	let y = ((instruction >> 4) & 0x000F) as usize; // 4 bits

	let n = (instruction) & 0x000F; // 4 bits
	let nn = (instruction) & 0x00FF; // 8 bits
	let nnn = (instruction) & 0x0FFF; // 12 bits

	match first_nibble {
		0x0 => {
			match nnn {
				0x0EE => {
					print!("return from subroutine... ");

					*pc = stack.pop();
					print!("to {:#x}\n", pc);
				},

				0x0E0 => {
					db!("clear screen");

					*d = [0_u64; 32];
				},
				_ => {}
			}
		},
		0x1 => {
			db!("jump {:#x}", nnn);

			*pc = nnn;
		},
		0x2 => {
			db!("go to subroutine at {nnn:#x}");

			stack.push(*pc);

			*pc = nnn;
		},
		0x3 => {
			print!("skip? ");

			if (v[x] as u16) == nn {
				print!("yes\n");
				*pc += 2;                
			} else {
				print!("no\n");
			}
		},
		0x4 => {
			print!("skip? ");

			if (v[x] as u16) != nn {
				print!("yes\n");
				*pc += 2;                
			} else {
				print!("no\n");
			}
		}
		0x5 => {
			print!("skip? ");

			if v[x] == v[y] {
				print!("yes\n");
			} else {
				print!("no\n");
			}
		}

		0x6 => {
			db!("set register {:#x} to {:#x}", x, nn);

			let nn = nn.to_be_bytes()[1];

			v[x] = nn;
		}
		0x7 => {
			db!("add to register {:#x}, value {:#x}", x, nn);
			
			// nn is only a 1 byte number

			let nn = nn.to_be_bytes()[1];

			// panic stricken
			// v[x] += nn;

			if let Some(val) = v[x].checked_add(nn) {
				v[x] = val;
			}
		}

		0x8 => {
			match n {
				0x0 => {
					println!("set V{x:#x} to V{y:#x}");
					v[x] = v[y];
				}
				0x1 => {
					println!("bitwise OR");
					v[x] = v[x] | v[y];
				}
				0x2 => {
					println!("bitwise AND");
					v[x] = v[x] & v[y];
				}
				0x3 => {
					println!("bitwise XOR");
					v[x] = v[x] ^ v[y];

				}
				0x4 => {
					println!("set V{x:#x} += V{y:#x}");
					match v[x].checked_add(v[y]) {
						Some(val) => {
							v[x] = val;
							v[0xF] = 0;
						}
						None => {
							println!("integer overflow");
							v[0xF] = 1;
						}
					}   
				}
				0x5 => {
					println!("set V{x:#x} to V{x:#x} - V{y:#x}");
					match v[x].checked_sub(v[y]) {
						Some(val) => {
							v[x] = val;
							v[0xF] = 1;
						}
						None => {
							println!("integer underflow");
							v[0xF] = 0;
						}
					}
				}
				0x6 => {
					println!("bit shift V:{x:#x} right 1");
					// options

					if v[x] & 0b00000001 == 0b00000001 {
						print!("bit 1 was shifted out");
						v[0xF] = 1;
					} else {
						print!("bit 0 was shifted out");
						v[0xF] = 0;
					}

					v[x] >>= 1;
				}
				0x7 => {
					println!("set V{x:#x} to V{y:#x} - V{x:#x}");
					match v[y].checked_sub(v[x]) {
						Some(val) => {
							v[x] = val;
							v[0xF] = 1;
						}
						None => {
							println!("integer underflow");
							v[0xF] = 0;
						}
					}

				}
				0xE => {
					print!("bit shift V:{x:#x} left 1 ...");
					if v[x] & 0b10000000 == 0b10000000 {
						print!("bit 1 was shifted out");
						v[0xF] = 1;
					} else {
						print!("bit 0 was shifted out");
						v[0xF] = 0;
					}

					v[x] <<= 1;
				}
				_ => {},
			}
		}

		0x9 => {
			print!("skip? ");

			if v[x] != v[y] {
				print!("yes\n");
			} else {
				print!("no\n");
			}
		}

		0xA => {
			db!("set index register I, {:#x}", nnn);

			*i = nnn;
		}
		// 0xB => {

		// }
		0xC => {
			db!("generate random nummber");

			let r = rand::random::<u8>();

			//  ?
			v[x] = r & (nn as u8);
		}
		0xD => {

			db!("draw {n} rows I character at position x V{:#x} {:#x}, y V{:#x} {:#x}", x, v[x], y, v[y]);

			// get x and y values

			let mut x = v[x] % 64;
			let mut y = v[y] % 32;

			v[0xF] = 0;

			for _i in (*i)..(*i+n) {

				if y == 32 { break; }

				// get the font data

				let data_row: u8 = chunk[_i as usize];

				// x -> x +
				// left -> right
				// more significant bit -> less significant bit
				
				for ix in 0..8 {

					let _x = x + ix;
					if _x == 64 { break; }

					if check_bit_8(data_row, ix) {

						if check_bit_64(d[y as usize], _x) {
							
							db!("turn off pixel {_x}, {y} and set flag register to 1");
				
							set_bit_64(&mut d[y as usize], _x, false);
							
							v[0xF] = 1;
						} else {

							db!("turn on pixel {_x}, {y}");

							set_bit_64(&mut d[y as usize], _x, true);
						}
					}
				}

				y += 1;
			}
		}
		0xF => {

			// print!("retreive font address of character V{:#x} {:#x} ... ", x, v[x]);

			// let address = 5 * (v[x] as u16);

			// print!("{:#x}\n", address);

			// *i = address as u16;
		
			match nn {
				0x55 => {
					println!("retreive memory to V0 - V{x:#x}");

					let i = *i as usize;

					for vi in 0..=x {
						if i + vi >= chunk.len() {
							println!("tried to retreive memory from a location past the end");

							break;
						}

						v[vi] = chunk[i + vi];
					}


				}
				0x65 => {
					println!("store values registers V0 - V{x:#x} at index {i:#x}");

					let i = *i as usize;

					// for each register 
					// starting at 0
					// and going up to x

					for vi in 0..=x {
						// store the values starting at I

						if i + vi >= chunk.len() {
							println!("tried to store memory at a location past the end");
							break;
						}

						chunk[i + vi] = v[vi];
					}
				}
				_ => {}
			}
		}

		_ => panic!()
	}
}


fn main() {
	// initialize main chunk
	let mut chunk: [u8; 4096] = [0; 4096];

	chunk[ 0..5 ].copy_from_slice(&[0xF0, 0x90, 0x90, 0x90, 0xF0]); // 0
	chunk[ 5..10].copy_from_slice(&[0x20, 0x60, 0x20, 0x20, 0x70]); // 1
	chunk[10..15].copy_from_slice(&[0xF0, 0x10, 0xF0, 0x80, 0xF0]); // 2
	chunk[15..20].copy_from_slice(&[0xF0, 0x10, 0xF0, 0x10, 0xF0]); // 3
	chunk[20..25].copy_from_slice(&[0x90, 0x90, 0xF0, 0x10, 0x10]); // 4
	chunk[25..30].copy_from_slice(&[0xF0, 0x80, 0xF0, 0x10, 0xF0]); // 5
	chunk[30..35].copy_from_slice(&[0xF0, 0x80, 0xF0, 0x90, 0xF0]); // 6
	chunk[35..40].copy_from_slice(&[0xF0, 0x10, 0x20, 0x40, 0x40]); // 7
	chunk[40..45].copy_from_slice(&[0xF0, 0x90, 0xF0, 0x90, 0xF0]); // 8
	chunk[45..50].copy_from_slice(&[0xF0, 0x90, 0xF0, 0x10, 0xF0]); // 9
	chunk[50..55].copy_from_slice(&[0xF0, 0x90, 0xF0, 0x90, 0x90]); // A
	chunk[55..60].copy_from_slice(&[0xE0, 0x90, 0xE0, 0x90, 0xE0]); // B
	chunk[60..65].copy_from_slice(&[0xF0, 0x80, 0x80, 0x80, 0xF0]); // C
	chunk[65..70].copy_from_slice(&[0xE0, 0x90, 0x90, 0x90, 0xE0]); // D
	chunk[70..75].copy_from_slice(&[0xF0, 0x80, 0xF0, 0x80, 0xF0]); // E
	chunk[75..80].copy_from_slice(&[0xF0, 0x80, 0xF0, 0x80, 0x80]); // F


	let mut pc: u16 = 0x200;



	
	// how to check if input exists...

	// for (i, byte) in io::stdin().bytes().enumerate() {

	// 	let pc = pc as usize;
		
	// 	chunk[pc + i] = byte.unwrap();

	// }



	let mut stack = Stack::new();

	let mut display: [u64; 32] = [0; 32];

	let mut i_r: u16 = 0;

	let mut v0vf: [u8; 16] = [0; 16];

	// execution rate

	let seconds = Duration::from_secs(1 / 10);

	let mut start = SystemTime::now();

	let timer_seconds = Duration::from_secs(2);

	let mut timer_start = SystemTime::now();





	let mut rw = rewrite::State::new();



	rw = data!(

		rw,

		"mom",

		1 0 0 1 0 0 0 0
		1 1 1 1 1 1 1 1
		);


	println!("{rw:?}");

	



	// 00E0 (clear screen)
	// 1NNN (jump)
	// 6XNN (set register VX)
	// 7XNN (add value to register VX)
	// ANNN (set index register I)
	// DXYN (display/draw)

	// FX29 font character

	// 2NNN (enter subroutine)
	// 00EE (return from subroutine)

	// 3XNN (skip if register VX == NN)
	// 4XNN (skip if register VX != NN)
	// 5XY0 (skip if VX == VY)
	// 9XY0 (skip if VX != VY)

	// 8XY0 (set register VX to VY)
	// 8XY1 (set register VX to VX | VY)
	// 8XY2 (set register VX to VX & VY)
	// 8XY3 (set register VX to VX ^ VY)
	// 8XY4 (set register VX to VX + VY)

	// 8XY5 sets VX to the result of VX - VY.
	// 8XY7 sets VX to the result of VY - VX.

	// 8XY6 bit shift VX right
	// 8XYE bit shift VX left

	// FX55 store memory values of [V0-VX]
	// FX65 load memory values of [V0-VX]





	// decode(0x6010, &mut display, &mut v0vf, &mut i_r, &mut pc, &mut chunk, &mut stack);
	// decode(0x6109, &mut display, &mut v0vf, &mut i_r, &mut pc, &mut chunk, &mut stack);
	// decode(0x6208, &mut display, &mut v0vf, &mut i_r, &mut pc, &mut chunk, &mut stack);
	// decode(0x6307, &mut display, &mut v0vf, &mut i_r, &mut pc, &mut chunk, &mut stack);

	// decode(0xAFF0, &mut display, &mut v0vf, &mut i_r, &mut pc, &mut chunk, &mut stack);
	// decode(0xF365, &mut display, &mut v0vf, &mut i_r, &mut pc, &mut chunk, &mut stack);

	// decode(0x6000, &mut display, &mut v0vf, &mut i_r, &mut pc, &mut chunk, &mut stack);
	// decode(0x6100, &mut display, &mut v0vf, &mut i_r, &mut pc, &mut chunk, &mut stack);
	// decode(0x6200, &mut display, &mut v0vf, &mut i_r, &mut pc, &mut chunk, &mut stack);
	// decode(0x6300, &mut display, &mut v0vf, &mut i_r, &mut pc, &mut chunk, &mut stack);

	// test_print_registers(&v0vf);

	// decode(0xF355, &mut display, &mut v0vf, &mut i_r, &mut pc, &mut chunk, &mut stack);

	// test_print_registers(&v0vf);

	// test_print_slice(&chunk[(i_r as usize)..(i_r as usize)+4]);




	// loop {
	// 	match start.elapsed() {
	// 		Ok(elapsed) if elapsed > seconds => {
	// 			db!("execution now");

	// 			start = SystemTime::now();

	// 			let instruction = fetch(&mut pc, &chunk);

	// 			decode(instruction, &mut display, &mut v0vf, &mut i_r, &mut pc, &mut chunk, &mut stack);

	// 			test_draw_display(&display);
	// 		}
	// 		_ => (),
	// 	}
	// }


}



	// rust requires that slices are indexed with the system pointer
	// which makes sense
	// because they're elements that are next to eachother in system memory



	// https://stackoverflow.com/questions/34606043/how-do-i-replace-specific-characters-idiomatically-in-rust


// https://github.com/daniel5151/AC8E/tree/master
