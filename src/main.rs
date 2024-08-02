// total 4096 bytes (000-FFF)
// 
// first 512 are empty (000-1FF)
// 
// except for the font










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



const DB: bool = false;


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

use std::{io::stdout, time::{Duration, SystemTime}};





const FONT_0: [u8; 5] = [0xF0, 0x90, 0x90, 0x90, 0xF0]; // 0
const FONT_1: [u8; 5] = [0x20, 0x60, 0x20, 0x20, 0x70]; // 1
const FONT_2: [u8; 5] = [0xF0, 0x10, 0xF0, 0x80, 0xF0]; // 2
const FONT_3: [u8; 5] = [0xF0, 0x10, 0xF0, 0x10, 0xF0]; // 3
const FONT_4: [u8; 5] = [0x90, 0x90, 0xF0, 0x10, 0x10]; // 4
const FONT_5: [u8; 5] = [0xF0, 0x80, 0xF0, 0x10, 0xF0]; // 5
const FONT_6: [u8; 5] = [0xF0, 0x80, 0xF0, 0x90, 0xF0]; // 6
const FONT_7: [u8; 5] = [0xF0, 0x10, 0x20, 0x40, 0x40]; // 7
const FONT_8: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0xF0]; // 8
const FONT_9: [u8; 5] = [0xF0, 0x90, 0xF0, 0x10, 0xF0]; // 9
const FONT_A: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0x90]; // A
const FONT_B: [u8; 5] = [0xE0, 0x90, 0xE0, 0x90, 0xE0]; // B
const FONT_C: [u8; 5] = [0xF0, 0x80, 0x80, 0x80, 0xF0]; // C
const FONT_D: [u8; 5] = [0xE0, 0x90, 0x90, 0x90, 0xE0]; // D
const FONT_E: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0xF0]; // E
const FONT_F: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0x80]; // F

// f: 1111_0000 1000_0000 1111_0000 1000_000 1000_0000


fn test_draw_display(d: &[u64; 32]) {
    for line_64 in d {
        println!("{:066b}", line_64);
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
        if i >= s.len() { break; }

        let byte = u16::from_be_bytes([s[i], s[i+1]]);
        print!("{:04x}, ", byte);

        i += 2;
    }
    print!("\n");
}

// fn push_stack(stack: &mut [u16; 16], stack_pointer: &mut u16, value: &u16) {
//     *stack_pointer += 1;

//     stack[(*stack_pointer) as usize] = *value;
// }

// fn pop_stack(stack_pointer: &mut u16) {
//     *stack_pointer -= 1;
// }

// bits 0-63
// 0 = most significant
// 63 = least
// if x > 63, panics

fn check_bit_64(row: u64, x: u8) -> bool {
    let shl = (63 - x);

    (row >> shl & 1_u64) == 1
}

fn set_bit_64(row: &mut u64, x: u8, v: bool) {
    let mask = 1_u64 << (63 - x);

    if v {
        *row |= mask;
    } else {
        *row = *row & !mask;
    }
}

// bits 0-7
// 0 = most significant
// 7 = least
// if x > 7, panics

fn check_bit_8(row: u8, x: u8) -> bool {
    let shl = (7 - x);

    (row >> shl & 1_u8) == 1
}

// the PC is at a location
// load two bytes from there
// then increment PC by two bytes

fn fetch(pc: &mut u16, chunk: &[u8; 4096]) -> u16 {
    let _pc = *pc as usize;

    let instruction = u16::from_be_bytes([chunk[_pc], chunk[_pc+1]]);

    *pc += 2;

    instruction
}




fn decode(instruction: &u16, d: &mut[u64; 32], r: &mut [u8; 16], i: &mut u16, pc: &mut u16, chunk: &[u8]) {
    // byte order big endian here

    let first_nibble = (instruction >> 12) & 0x000F;

    let x = (instruction >> 8) & 0x000F;
    let y = (instruction >> 4) & 0x000F;

    let n = (instruction) & 0x000F;
    let nn = (instruction) & 0x00FF;
    let nnn = (instruction) & 0x0FFF;

    match first_nibble {
        0x0 => {
            match nnn {
                0x0EE => {
                    db!("return from subroutine");

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
        0x6 => {
            db!("set register {:#x} to {:#x}", x, nn);

            let nn = nn.to_be_bytes()[1];

            r[x as usize] = nn;
        },
        0x7 => {
            db!("add to register {:#x}, value {:#x}", x, nn);

            let nn = nn.to_be_bytes()[1];

            r[x as usize] += nn;
        },
        0xA => {
            db!("set index register I, {:#x}", nnn);

            *i = nnn;

        },
        0xD => {

            db!("draw {n} rows of set I character at position x V{:#x} {:#x}, y V{:#x} {:#x}", x, r[x as usize], y, r[y as usize]);

            // get x and y

            let mut x = r[x as usize] % 64;
            let mut y = r[y as usize] % 32;

            r[0xF] = 0;

            for _i in (*i)..(*i+n) {

                if y == 32 { break; }

                // get the font data

                let data_row: u8 = chunk[_i as usize];

                // x -- + x
                // left -- right
                // more significant bit -- less significant bit


                
                for ix in 0..8 {


                    let _x = x + ix;
                    if _x == 64 { break; }

                    if check_bit_8(data_row, ix) {

                        if check_bit_64(d[y as usize], _x) {
                            db!("turn off pixel {_x}, {y} and set flag register to 1");
                
                            set_bit_64(&mut d[y as usize], _x, false);
                            
                            r[0xF] = 1;

                        } else {
                            db!("turn on pixel {_x}, {y}");

                            set_bit_64(&mut d[y as usize], _x, true);
                        }
                    }
                }

                y += 1;
            }

        },
        0xF => {
            print!("retreive font address of character V{:#x} {:#x} ... ", x, r[x as usize]);

            let address: u8 = 5 * r[x as usize];

            print!("{:#x}\n", address);

            *i = address as u16;
        },

        _ => panic!()
    }
}


fn main() {
    // initialize main chunk
    let mut chunk: [u8; 4096] = [0; 4096];

    chunk[ 0..5 ].copy_from_slice(&FONT_0);
    chunk[ 5..10].copy_from_slice(&FONT_1);
    chunk[10..15].copy_from_slice(&FONT_2);
    chunk[15..20].copy_from_slice(&FONT_3);
    chunk[20..25].copy_from_slice(&FONT_4);
    chunk[25..30].copy_from_slice(&FONT_5);
    chunk[30..35].copy_from_slice(&FONT_6);
    chunk[35..40].copy_from_slice(&FONT_7);
    chunk[40..45].copy_from_slice(&FONT_8);
    chunk[45..50].copy_from_slice(&FONT_9);
    chunk[50..55].copy_from_slice(&FONT_A);
    chunk[55..60].copy_from_slice(&FONT_B);
    chunk[60..65].copy_from_slice(&FONT_C);
    chunk[65..70].copy_from_slice(&FONT_D);
    chunk[70..75].copy_from_slice(&FONT_E);
    chunk[75..80].copy_from_slice(&FONT_F);


    let mut pc: u16 = 0x200;

    let instructions: [u16; 66] = [
0x00e0, 0xa22a, 0x600c, 0x6108, 0xd01f, 0x7009, 0xa239, 0xd01f,
0xa248, 0x7008, 0xd01f, 0x7004, 0xa257, 0xd01f, 0x7008, 0xa266,
0xd01f, 0x7008, 0xa275, 0xd01f, 0x1228, 0xff00, 0xff00, 0x3c00,
0x3c00, 0x3c00, 0x3c00, 0xff00, 0xffff, 0x00ff, 0x0038, 0x003f,
0x003f, 0x0038, 0x00ff, 0x00ff, 0x8000, 0xe000, 0xe000, 0x8000,
0x8000, 0xe000, 0xe000, 0x80f8, 0x00fc, 0x003e, 0x003f, 0x003b,
0x0039, 0x00f8, 0x00f8, 0x0300, 0x0700, 0x0f00, 0xbf00, 0xfb00,
0xf300, 0xe300, 0x43e0, 0x00e0, 0x0080, 0x0080, 0x0080, 0x0080,
0x00e0, 0x00e0];

    


    let mut counter = pc as usize;

    for ii in instructions {
        chunk[counter..counter+2].copy_from_slice(&ii.to_be_bytes());
        counter += 2;
    }

    test_print_slice_as_u16(&chunk[80..90]);



    // let mut stack: [u16; 16] = [0; 16];

    // // the type of this doesn't matter actually

    // let mut stack_top: u16 = 0;


    // 00E0 (clear screen)
    // 1NNN (jump)
    // 6XNN (set register VX)
    // 7XNN (add value to register VX)
    // ANNN (set index register I)
    // DXYN (display/draw)


    // FX29 (set I to font character in register VX)
    // 2NNN (enter subroutine NNN)
    // 00EE (exit subroutine)


    let mut display: [u64; 32] = [0; 32];

    let mut index_register: u16 = 0;

    let mut v0vf: [u8; 16] = [0; 16];







    // execution rate

    let seconds = Duration::from_secs(1);
    let mut start = SystemTime::now();

    // let timer_seconds = Duration::from_secs(2);
    // let mut timer_start = SystemTime::now();


    loop {
        match start.elapsed() {
            Ok(elapsed) if elapsed > seconds => {
                println!("elepsed 1");
                start = SystemTime::now();

                // execute 1
                let instruction = fetch(&mut pc, &chunk);

                decode(&instruction, &mut display, &mut v0vf, &mut index_register, &mut pc, &mut chunk);

                test_draw_display(&display)
            }
            _ => (),
        }
    }


    // exit loop?
    // println!("Hello, world!");
}



    // rust requires that slices are indexed with the system pointer
    // which makes sense
    // because they're elements that are next to eachother in system memory


// https://stackoverflow.com/questions/44690439/how-do-i-print-an-integer-in-binary-with-leading-zeros
// https://doc.rust-lang.org/rust-by-example/attribute/cfg.html
