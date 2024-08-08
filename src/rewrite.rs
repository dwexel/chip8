

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
		// let name = String::from(name);

		// println!("{:?}", self.assignments);
		// println!("{name}");


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


	// let x = state.find_register(Assignment::Anonymous);
	let x = state.get(name);

	let nn = match condition {
	    Some(nn) => nn,
	    None => 1,
	};


	// 6XNN
	// b = 0x6 << 4; b |= x; state.byte_push(b);
	// b = nn; state.byte_push(b);

	// 3XNN
	b = 0x3 << 4; b |= x; state.byte_push(b);
	b = nn; state.byte_push(b);

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
			state.non_user_stack.push(16);
			x
	    }
	    None => {
	    	let x = state.find_register(Assignment::Anonymous);
	    	state.non_user_stack.push(x);
	    	x
	    }
	};

	let count = match count {
		Valued::Literal(_c) => _c,
		_ => panic!()
	};


	let mut b: u8;
	

	// let mut nn = 0x00;
	let mut nn = 0x00;

	// 6XNN
	b = 0x6 << 4; b = b | x; state.byte_push(b);
	b = nn; state.byte_push(b);

	state.send_forward.push(Section::Loop(state.pcc));

	let nn = count;
	
	// 4XNN
	b = 0x4 << 4; b = b | x; state.byte_push(b);
	b = nn; state.byte_push(b);
	
	state.pcc += 2;

	let nn = 1;
	
	// 7XNN
	b = 0x7 << 4; b = b | x; state.byte_push(b);
	b = nn; state.byte_push(b);
}

pub fn loop_end(state: &mut State) {
	let s = state.send_forward.pop().expect("e");

	if let Section::Loop(jump_back_addr) = s {
		let mut b: u8;

		let nnn = jump_back_addr.to_be_bytes();
		
		// 1NNN
		b = 0x1 << 4; b |= nnn[0]; state.byte_push(b);
		b = nnn[1]; state.byte_push(b);

		let nnn = state.pcc.to_be_bytes();
		let copy_to_addr = jump_back_addr as usize + 2;

		// todo could use .memcpy
		// 1NNN
		b = 0x1 << 4; b |= nnn[0]; state.program[copy_to_addr] = b;
		b = nnn[1]; state.program[copy_to_addr + 1] = b;


		let loop_reg = state.non_user_stack.pop().expect("same error as above gr");

		if loop_reg != 16 {
			state.dissasign(loop_reg);
		}
	} else {
		panic!("syntax error");
	}
}



pub enum Ops {
    Add,
    Subtract,
}



pub fn operate(state: &mut State, name: &str, operator: Ops, operand: Valued) {
	let mut b: u8;

	let x = state.get(name);

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
			}
	    }
	    Ops::Subtract => {
	    	match operand {
	    		Valued::Symbol(oname) => {
	    			let y = state.get(&oname);

	    			// 8XY5
	    			b = 0x8 << 4; b |= x; state.byte_push(b);
	    			b = y << 4; b |= 5; state.byte_push(b);
	    		}
	    		_ => todo!()
	    	}
	    }
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
	   Valued::Data(_) => panic!()
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
		Valued::Data(_) => panic!()
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
		}
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
pub fn declare(state: &mut State, register: Option<u8>, name: &str) {
	let _x = match register {
		Some(_x) => todo!(),

		None => {
			state.find_register(Assignment::from(name))
		}
	};
}

pub fn assign(state: &mut State, name: &str, value: Valued) {
	let mut b: u8;
	let x = state.get(name);
	
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
		Valued::Data(_) => todo!()
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