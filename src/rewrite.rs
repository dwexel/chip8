

use std::{collections::HashMap, ops::Index};

use crate::{test_print_slice, test_print_slice_as_u16};

#[derive(Debug)]
enum Assignment {
	None,
	Anonymous,
	Nonymous(String)  
}

impl From<&str> for Assignment {
    fn from(value: &str) -> Self {
        Assignment::Nonymous(String::from(value))
    }
}

enum Section {
	If(u16),
	Loop(u16)
}


const PROGRAM_START: u16 = 0x200;
const PROGRAM_LEN: u16 = 0xFFF - PROGRAM_START; 
const DATA_SECTION: u16 = 0x200 + (PROGRAM_LEN / 2);




pub struct State {
	assignments: [Assignment; 16],
	datas: HashMap<String, u16>,
	program: [u8; 4096],
	pcc: u16,
	dpcc: u16,

	// send_forward: Vec<u16>,
	send_forward: Vec<Section>,

	non_user_stack: Vec<u8>,

	// send_forward_if: Vec<u16>,
	shift_machine: Option<u16>,

}

impl State {
	pub fn new() -> Self {
		Self {
			assignments: [Assignment::None, Assignment::None, Assignment::None, Assignment::None, Assignment::None, Assignment::None, Assignment::None, Assignment::None, Assignment::None, Assignment::None, Assignment::None, Assignment::None, Assignment::None, Assignment::None, Assignment::None, Assignment::Nonymous(String::from("overflow")) ],
			datas: HashMap::<String, u16>::new(),
			program: [0; 4096],
			// pcc: 0x200,
			pcc: PROGRAM_START,
			dpcc: DATA_SECTION,

			send_forward: Vec::new(),
			non_user_stack: Vec::new(),

			// send_forward_if: Vec::new(),
			shift_machine: None
		}
	}

	fn byte_push(&mut self, b: u8) {
		if self.pcc >= DATA_SECTION {
			panic!("ough");
		}

		self.program[self.pcc as usize] = b;
		self.pcc += 1;
	}

	pub fn copy_program_to_memory(&self, chunk: &mut[u8; 0x1000]) {
		chunk[0x200..0x1000].copy_from_slice(&self.program[0x200..0x1000]);

	}

	fn find_register(&mut self, a: Assignment) -> u8 {
		let mut v: u8 = 0;
		let mut replace: bool = false;

		for _a in self.assignments.iter() {
			if matches!(_a, Assignment::None) { replace = true; break; }
			v += 1;
		}

		if replace {
			match &a {
			    Assignment::None => panic!(),
			    Assignment::Nonymous(name) => {
			    	
			    	State::is_a_good_name(name).expect("bad name error");
			    	println!("assign register {v}, to name {name}");

			    },
			    Assignment::Anonymous => println!("assign register {v} Anonymously")
			}
			self.assignments[v as usize] = a;
			return v;
		}

		panic!();
	}

	fn dissasign(&mut self, v: u8) {
		println!("dissasign register {v}");

		self.assignments[v as usize] = Assignment::None;
	}

	fn get(&self, name: &str) -> u8 {
		let mut i: u8 = 0;

		for _i in 0..16 {
			match &self.assignments[_i] {
				Assignment::Nonymous(n) if n.eq(&name) => {
					println!("resolved register {i} name {n}");
					return i;
				}
				_ => {}
			}
			i += 1;
		}

		panic!("register variable {name} is not declared");
	}

	fn is_a_good_name(name: &String) -> Result<(), ()> {
		if name.is_empty() {
			return Err(());
		}

		Ok(())
	}

	fn print_up(&self) {
    	test_print_slice_as_u16(&self.program[(PROGRAM_START as usize)..(self.pcc as usize)]);

	}
}


pub fn data(state: &mut State, name: &str, bytes: &[u8]) {
	let data_start = state.dpcc;

	state.datas.insert(String::from(name), state.dpcc);

	// let bytes: [ u8; ${count($e)} ] = [ $( $e, )* ];

	for b in bytes {
		state.program[state.dpcc as usize] = *b;
		state.dpcc += 1;
	}

	print!("data slice starting at {data_start:#x} ");

	test_print_slice(&state.program[(data_start as usize)..(state.dpcc as usize)]);
}



pub enum Valued {
	Literal(u8),
	Symbol(String),
	Data(String),

	Register(u8),
}

impl From<&str> for Valued {
    fn from(value: &str) -> Self {
        Valued::Symbol(String::from(value))
    }
}

impl From<u8> for Valued {
    fn from(value: u8) -> Self {
        Valued::Literal(value)
    }
}


pub fn if_start(state: &mut State, condition: Option<u8>, name: &str) {
	let mut b: u8;

	let x = state.get(name);

	let nn = match condition {
	    Some(nn) => {
	    	// check equivalence

			// 3XNN
			b = 0x3 << 4; b |= x; state.byte_push(b);
			b = nn; state.byte_push(b);
	    },
	    None => {
	    	// check boolness
	    	let nn = 0x00;

			// 4XNN
			b = 0x4 << 4; b |= x; state.byte_push(b);
			b = nn; state.byte_push(b);

			// println!("{:?}", state.program);
			state.print_up();
	    },
	};



	// 1NNN
	state.send_forward.push(Section::If(state.pcc));
	state.pcc += 2;

}

pub fn if_end(state: &mut State) {
	let s = state.send_forward.pop().expect("e");

	if let Section::If(copy_to_addr) = s {
		let mut b: u8;

		let copy_to_addr = copy_to_addr as usize;
		let nnn = state.pcc.to_be_bytes();

		// 1NNN
		b = 0x1 << 4; b |= nnn[0]; state.program[copy_to_addr] = b;
		b = nnn[1]; state.program[copy_to_addr + 1] = b;

	} else {
		panic!("syntax error: unclosed loop");
	}
}


pub fn loop_start(state: &mut State, count: Valued, name: Option<&str>) {
	let x = match name {
	    Some(name) => {
			let x = state.find_register(Assignment::from(name));
			state.non_user_stack.push(x);
			x
	    }
	    None => {
	    	let x = state.find_register(Assignment::Anonymous);
	    	state.non_user_stack.push(x);
	    	x
	    }
	};

	let mut b: u8;
	
	let mut nn = 0x00;

	// hm
	// 6XNN
	b = 0x6 << 4; b = b | x; state.byte_push(b);
	b = nn; state.byte_push(b);

	state.send_forward.push(Section::Loop(state.pcc));

	match count {
	    Valued::Literal(nn) => {

			// 4XNN
			b = 0x4 << 4; b = b | x; state.byte_push(b);
			b = nn; state.byte_push(b);	    	
	    }
	    Valued::Symbol(ref name) => {
	    	let y = state.get(name);

			// 9XY0
			b = 0x9 << 4; b |= x; state.byte_push(b);
			b = y << 4; b |= 0x0; state.byte_push(b);

	    }
	    Valued::Data(_) => panic!(),
	    Valued::Register(_) => panic!(),
	}
	
	// >> 1NNN
	state.pcc += 2;

	
	// // 7XNN
	// let nn = 1;
	// b = 0x7 << 4; b |= x; state.byte_push(b);
	// b = nn; state.byte_push(b);
}

pub fn loop_end(state: &mut State) {
	let s = state.send_forward.pop().expect("e");

	if let Section::Loop(jump_back_addr) = s {
		let mut b: u8;

		// 7XNN
		let nn = 1;
		let loop_reg = state.non_user_stack.pop().expect("same error as above gr");
		b = 0x7 << 4; b |= loop_reg; state.byte_push(b);
		b = nn; state.byte_push(b);


		let nnn = jump_back_addr.to_be_bytes();
		
		// 1NNN
		b = 0x1 << 4; b |= nnn[0]; state.byte_push(b);
		b = nnn[1]; state.byte_push(b);

		let nnn = state.pcc.to_be_bytes();
		let copy_to_addr = jump_back_addr as usize + 2;

		// todo could use .memcpy
		// << 1NNN
		b = 0x1 << 4; b |= nnn[0]; state.program[copy_to_addr] = b;
		b = nnn[1]; state.program[copy_to_addr + 1] = b;


		// if loop_reg != 16 {
			state.dissasign(loop_reg);
		// }
	} else {
		panic!("syntax error");
	}
}



pub enum Ops {
    Add,
    Subtract,
    Shl,
    Shr,
    BitAnd,
    BitOr,
    BitXor
}



pub fn operate(state: &mut State, variable: Valued, operator: Ops, operand: Valued) {
	let mut b: u8;

	// let x = state.get(name);
	let x = match variable {
	    Valued::Symbol(ref name) => state.get(name),
	    Valued::Register(x) => x,
	    _ => todo!(),
	};

	match operator {
	    Ops::Add => {
	    	match operand {
				Valued::Literal(value) => {
					let nn = value;

					// 7XNN
					b = 0x7 << 4; b |= x; state.byte_push(b);
					b = nn; state.byte_push(b);
				},
				Valued::Symbol(oname) => {
					let y = state.get(&oname);
					
					// 8XY4
					b = 0x8 << 4; b |= x; state.byte_push(b);
					b = y << 4; b |= 4; state.byte_push(b);
				},
				Valued::Data(_) => todo!(),
		
			    Valued::Register(y) => {

					// 8XY4
					b = 0x8 << 4; b |= x; state.byte_push(b);
					b = y << 4; b |= 4; state.byte_push(b);			    	
			    },

			}
	    }
	    Ops::Subtract => {
	    	match operand {
	    		Valued::Symbol(oname) => {
	    			let y = state.get(&oname);

	    			// 8XY5
	    			b = 0x8 << 4; b |= x; state.byte_push(b);
	    			b = y << 4; b |= 0x5; state.byte_push(b);
	    		}
	    		_ => todo!()
	    	}
	    }
	    Ops::Shl => {
	    	match operand {
	    	    Valued::Symbol(ref oname) => {

	    	    	// shift operator
	    	    	let _x = state.get("_so");

	    	    	// move 7
	    	    	// 6XNN
	    	    	b = 0x6 << 4; b |= _x; state.byte_push(b);
	    	    	b = 0x07; state.byte_push(b);
	    	    	
	    	    	let y = state.get(oname);

	    	    	// bit AND
	    	    	// 8XY2
	    	    	b = 0x8 << 4; b |= _x; state.byte_push(b);
	    	    	b = y << 4; b |= 0x2; state.byte_push(b);

	    	    	let nnn = state.shift_machine.expect("mistake you did boy").to_be_bytes();

	    	    	// sv = shift value (register)
	    	    	let _x = state.get("_sv");
	    	    	let y = x;

	    	    	// move in the operated
	    	    	// 8XY0
	    	    	b = 0x8 << 4; b |= _x; state.byte_push(b);
	    	    	b = y << 4; b |= 0x0; state.byte_push(b);

	    	    	// 2NNN
	    	    	b = 0x2 << 4; b |= nnn[0]; state.byte_push(b);
	    	    	b = nnn[1]; state.byte_push(b);

	    	    	// move it back home
	    	    	// 8XY0
	    	    	b = 0x8 << 4; b |= y; state.byte_push(b);
	    	    	b = _x << 4; b |= 0x0; state.byte_push(b);
	    	    }

	    	    Valued::Literal(value) => {
	    	    	if value > 0 {
	    	    		if value > 7 {
	    	    			panic!();
	    	    		}

	    	    		for i in 0..value {
	    	    			// y doesn't matter right now
			    	    	// 8XYE
			    	    	b = 0x8 << 4; b |= x; state.byte_push(b);
			    	    	b = 0 << 4; b |= 0xE; state.byte_push(b);
	    	    		}
	    	    	}
	    	    }
	    	    _ => todo!()
	    	}
	    }
	    Ops::Shr => {
	    	match operand {
	    	    Valued::Symbol(oname) => {

	    	    	// 8XY6
	    	    	todo!();
	    	    }
	    	    _ => todo!()
	    	}

	    }
	    Ops::BitAnd => {
	    	match operand {
	    	    Valued::Literal(value) => { todo!(); }
	    	    Valued::Symbol(name) => {
	    	    	let y = state.get(&name);

	    			// 8XY2
	    			b = 0x8 << 4; b |= x; state.byte_push(b);
	    			b = y << 4; b |= 0x2; state.byte_push(b);
	    	    }

	    	    _ => todo!()
	    	}



	    },
	    Ops::BitOr => todo!(),
	    Ops::BitXor => todo!(),
	}



}

// todo optimize by moving "set index register" out of the loop

pub fn draw(state: &mut State, data: Valued, xval: Valued, yval: Valued, rows: Valued) {
	let emit_start = state.pcc;


	let x = match xval {
		Valued::Literal(xval) => {
			let mut b: u8;

			// 6XNN
			let x = state.find_register(Assignment::Anonymous);
			let nn = xval;
			b = 0x6 << 4; b = b | x; state.byte_push(b);
			b = nn; state.byte_push(b);

			x
		},
		Valued::Symbol(ref name) => {
	   		state.get(name)
	  	},
		Valued::Data(_) => panic!(),
		Valued::Register(_) => panic!(),

	};

	let y = match yval {
	   Valued::Literal(yval) => {
			let mut b: u8;

			// 6XNN
	   		let x = state.find_register(Assignment::Anonymous);
			let nn = yval;
			b = 0x06 << 4; b = b | x; state.byte_push(b);
			b = nn; state.byte_push(b);

			x
		},
		Valued::Symbol(ref name) => {
			state.get(name)
		},
		Valued::Data(_) => panic!(),
	    Valued::Register(_) => panic!(),
	};


	match data {
		Valued::Literal(font_character) => {
	   		let mut b: u8;

			// 6XNN
			let nn = font_character as u8;
			let _x = state.find_register(Assignment::Anonymous);

			b = 0x6 << 4; b = b | _x; state.byte_push(b);
			b = nn; state.byte_push(b);

			// FX29
			b = 0xF << 4; b = b | _x; state.byte_push(b);
			b = 0x29; state.byte_push(b);

			state.dissasign(_x);
		},
		Valued::Symbol(name) => {
			panic!();
		}
		Valued::Data(name) => {
			let mut b: u8;

			// ANNN
			let nnn = *state.datas.get(&name).expect("symbol not existing");
			let nnn = nnn.to_be_bytes();


			b = 0xA << 4; b |= nnn[0]; state.byte_push(b);
			b = nnn[1]; state.byte_push(b);
		},
	    Valued::Register(_) => panic!(),

	};


	let n = match rows {
		Valued::Literal(value) => value,
		_ => panic!() 
	};

	let mut b: u8;

	let x = x;
	let y = y;
	let n = n;

	// DXYN
	b = 0xD << 4; b = b | x; state.byte_push(b);
	b = y << 4; b = b | n; state.byte_push(b);


	print!("draw function: ");
	test_print_slice_as_u16(&state.program[(emit_start as usize)..(state.pcc as usize)]);

	if matches!(xval, Valued::Literal(_)) { state.dissasign(x) }
	if matches!(yval, Valued::Literal(_)) { state.dissasign(y) }
}


// todo option to give a value
// hm 
// don't return
pub fn declare(state: &mut State, register: Option<u8>, name: &str) -> u8 {
	let _x = match register {
		Some(_x) => todo!(),

		None => {
			state.find_register(Assignment::from(name))
		}
	};

	_x
}

pub fn assign(state: &mut State, variable: Valued, value: Valued) {
	let mut b: u8;
	
	// let x = state.get(name);

	let x = match variable {
	    Valued::Register(x) => x,
	    Valued::Symbol(ref name) => state.get(name),

	    Valued::Literal(_) => todo!(),
	    Valued::Data(_) => todo!(),
	};

	
	match value {
	    Valued::Literal(value) => {
	    	let nn = value;

	    	// 6XNN
	    	b = 0x6 << 4; b |= x; state.byte_push(b);
	    	b = nn; state.byte_push(b);
	    },
	    Valued::Symbol(ref name) => {
	    	let y = state.get(name);

	    	// 8XY0
	    	b = 0x8 << 4; b |= x; state.byte_push(b);
	    	b = y << 4; b |= 0x0; state.byte_push(b);
	    }
	    Valued::Register(y) => {

	    	// 8XY0
	    	b = 0x8 << 4; b |= x; state.byte_push(b);
	    	b = y << 4; b |= 0x0; state.byte_push(b);	    	
	    },
		Valued::Data(_) => todo!(),
	}
}

pub fn increment(state: &mut State, name: &str) {
	let mut b: u8;

	let x = state.get(name);
	let nn = 1;

	// 7XNN
	b = 0x7 << 4; b = b | x; state.byte_push(b);
	b = nn; state.byte_push(b);
}

pub fn gap(state: &mut State) {
	state.pcc += 2;
}




pub fn shift_machine(state: &mut State) {
	let mut b: u8;

	let x = state.find_register(Assignment::from("_sv"));
	let _y = state.find_register(Assignment::from("_so"));

	// >> 1NNN
	let jump_past = state.pcc as usize;
	state.pcc += 2;

	state.shift_machine = Some(state.pcc);

	let _x = state.find_register(Assignment::Anonymous);
	let nn = 0x00;

	// 6XNN
	b = 0x6 << 4; b |= _x; state.byte_push(b);
	b = nn; state.byte_push(b);

	//
	let jump_back_addr = state.pcc.to_be_bytes();

	// 9XY0
	// test x != y
	b = 0x9 << 4; b |= _x; state.byte_push(b);
	b = _y << 4; b |= 0x0; state.byte_push(b);

	// 00EE
	b = 0x00; state.byte_push(b);
	b = 0xEE; state.byte_push(b);
	
	// 7XNN
	let nn = 1;
	b = 0x7 << 4; b |= _x; state.byte_push(b);
	b = nn; state.byte_push(b);

	// bit chift
	// ya y register huh
	b = 0x8 << 4; b |= x; state.byte_push(b);
	b = 0x0 << 4; b |= 0xE; state.byte_push(b);

	// 1NNN
	let nnn = jump_back_addr;
	b = 0x1 << 4; b |= nnn[0]; state.byte_push(b);
	b = nnn[1]; state.byte_push(b);

	// << 1NNN
	let nnn = state.pcc.to_be_bytes();
	b = 0x1 << 4; b |= nnn[0]; state.program[jump_past] = b;
	b = nnn[1]; state.program[jump_past + 1] = b;
}




fn num_bits(b: u8) -> u8 {
	match b {
		0 => 0,
		1 => 1,
		2..4 => 2,
		4..8 => 3,
		8..16 => 4,
		16..32 => 5,
		32..64 => 6,
		64..128 => 7,
		128..=255 => 8
	}
}



// it should syntax error if you don't close loop before program ends

pub fn multiply(state: &mut State) {
	let m1 = state.find_register(Assignment::Anonymous); assign(state, Valued::Register(m1), Valued::Literal(21));
	let by = Valued::Literal(3);

	let dest = state.find_register(Assignment::Anonymous);

	// toodoo
	match by {
		Valued::Literal(by) => {

			for i in 0..num_bits(by) {
				if (by & (1 << i) != 0) {
					// if 0 < i {
					// 	operate(state, Valued::Register(m1), Ops::Shl, Valued::Literal(1));
					// }

					operate(state, Valued::Register(dest), Ops::Add, Valued::Register(m1));
					operate(state, Valued::Register(m1), Ops::Shl, Valued::Literal(1));

				} else {
					println!("no");
				}
			}



		}
		_ => todo!()
	}

	state.dissasign(m1);
	state.dissasign(dest);


}

pub fn multiply_v(state: &mut State) {
	// let m1 = state.find_register(Assignment::Anonymous); assign(state, Valued::Register(m1), Valued::Literal(21));
	// let m2 = state.find_register(Assignment::Anonymous); assign(state, Valued::Register(m2), Valued::Literal(3));




	state.find_register(Assignment::from("m1"));
	assign(state, Valued::from("m1"), Valued::Literal(3));

	state.find_register(Assignment::from("m2"));
	assign(state, Valued::from("m2"), Valued::Literal(21));


	state.find_register(Assignment::from("dest"));



	let mut b: u8;




	loop_start(state, Valued::Literal(8), Some("i"));


    	assign(state, Valued::from("_so"), Valued::from("i"));


    	// sv = shift value (register)
    	// 6XNN
    	let _x = state.get("_sv");
    	let nn = 0x1;
    	b = 0x6 << 4; b |= _x; state.byte_push(b);
    	b = nn; state.byte_push(b);


    	// 2NNN
    	let nnn = state.shift_machine.expect("mistake you did boy").to_be_bytes();
    	b = 0x2 << 4; b |= nnn[0]; state.byte_push(b);
    	b = nnn[1]; state.byte_push(b);


    	// bit and
    	// m1 and reg result
    	operate(state, Valued::from("_sv"), Ops::BitAnd, Valued::from("m1"));



		// if_start(state, Some(4), "_sv");
		if_start(state, None, "_sv");

			// add m2 to dest
			operate(state, Valued::from("dest"), Ops::Add, Valued::from("m2"));



			// draw(state, Valued::Literal(1), Valued::Literal(1), Valued::Literal(1), Valued::Literal(5));

		if_end(state);


		// then shift m2 left

		// 8XYE
		let x = state.get("m2");
		b = 0x8 << 4; b |= x; state.byte_push(b);
		b = 0x0 << 4; b |= 0xE; state.byte_push(b);




	loop_end(state);
}




pub fn place(state: &mut State, instruction: u16) {
	
	let instruction = u16::to_be_bytes(instruction);
	state.byte_push(instruction[0]);
	state.byte_push(instruction[1]);
}



// pub fn parse(expression: &str) {
	// let tok: Vec<&str> = Vec::new();

	// expression.char_indices().fold(tok, |acc, (i, c)| {

	// });

	// for (i, c) in expression.char_indices() {
	// 	if c.is_whitespace() { continue; }
	// }



	// let chars = expression.chars().collect();
	// let chars: Vec<char> = expression.chars().collect();

	// for i in 0..chars.len() {
	// 	let c = chars.get(i).expect("huh");

	// 	while c.is_digit(10) {
	// 	    i += 1;
	// 	}
	// }


	// let mut chars = expression.chars();


	// while let Some(c) = chars.next() {
	// 	while c.is_digit(10) {
	// 		chars.next();
	// 	}
	// }


// }

