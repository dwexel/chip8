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

#![allow(unused)]


use std::{env, fs::File, io::{self, Read}, process::exit, time::{Duration, SystemTime}};
use rand;



mod rewrite{
	use std::collections::HashMap;

	type Symbols = HashMap<String, usize>;
	type Assignments = [bool; 16];



	pub const PROGRAM_LEN: usize = 0xFFF - 0x200; 
	pub const DATA_SECTION: usize = (PROGRAM_LEN / 2);

	

	#[derive(Debug)]
	pub struct State {
		pub symbols: Symbols,
		pub program: [u8; PROGRAM_LEN],
		pub pcc: usize,

		pub assignments: Assignments,
	}

	impl State {
		pub fn new() -> Self {
			Self {
				symbols: Symbols::new(),
				program: [0; PROGRAM_LEN],
				pcc: 0,
				assignments: [false; 16]
			}
		}

		pub fn byte_push(&mut self, b: u8) {
			self.program[self.pcc] = b;
			self.pcc += 1;
		}
	}
}




// todo... it should be able to take any types...
// like hex codes..
// 
// and work even if a full row isn't filled out...



macro_rules! data {
	($state:expr, $i:expr, $($e:expr)*) => {
		
		// find the next empty placce in data

		let _pcc = rewrite::DATA_SECTION;

		while $state.program[_pcc] != 0 {
			_pcc += 1;
		}

		$state.symbols.insert(
			String::from($i), 
			_pcc
			);

		let bytes: [ u8; ${count($e)} ] = [ $( $e, )* ];

		for b in bytes {
			$state.program[_pcc] = b;
			_pcc += 1;
		}
	};
}


// todo
// make number of rows optional


// let it use a font data
// 
// or "data"
//

// yeah macros maybe not cutting it?
// proc macros?

macro_rules! draw_data {
	($state:expr, $name:expr, $x:expr, $y:expr, $rows:expr) => {



		match $state.symbols.get($name) {
			Some(addr) => {


				$state.assignments[0x01] = true;
				$state.assignments[0x02] = true;

				
				println!("draw data charater named {} at x {:#x}, y {:#x}, rows {:#x}", $name, $x, $y, $rows);


				// ANNN
				let nnn = addr;

				// 6XNN
				let x = 0x01;
				let nn = $x as u8;


				b = 0x06 << 4;
				b = b | x;

				$state.byte_push(b);

				b = nn;

				$state.byte_push(b);

				// 6XNN
				let x = 0x02;
				let nn = $y as u8;


				b = 0x06 << 4;
				b = b | x;

				$state.byte_push(b);

				b = nn;

				$state.byte_push(b);


				// DXYN
				let x = 0x01;
				let y = 0x02;
				let n = $rows as u8;

				b = 0x0D << 4;
				b = b | x;

				$state.byte_push(b);

				b = y << 4;
				b = b | n;

				$state.byte_push(b);
			
				$state.assignments[0x01] = false;
				$state.assignments[0x02] = false;



			},
			None => panic!(),
		}
	}
}

macro_rules! draw {
	($state:expr, $name:expr, $x:expr, $y:expr, $rows:expr) => {


		$state.assignments[0x01] = true;
		$state.assignments[0x02] = true;

		
		println!("draw font charater {:#x} at x {:#x}, y {:#x}, rows {:#x}", $name, $x, $y, $rows);

		// 6XNN
		// FX29
		let nn = $name as u8;
		let x = 0x01;		

		let mut b: u8 = 0x06 << 4;
		b = b | x;

		$state.byte_push(b);

		b = nn;

		$state.byte_push(b);

		b = 0x0F << 4;
		b = b | x;

		$state.byte_push(b);

		b = 0x29;

		$state.byte_push(b);

		// 6XNN
		let x = 0x01;
		let nn = $x as u8;


		b = 0x06 << 4;
		b = b | x;

		$state.byte_push(b);

		b = nn;

		$state.byte_push(b);

		// 6XNN
		let x = 0x02;
		let nn = $y as u8;


		b = 0x06 << 4;
		b = b | x;

		$state.byte_push(b);

		b = nn;

		$state.byte_push(b);


		// DXYN
		let x = 0x01;
		let y = 0x02;
		let n = $rows as u8;

		b = 0x0D << 4;
		b = b | x;

		$state.byte_push(b);

		b = y << 4;
		b = b | n;

		$state.byte_push(b);
	
		$state.assignments[0x01] = false;
		$state.assignments[0x02] = false;

	};
}





const DB: bool = true;
const DB_REGISTERS: bool = true;


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
		// 
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

// insert an instruction into the chunk

fn place(instruction: u16, wh: usize, chunk: &mut [u8; 4096]) {
	let wh = wh as usize;

	chunk[wh..wh+2].copy_from_slice(
		&u16::to_be_bytes(instruction)
		);

}

// the PC is at a location
// load two bytes from there, then increment PC by two bytes

fn fetch(pc: &mut usize, chunk: &[u8; 4096]) -> u16 {
	// let _pc = *pc as usize;
	// let pc = *pc;

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

			// supposed to overflow?

			// flag?

			v[x] = v[x].wrapping_add(nn);

			// if let Some(val) = v[x].checked_add(nn) {
			// 	v[x] = val;
			// }
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
					// no
					println!("set V{x:#x} += V{y:#x}");

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
					// no
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
				0x29 => {
					print!("retreive font address of character V{:#x} {:#x} ... ", x, v[x]);

					let address = 5 * (v[x] as usize);

					print!("{:#x}\n", address);

					*i = address;
				}


				0x0A => {
					db!("wait for key...");

					db!("got key");
				}


				0x07 => {
					db!("set V{:#x} to value of timer, that being: {:#x}", x, t);

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

	let e_rate = Duration::from_secs(1 / 10);
	let mut start = SystemTime::now();

	let t_rate = Duration::from_secs(1 / 60);
	let mut t_start = SystemTime::now();

	let mut timer: u8 = 0;
	let mut sound_timer: u8 = 0;



	// let mut rw = rewrite::State::new();
	// draw!(rw, 0xF, 5, 5, 5);
	// data!(rw, 'b', ...)
	// draw!(rw, 'b', 10, 10, 10);

	// copy rw to program
	// chunk[pc..0xFFF].copy_from_slice(&rw.program[0..rewrite::PROGRAM_LEN]);
	// test_print_slice_as_u16(&chunk[pc..pc+24]);



	if let Some(fname) = env::args().nth(1) {
		println!("The filename argument is {}", fname);

		let mut f = File::open(fname).expect("bad filename");
		f.read(&mut chunk[pc..0xFFF]).expect("ughhhhghghg");
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
				test_draw_display(&display);
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
