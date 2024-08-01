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

// the timer should execute independently and should decrement at
// 60 times a second




// instructions are 2 bytes long and are separated
// into broad catergories based on their first nibble (half octet)

// right?


// https://en.wikipedia.org/wiki/Nibble



// a nibble is half a byte (0-F)

// an instruction is two bytes and thus four nibbles (0000-FFFF)







// 2-byte program counter
// 16 registers (V0-VF)

use std::time::{Duration, SystemTime};



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
    for i in d {
        println!("{} ", i);
    }
}

// load two bytes from chunk
fn fetch(pc: &u16) {

}

// would it be better to use a tuple?

// use macros

// from_le_bytes
// from_be_bytes

fn decode(instruction: &u16) {
    let first_nibble = (instruction >> 12) & 0x000F;
    let x = (instruction >> 8) & 0x000F;
    let y = (instruction >> 4) & 0x000F;

    // println!("{:#x}", first_nibble);


    // x
    // y

    // n
    // nn
    // nnn

    let n = (instruction) & 0x000F;
    let nn = (instruction) & 0x00FF;
    let nnn = (instruction) & 0x0FFF;
    // println!("{:#x}", nnn);

    match first_nibble {
        0x0 => {
            println!("clear screen");
        },
        0x1 => {
            println!("jump {:#x}", nnn);
        },
        0x6 => {
            println!("set register {:#x} to {:#x}", x, nn);
        },
        0x7 => {
            println!("add to register {:#x} value {:#x}", x, nn);
        },
        0xA => {
            println!("set index register I, {:#x}", nnn);
        },
        0xD => {
            println!("display / draw x:{:#x} y:{:#x} n:{:#x}", x, y, n);
        },
        _ => panic!()
    }
}


fn main() {
    // initialize main chunk

    let mut chunk: [u8; 4096] = [0; 4096];

    // program counter - PC

    let mut pc: u16 = 0;



    // let instruction = [chunk[(pc as usize)], chunk[(pc as usize)+1]];
    // pc += 2;


    // 00E0 (clear screen)
    // 1NNN (jump)
    // 6XNN (set register VX)
    // 7XNN (add value to register VX)
    // ANNN (set index register I)
    // DXYN (display/draw)


    decode(&0x00E0);
    decode(&0x1777);
    decode(&0x6344);
    decode(&0x7233);
    decode(&0xA765);
    decode(&0xD231);

    



    // load font

    let mut display: [u64; 32] = [0; 32];


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

