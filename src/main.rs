// total 4096 bytes (000-FFF)
// 
// first 512 are empty (000-1FF)
// 
// except for the font

// the drawing instruction needs
// a sprite (1-15 bytes)
// the screen buffer
// a position?


// the main loop should execute at around 700 instructions per second

// the timer should loop independently and should decrement at
// 60 times a second






// https://en.wikipedia.org/wiki/Nibble



// a nibble is half a byte (0-F)

// an instruction is two bytes and thus four nibbles (0000-FFFF)


// instructions are separated
// into broad catergories based on their first nibble (half octet)






// 2-byte program counter
// 16 registers (V0-VF)

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



fn test_draw_sprite() {

}

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



// the PC is at a location
// load two bytes from there
// then increment PC by two bytes

fn fetch(pc: &u16) {

}



// would it be better to use a tuple?

// use macros

// from_le_bytes
// from_be_bytes

// debug print off
// macro


// could put the importand stuff in a struct
// "state"

// the "as usize" bother me


fn decode(instruction: &u16, d: &mut[u64; 32], r: &mut [u8; 16], i: &mut u16) {
    // byte order big endian here

    let first_nibble = (instruction >> 12) & 0x000F;

    let x = (instruction >> 8) & 0x000F;
    let y = (instruction >> 4) & 0x000F;

    let n = (instruction) & 0x000F;
    let nn = (instruction) & 0x00FF;
    let nnn = (instruction) & 0x0FFF;

    match first_nibble {
        0x0 => {
            // E0
            println!("clear screen");

            *d = [0_u64; 32];
            // EE

        },
        0x1 => {
            println!("jump {:#x}", nnn);


        },
        0x6 => {
            println!("set register {:#x} to {:#x}", x, nn);

            let nn = nn.to_be_bytes()[1];

            r[x as usize] = nn;
        },
        0x7 => {
            println!("add to register {:#x}, value {:#x}", x, nn);

            let nn = nn.to_be_bytes()[1];

            r[x as usize] += nn;
        },
        0xA => {
            println!("set index register I, {:#x}", nnn);

            *i = nnn;

        },
        0xD => {
            print!("display / draw ... ");
            print!("x: V{:#x} {:#x}, y: V{:#x} {:#x} .. ", x, r[x as usize], y, r[y as usize]);
            print!("n: {:#x} .. ", n);
            print!("I: {:#x} .. ", i);
            print!("\n");

        },
        0xF => {
            print!("retreive font address of character V{:#x} {:#x} ... ", x, r[x as usize]);

            let address = 5 * r[x as usize];


            print!("{:#x}\n", address);
            // set i to address


        },

        _ => panic!()
    }
}


fn main() {
    // initialize main chunk
    let mut chunk: [u8; 4096] = [0; 4096];


    // memcopy here

    chunk[  0..5].copy_from_slice(&FONT_0);
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



    test_print_slice(&chunk[0x4B..0x50]);



    let mut pc: u16 = 0;




    // 00E0 (clear screen)
    // 1NNN (jump)
    // 6XNN (set register VX)
    // 7XNN (add value to register VX)
    // ANNN (set index register I)
    // DXYN (display/draw)


    // FX29 (font character in register VX)


    // load font

    let mut display: [u64; 32] = [0; 32];

    let mut index_register: u16 = 0;

    let mut v0vf: [u8; 16] = [0; 16];


    decode(&0x00E0, &mut display, &mut v0vf, &mut index_register);

    // decode(&0x1777);

    decode(&0x6341, &mut display, &mut v0vf, &mut index_register);

    decode(&0x7233, &mut display, &mut v0vf, &mut index_register);

    decode(&0xA765, &mut display, &mut v0vf, &mut index_register);

    decode(&0xD231, &mut display, &mut v0vf, &mut index_register);
    

    decode(&0x600F, &mut display, &mut v0vf, &mut index_register);    
    decode(&0xF029, &mut display, &mut v0vf, &mut index_register);




    test_print_registers(&v0vf);


    // test_draw_display(&display);


    // stack is used to call and return from subroutines
    // stack memory can be outside of the emulated memory

    // a list of 16-bit memory addresses

    //  how big are the stack frames

    let stack: [u16; 16] = [0; 16];
    let stack_top: u8 = 0;

    // stack push, stack pop etc

    // execution rate
    let seconds = Duration::from_secs(1);
    let mut start = SystemTime::now();

    // timer rate
    // timer timer


    // loop {
    //     match start.elapsed() {
    //         Ok(elapsed) if elapsed > seconds => {
    //             println!("elepsed 1");
    //             start = SystemTime::now();
    //         }
    //         _ => (),
    //     }
    // }

    // exit loop?
    // println!("Hello, world!");
}



    // rust requires that slices are indexed with the system pointer
    // which makes sense
    // because they're elements that are next to eachother in system memory


// https://stackoverflow.com/questions/44690439/how-do-i-print-an-integer-in-binary-with-leading-zeros
// https://doc.rust-lang.org/rust-by-example/attribute/cfg.html
