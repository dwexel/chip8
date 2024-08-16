
use std::{collections::HashMap, ops::Index};

use crate::{test_print_slice, test_print_slice_as_u16};

macro_rules! instruction {
	($state:ident, $o:expr) => {};
	
	($state:ident, $o:expr, NNN:$nnn:expr) => {
		let mut b: u8;
		let nnn = $nnn.to_be_bytes();

		b = $o << 4; b |= nnn[0]; $state.byte_push(b);
		b = nnn[1]; $state.byte_push(b);
	};

	($state:ident, $o:expr, X:$x:expr, NN:$nn:expr) => {
		let mut b: u8;
		b = $o << 4; b |= $x; $state.byte_push(b);
		b = $nn; $state.byte_push(b);
	};

	($state:ident, $o:expr, X:$x:expr, $o2:expr) => {
		let mut b: u8;
		b = $o << 4; b |= $x; $state.byte_push(b);
		b = $o2; $state.byte_push(b);
	};

	($state:ident, $o:expr, X:$x:expr, Y:$y:expr, $o2:expr) => {
		let mut b: u8;
		b = $o << 4; b |= $x; $state.byte_push(b);
		b = $y << 4; b |= $o2; $state.byte_push(b);
	};

	($state:ident, $o:expr, X:$x:expr, Y:$y:expr, N:$n:expr) => {
		let mut b: u8;
		b = $o << 4; b |= $x; $state.byte_push(b);
		b = $y << 4; b |= $n; $state.byte_push(b);
	};
}

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
	Loop(u16, Valued)
}

const PROGRAM_START: u16 = 0x200;
// PROGRAM_END = 0xFFF
const PROGRAM_LEN: u16 = 0xFFF - PROGRAM_START; 
const DATA_SECTION: u16 = 0x200 + (PROGRAM_LEN / 2);

pub struct State {
	assignments: [Assignment; 16],
	datas: HashMap<String, (u16, u16)>,
	program: [u8; 4096],
	pcc: u16,
	pcd: u16,
	send_forward: Vec<Section>,
	non_user_stack: Vec<u8>,
	shift_machine: Option<u16>,
}

impl State {
	pub fn new() -> Self {
		Self {
			assignments: [Assignment::None, Assignment::None, Assignment::None, Assignment::None, Assignment::None, Assignment::None, Assignment::None, Assignment::None, Assignment::None, Assignment::None, Assignment::None, Assignment::None, Assignment::None, Assignment::None, Assignment::None, Assignment::Nonymous(String::from("overflow")) ],
			datas: HashMap::<String, (u16, u16)>::new(),
			program: [0; 4096],
			pcc: PROGRAM_START,
			pcd: DATA_SECTION,
			send_forward: Vec::new(),
			non_user_stack: Vec::new(),
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
		for i in 0_u8..16 {
			match &self.assignments[i as usize] {
				Assignment::Nonymous(n) if n.eq(&name) => {
					println!("resolved register {i} name {n}");
					return i;
				}
				_ => {}
			}
		}

		panic!("register variable {name} is not declared");
	}

	fn try_get(&self, name: &str) -> Option<u8> {
		for i in 0_u8..16 {
			match &self.assignments[i as usize] {
				Assignment::Nonymous(n) if n.eq(&name) => {
					println!("resolved register {i} name {n}");
					return Some(i);
				}
				_ => {}
			}
		}

		None
	}

	fn is_a_good_name(name: &str) -> Result<(), ()> {
		if name.is_empty() {
			return Err(());
		}

		Ok(())
	}

	fn print_up(&self) {
    	test_print_slice_as_u16(&self.program[(PROGRAM_START as usize)..(self.pcc as usize)]);
	}
}

// use std::intrinsics::bitreverse;



pub struct Flip (pub bool, pub bool);

pub fn data(state: &mut State, name: &str, bytes: &[u8]) {
	state.datas.insert(name.to_owned(), (state.pcd, bytes.len().try_into().unwrap()));

	for b in bytes {
		state.program[state.pcd as usize] = *b;
		state.pcd += 1;
	}
}

pub fn data_flipped(state: &mut State, name: &str, data: Valued, flip: Flip) {
	let (pcd, len) = match data {
	    Valued::Data(name) => {
	    	state.datas.get(&name).expect("fuck")
	    },
	    Valued::Literal(character) => { todo!(); },
	    Valued::Symbol(name) => todo!(),
	    Valued::Register(_) => todo!(),
		_ => todo!(),
	};

	let mut bytes = Vec::<u8>::new();

	let Flip(x, y) = flip;

	if y {
		let range = ((*pcd)..(*pcd + *len)).rev();
		for _i in range {
			let byte = state.program[_i as usize];

			if x {
				bytes.push(byte.reverse_bits());
			} else {
				bytes.push(byte);
			}
		}
	} else {
		let range = (*pcd)..(*pcd + *len);
		for _i in range {
			let byte = state.program[_i as usize];

			if x {
				bytes.push(byte.reverse_bits());
			} else {
				bytes.push(byte);
			}
		}			
	}

	println!("{bytes:?}");

	state.datas.insert(name.to_owned(), (state.pcd, bytes.len().try_into().unwrap()));

	// lol
	for b in bytes {
		state.program[state.pcd as usize] = b;
		state.pcd += 1;
	}
}

#[derive(Clone)]
pub enum Valued {
	Literal(u8),
	Symbol(String),
	Data(String),

	Expression(Vec<(Valued, Ops)>),
		// Expression(Vec<Valued>/),
		// internal use
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
#[derive(Clone)]
pub enum Ops {
    Add,
    Subtract,
    Multiply,

    Shl,
    Shr,
    BitAnd,
    BitOr,
    BitXor
}

pub fn if_start(state: &mut State, condition: Option<u8>, variable: Valued) {
	let mut b: u8;

	let x = match variable {
	    Valued::Symbol(ref name) => state.get(name),
	    Valued::Register(x) => x,
	    Valued::Expression(ref expr) => {
	    	let v_dest = state.find_register(Assignment::Anonymous);
	    	for (operand, operator) in expr {
	    		operate(state, v_dest, operand.clone(), operator.clone());
	    	}
			v_dest
	    }
		_ => todo!(),
	};

	match condition {
	    Some(nn) => {
	    	// check equivalence
			// 3XNN
			instruction!(state, 0x3, X:x, NN:nn);
	    },
	    None => {
	    	let nn = 0x00;
	    	// check boolness
			// 4XNN
			instruction!(state, 0x4, X:x, NN:nn);
	    },
	}

	match variable { Valued::Expression(_) => { state.dissasign(x); }, _ => {} };

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
		panic!("syntax error");
	}
}

pub fn loop_start(state: &mut State, count: Valued, name: Option<&str>) {
	let x = match name {
	    Some(name) => {
			state.find_register(Assignment::from(name))
	    }
	    None => {
	    	state.find_register(Assignment::Anonymous)
	    }
	};

	// 6XNN
	instruction!(state, 0x6, X:x, NN:0x0);
	state.non_user_stack.push(x);
	state.send_forward.push(Section::Loop(state.pcc, count));
}

pub fn loop_end(state: &mut State) {
	let s = state.send_forward.pop().expect("closing loop no opener");

	if let Section::Loop(jump_back_addr, count) = s {

		// 7XNN
		let loop_reg = state.non_user_stack.pop().expect("same error as above gr");
		instruction!(state, 0x7, X:loop_reg, NN:1);

		// test
		match count {
		    Valued::Literal(nn) => {
				// 3XNN skip if equal
				instruction!(state, 0x3, X:loop_reg, NN:nn);
		    }
		    Valued::Symbol(ref name) => {
		    	// let y = state.get(name);
		    	// let x = loop_reg;
				// 5XY0 skip if equal
				instruction!(state, 0x5, X:loop_reg, Y:state.get(name), 0x0);
		    }
		    _ => todo!()
		}
		
		// 1NNN
		instruction!(state, 0x1, NNN:jump_back_addr);

		// still dissasigns it if the symbol existed before the loop
		state.dissasign(loop_reg);

	} else {
		panic!("syntax error");
	}
}

pub fn while_loop_start(state: &mut State) {
	state.non_user_stack.push(16);
	state.send_forward.push(Section::Loop(state.pcc, Valued::Register(0)));
}

pub fn while_loop_end(state: &mut State, condition: Valued) {
	state.non_user_stack.pop().expect("uh");
	let s = state.send_forward.pop().expect("uh");

	if let Section::Loop(loop_start, _) = s {
		// match condition {
		// 	Valued::Symbol(name) => {
		// 		let x = state.get(&name);
		// 		instruction!(state, 0x4, X:x, NN:0x0);
		// 	}
		// 	_ => panic!()
		// }
		if let Valued::Symbol(name) = condition {
			instruction!(state, 0x4, X:state.get(&name), NN:0x0);
		}
		instruction!(state, 0x1, NNN:loop_start);
	} else {
		panic!();
	}
}

// would need a global break stack
// pub fn loop_break() {}

pub fn assign(state: &mut State, variable: Valued, value: Valued) {
	let mut b: u8;
	
	let x = match variable {
	    Valued::Symbol(name) => {
	    	if let Some(x) = state.try_get(&name) {
	    		x
	    	} else {
	    		state.find_register(Assignment::Nonymous(name))
	    	}
	    },
	    Valued::Register(x) => x,
		_ => todo!(),
	};
	
	match value {
	    Valued::Literal(nn) => {
	    	instruction!(state, 0x6, X:x, NN:nn);
	    },
	    Valued::Symbol(ref name) => {
	    	let y = state.get(name);
	    	instruction!(state, 0x8, X:x, Y:y, 0x0);
	    }
	    Valued::Register(y) => {
	    	instruction!(state, 0x8, X:x, Y:y, 0x0);
	    },
		Valued::Data(_) => todo!(),
		// todo single expr
		Valued::Expression(expression) => {
			for (operand, operator) in expression {
				operate(state, x, operand, operator);
			}
		}
		_ => todo!(),
	}
}

pub fn increment(state: &mut State, name: &str) {
	instruction!(state, 0x7, X:state.get(name), NN:0x1);
}

// fn test_expression(state: &mut State, expression: &[(Valued, Ops)]) {
// 	let v_dest = state.find_register(Assignment::Anonymous);
// 	// state.disassign(v_dest); i fucking spell wrog?
// 	for (operand, operator) in expression {
// 		operate_one(state, Valued::Register(v_dest), operand.clone(), operator.clone());
// 	}
	
// 	state.dissasign(v_dest);
// }

// pub fn operate(state: &mut State, name: &str, expression: &[(Valued, Ops)]){
// 	if state.try_get(&name).is_none() {
// 		state.find_register(Assignment::Nonymous(String::from(name)));
// 	}
	
// 	for (operand, operator) in expression {
// 		operate_one(state, Valued::from(name), operand.clone(), operator.clone());
// 	}
// }

fn operate(state: &mut State, v_var: u8, operand: Valued, operator: Ops) {
	
	match operator {
	    Ops::Add => {
	    	match operand {
				Valued::Literal(ovalue) => {
					// 7XNN
					instruction!(state, 0x7, X:v_var, NN:ovalue);
				},
				Valued::Symbol(oname) => {
					let v_op = state.get(&oname);					
					instruction!(state, 0x8, X:v_var, Y:v_op, 0x4);
				},		
			    Valued::Register(v_op) => {
					instruction!(state, 0x8, X:v_var, Y:v_op, 0x4);
			    },
				Valued::Data(_) => todo!(),
				_ => todo!(),
			}
	    }
	    Ops::Subtract => {
	    	match operand {
	    		Valued::Symbol(oname) => {
	    			let x = v_var;
	    			let y = state.get(&oname);
	    			// 8XY5
	    			instruction!(state, 0x8, X:x, Y:y, 0x5);
	    		}
	    		_ => todo!()
	    	}
	    }
	    Ops::Multiply => {
	    	let v_dest = state.find_register(Assignment::Anonymous);
	    	// move 0
	    	instruction!(state, 0x6, X:v_dest, NN:0x0);
	    	match operand {
	    		Valued::Symbol(name) => {
	    			let v_by = state.get(&name);
					let v_shift = state.find_register(Assignment::Anonymous);
					let v_test = state.find_register(Assignment::Anonymous);
					// set
					// 6XNN
					instruction!(state, 0x6, X:v_shift, NN:0x1);					
			    	loop_start(state, Valued::Literal(8), None);
				    	// move
				    	// 8XY0
				    	instruction!(state, 0x8, X:v_test, Y:v_by, 0x0);
		    			instruction!(state, 0x8, X:v_test, Y:v_shift, 0x2);
		    			// test ya
				    	if_start(state, None, Valued::Register(v_test));
				    		// 8XY4
				    		instruction!(state, 0x8, X:v_dest, Y:v_var, 0x4);
				    	if_end(state);
				    	// 8XYE
			    		// shift
			    		instruction!(state, 0x8, X:v_shift, Y:0, 0xE);
						// 8XYE
						// shift
						instruction!(state, 0x8, X:v_var, Y:0, 0xE);			    	
			    	loop_end(state);
			    	state.dissasign(v_shift);
			    	state.dissasign(v_test);
	    		}
	    		Valued::Literal(by) => {
	    			// make sure v_dest is clear

					for i in 0..num_bits(by) {
						if (by & (1 << i) != 0) {
							// add 
					    	// 8XY4
					    	let x = v_dest;
					    	let y = v_var;
					    	instruction!(state, 0x8, X:x, Y:y, 0x4);
						} 

						// 8XYE
						// shift
						let x = v_var;
						instruction!(state, 0x8, X:x, Y:0, 0xE);
					}

	    		}
	    		_ => todo!()
	    	}
			// move
	    	// 8XY0
	    	let x = v_var;
	    	let y = v_dest;
	    	instruction!(state, 0x8, X:x, Y:y, 0x0);
			state.dissasign(v_dest);
	    }

	    Ops::Shl => {
	    	match operand {
	    	    Valued::Symbol(ref oname) => {
	    	    	todo!();
	    	    }
	    	    Valued::Literal(value) => {
    	    		if value > 7 {
    	    			panic!();
    	    		}
    	    		let x = v_var;
    	    		for i in 0..value {
    	    			// y doesn't matter right now
		    	    	// 8XYE
		    	    	instruction!(state, 0x8, X:x, Y:0, 0xE);
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
	    			instruction!(state, 0x8, X:v_var, Y:y, 0x2);
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
		Valued::Literal(nn) => {
			let x = state.find_register(Assignment::Anonymous);
			instruction!(state, 0x6, X:x, NN:nn);
			x
		},
		Valued::Symbol(ref name) => {
	   		state.get(name)
	  	},
		Valued::Data(_) => panic!(),
		Valued::Register(_) => panic!(),

		_ => todo!(),
	};

	let y = match yval {
	   Valued::Literal(yval) => {
	   		let x = state.find_register(Assignment::Anonymous);
			instruction!(state, 0x6, X:x, NN:yval);
			x
		},
		Valued::Symbol(ref name) => {
			state.get(name)
		},
		Valued::Data(_) => panic!(),
	    Valued::Register(_) => panic!(),
		_ => todo!(),
	};


	match data {
		Valued::Literal(font_character) => {
	   		let mut b: u8;

			// 6XNN
			let nn = font_character;
			let _x = state.find_register(Assignment::Anonymous);

			instruction!(state, 0x6, X:_x, NN:nn);
			instruction!(state, 0xF, X:_x, 0x29);
			state.dissasign(_x);
		},
		Valued::Symbol(name) => {
			panic!();
		}
		Valued::Data(name) => {
			let (nnn, count) = *state.datas.get(&name).expect("symbol not existing");

			// emit a warning if the number of rows is off 
			if let Valued::Literal(rows) = rows {
				if rows  as u16 > count {
					eprintln!("waring: drawing too many rows");
				}
			}

			instruction!(state, 0xA, NNN:nnn);
		},
	    Valued::Register(_) => panic!(),
		_ => todo!(),
	};


	let n = match rows {
		Valued::Literal(value) => value,
		_ => panic!() 
	};

	let mut b: u8;

	// DXYN
	instruction!(state, 0xD, X:x, Y:y, N:n);

	print!("draw function: ");
	test_print_slice_as_u16(&state.program[(emit_start as usize)..(state.pcc as usize)]);

	if matches!(xval, Valued::Literal(_)) { state.dissasign(x) }
	if matches!(yval, Valued::Literal(_)) { state.dissasign(y) }
}


// todo option to give a value
// hm 
// don't return

// pub fn declare(state: &mut State, register: Option<u8>, name: &str) {
// 	match register {
// 		Some(_x) => todo!(),
// 		None => {
// 			state.find_register(Assignment::from(name))
// 		}
// 	};
// }


pub fn gap(state: &mut State) {
	state.pcc += 2;
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


pub fn place(state: &mut State, instruction: u16) {
	
	let instruction = u16::to_be_bytes(instruction);
	state.byte_push(instruction[0]);
	state.byte_push(instruction[1]);
}
