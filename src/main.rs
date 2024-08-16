// total 4096 bytes (000-FFF)
// first 512 are empty (000-1FF)
// except for the font
// so the program starts at 0x200
// only 12 bytes are required to address the 4096 bytes of memory
// but in practice, 16-bit numbers are used as addresses


// a nibble is half a byte (0-F)
// an instruction is two bytes and thus four nibbles (0000-FFFF)
// instructions are separated into broad catergories based on their first nibble (half octet)
// which by the first we mean most significant



// all the chip8 programs I've seen so far have been stored in big endian
// meaning the most significant bit of a word () is earlier in the program 



// the main loop should execute at around 700 instructions per second
// the timer should loop independently and should decrement at
// 60 times a second

// also: 
// 2-byte program counter
// 16 one-byte registers (V0-VF)


#![allow(unused)]

use std::{env, fs::File, io::{self, Read}, process::exit, time::{Duration, SystemTime}};

mod rewrite;

const DB: bool = true;

macro_rules! db {
	($fmt_str:expr) => {
		if DB {
			println!($fmt_str);

		}
	};

	($fmt_str:expr, $( $v:expr ),* ) => {
		if DB {
			println!($fmt_str,  $($v,)* );
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
		if i >= (s.len() - 1) { break; }

		let byte = u16::from_be_bytes([s[i], s[i+1]]);
		print!("{:04x}, ", byte);

		i += 2;
	}
	print!("\n");
}


struct Stack {
	// in reality these are 16 bit numbers
	stack: [usize; 16],
	pointer: usize
}

impl Stack {
	fn new() -> Self { Self { stack: [0; 16], pointer: 0 } }

	fn push(&mut self, value: usize) {
		self.stack[self.pointer] = value;
		self.pointer += 1;
	}
	
	fn pop(&mut self) -> usize {
		// panic here ... pop empty stack ya
		self.pointer -= 1;
		self.stack[self.pointer]
	}
}


// bits 0-63
// 0 = most significant, 63 = least. if x > 63, panics

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
// 0 = most significant, 7 = least
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

fn fetch(pc: &mut usize, chunk: &[u8; 4096]) -> u16 {
	let instruction = u16::from_be_bytes(
		[chunk[*pc], chunk[*pc+1]]
	);

	*pc += 2;
	instruction
}

// decode and execute after fetching
fn decode(instruction: u16, d: &mut[u64; 32], v: &mut [u8; 16], i: &mut usize, pc: &mut usize, chunk: &mut [u8], stack: &mut Stack, t: &mut u8, st: &mut u8) {
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

			*pc = nnn as usize;
		},
		0x2 => {
			db!("go to subroutine at {nnn:#x}");

			stack.push(*pc);

			*pc = nnn as usize;
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
			print!("V{:#x} {:#x} not equal ", x, v[x]);
			print!("{:#x} ", nn);


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
				*pc += 2;
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
			
			let nn = nn.to_be_bytes()[1];

			v[x] = v[x].wrapping_add(nn);
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
					println!("bitwise AND V{:#x} {:#x} &= V{:#x} {:#x}", x, v[x], y, v[y]);
					v[x] = v[x] & v[y];

					// print!(" ({:#x})\n", v[x]);
				}
				0x3 => {
					println!("bitwise XOR");
					v[x] = v[x] ^ v[y];

				}
				0x4 => {
					println!("V{x:#x} += V{y:#x}");

					match v[x].checked_add(v[y]) {
						Some(val) => {
							v[x] = val;
							v[0xF] = 0;
						}
						None => {
							println!("integer overflow");

							// it's meant to wrap...
							v[x] = v[x].wrapping_add(v[y]);
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

							// it's meant to wrap...
							v[x] = v[x].wrapping_sub(v[y]);
							v[0xF] = 0;
						}
					}
				}
				0x6 => {
					println!("bit shift V:{x:#x} right 1");

					let f = if v[x] & 0b00000001 == 0b00000001 {
						print!("bit 1 was shifted out");
						// v[0xF] = 1;
						1
					} else {
						print!("bit 0 was shifted out");
						// v[0xF] = 0;
						0
					};

					v[x] >>= 1;
					v[0xF] = f;
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

							//
							v[x] = v[y].wrapping_sub(v[x]);
							v[0xF] = 0;
						}
					}

				}
				0xE => {
					print!("bit shift V:{x:#x} left 1 ...");

					let f = if v[x] & 0b10000000 == 0b10000000 {
						print!("bit 1 was shifted out");
						// v[0xF] = 1;
						1
					} else {
						print!("bit 0 was shifted out");
						// v[0xF] = 0;
						0
					};

					v[x] <<= 1;
					v[0xF] = f;

				}
				_ => {},
			}
		}

		0x9 => {
			print!("skip? ");

			if v[x] != v[y] {
				print!("yes\n");
				*pc += 2;
			} else {
				print!("no\n");
			}
		}

		0xA => {
			db!("set index register I, {:#x}", nnn);

			*i = nnn as usize;
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

// for <item> in chunk.iter().skip((*i)).take((n as usize)) {
			for _i in (*i)..(*i+(n as usize)) {

				if y == 32 { break; }

				// get the font data

				let data_row: u8 = chunk[_i];

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

		
			match nn {

				0x0A => {
					db!("wait for key...");

					db!("got key");
				}

				0x07 => {
					db!("set V{:#x} to value of timer,: {:#x}", x, t);

					v[x] = *t;
				}
				0x15 => {
					db!("set timer to V{:#x} {:#x}", x, v[x]);

					*t = v[x];
				}
				0x18 => {
					db!("set sound timer to V{:#x} {:#x}", x, v[x]);

					*st = v[x];
				}

				0x1E => {
					db!("index I plus equals V{:#x} {:#x}", x, v[x]);
					
					*i += (v[x] as usize);
				}

				0x29 => {
					print!("retreive font address of character V{:#x} {:#x} ... ", x, v[x]);

					let address = 5 * (v[x] as usize);

					println!("{:#x}", address);

					*i = address;
				}

				0x33 => {
					println!("binary-coded decimal conversion");

					let number = v[x];

					// number
					let hundders = (number) / 100;
					let tens = (number % 100) / 10;
					let ones = number % 10;

					println!("{number}, {hundders}, {tens}, {ones}");

					chunk[(*i)] = hundders;
					chunk[(*i) + 1] = tens;
					chunk[(*i) + 2] = ones;
				}
				0x55 => {
					println!("store values registers V0 - V{x:#x} at index {i:#x}");

					let i = *i;

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

				0x65 => {
					println!("retreive memory to V0 - V{x:#x}");
					let i = *i;
					for vi in 0..=x {
						if i + vi >= chunk.len() {
							println!("tried to retreive memory from a location past the end");
							break;
						}
						v[vi] = chunk[i + vi];
					}
				}
				_ => {}
			}
		}

		_ => panic!("{:#x}", instruction)
	}
}




fn main() {
	// initialize main chunk
	let mut chunk: [u8; 4096] = [0; 4096];

	// insert font data
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

	// in reality, is a 16 bit number
	let mut pc: usize = 0x200;



	let mut stack = Stack::new();
	let mut display: [u64; 32] = [0; 32];

	// also in reality a 16bit
	let mut i_r: usize = 0;
	let mut v0vf: [u8; 16] = [0; 16];

	// execution rate
	let e_rate = Duration::from_secs(1 / 700);
	let mut start = SystemTime::now();

	let t_rate = Duration::from_secs(1 / 60);
	let mut t_start = SystemTime::now();

	let mut timer: u8 = 0;
	let mut sound_timer: u8 = 0;


	use rewrite::*;


	let mut state = State::new();

	data(&mut state, "g", &[0b11110011, 0b11110011, 0b11110011, 0b11000011, 0b11000011, 0b11000011, 0b11000011, 0b11000011]);
	data_flipped(&mut state, "gg", Valued::Data("g".into()), Flip(true, true));

	assign(&mut state, Valued::from("m1"), Valued::Literal(5));
	assign(&mut state, Valued::from("m2"), Valued::Literal(5));	
	assign(&mut state, Valued::from("m2"), Valued::Expression(
		vec![
			(Valued::from(1), Ops::Add), (Valued::from(1), Ops::Add)
		]
	));
	
	// if_start(&mut State, )

		// draw(&mut state, Valued::from(0xA), Valued::from("m1"), Valued::from("m2"), Valued::from(5));

	// draw(&mut state, Valued::Data(String::from("g")), Valued::from("m2"), Valued::from("m1"), Valued::from(8));
	// draw(&mut state, Valued::Data(String::from("gg")), Valued::from(20), Valued::from(20), Valued::from(8));

	state.copy_program_to_memory(&mut chunk);


	// if_keydown

	test_print_slice_as_u16(&chunk[pc..pc+60]);


	if let Some(fname) = env::args().nth(1) {
		println!("The filename argument is {}", fname);

		let mut f = File::open(fname).expect("bad filename");
		if let Ok(l) = f.read(&mut chunk[pc..0xFFF]) {

		}
	}

	loop {
		match t_start.elapsed() {
			Ok(elapsed) if elapsed > t_rate => {
				t_start = SystemTime::now();

				if timer > 0 {
					timer -= 1;
					println!("timeout");
				}

				if sound_timer > 0 {
					timer -= 1;
					println!("sound timer timeout");
				}
			}
			_ => ()
		}

		match start.elapsed() {
			Ok(elapsed) if elapsed > e_rate => {
				db!("execution now");

				start = SystemTime::now();

				let instruction = fetch(&mut pc, &chunk);

				if instruction == 0_u16 {
					println!("sayonara!");
					pc -= 2;
					exit(0);
				}

				decode(instruction, &mut display, &mut v0vf, &mut i_r, &mut pc, &mut chunk, &mut stack, &mut timer, &mut sound_timer);
				// test_draw_display(&display);
				test_print_registers(&v0vf);
			}
			_ => ()
		}

	}
}

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
