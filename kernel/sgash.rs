/* kernel::sgash.rs */

use core::*;
use core::str::*;
use core::option::{Some, Option, None}; // Match statement
use core::iter::Iterator;
use kernel::*;
use kernel::cstr::cstr;
use kernel::constants::{PROMPT, SPLASH};
use super::super::platform::*;
use kernel::memory::Allocator;

pub static mut buffer: cstr = cstr {
                p: 0 as *mut u8,
                p_cstr_i: 0,
                max: 0
            };

pub fn putchar(key: char) {
    unsafe {
        /*
         * We need to include a blank asm call to prevent rustc
         * from optimizing this part out
         */
        asm!("");
        io::write_char(key, io::UART0);
    }
}

fn putstr(msg: &str) {
    for c in slice::iter(as_bytes(msg)) {
        putchar(*c as char);
    }
}

pub unsafe fn drawstr(msg: &str) {
    //let old_fg = super::super::io::FG_COLOR;
    //let mut x: u32 = 0x6699AAFF;
    for c in slice::iter(as_bytes(msg)) {
        //x = (x << 8) + (x >> 24); 
        //super::super::io::set_fg(x);
        drawchar(*c as char);
    }
    //super::super::io::set_fg(old_fg);
}

pub unsafe fn drawcstr(msg: cstr) {
    //let old_fg = super::super::io::FG_COLOR;
    //let mut x: u32 = 0x6699AAFF;
    let mut ii=0; 
    while ii<msg.p_cstr_i {
        //x = (x << 8) + (x >> 24); 
        //super::super::io::set_fg(x);
        drawchar(*(((msg.p as uint) + ii) as *char)); 
        ii+=1; 
    }
  
    //super::super::io::set_fg(old_fg);

}

pub unsafe fn putcstr(s: cstr)
{
    let mut p = s.p as uint;
    while *(p as *char) != '\0' {
        putchar(*(p as *char));
        p += 1;
    }
}

pub unsafe fn parsekey(x: char) {
    let x = x as u8;
    // Set this to false to learn the keycodes of various keys!
    // Key codes are printed backwards because life is hard
        
    if (true) {
        match x { 
            13      =>  {
                        putstr(&"\n"); 
                        drawstr(&"\n");
                        parse();
                        prompt(false); 
            }
            127     =>  { 
                if (buffer.delete_char()) { 
                    putchar('');
                    putchar(' ');
                    putchar(''); 
                    backspace();
                }
            }
            _       =>  { 
                if (buffer.add_char(x)) { 
                    putchar(x as char);
                    drawchar(x as char);
                }
            }
        }
    }
    else {
        keycode(x);
    }
}

unsafe fn drawchar(x: char)
{
    if x == '\n' {
        io::CURSOR_Y += io::CURSOR_HEIGHT;
        io::CURSOR_X = 0u32;
        return;
    }

    io::restore();
    io::draw_char(x);
    io::CURSOR_X += io::CURSOR_WIDTH;
    if io::CURSOR_X >= io::SCREEN_WIDTH {io::CURSOR_X -= io::SCREEN_WIDTH; io::CURSOR_Y += io::CURSOR_HEIGHT}
    io::backup();
    io::draw_cursor();
}

unsafe fn backspace()
{
    io::restore();
    io::CURSOR_X -= io::CURSOR_WIDTH;
    io::draw_char(' ');
    io::backup();
    io::draw_cursor();
}

fn keycode(x: u8) {
    let mut x = x;
    while  x != 0 {
        putchar((x%10+ ('0' as u8) ) as char);
        x = x/10;
    }
    putchar(' ');
}

fn screen() {
    
    putstr(SPLASH);

}

pub unsafe fn init() {
    buffer = cstr::new(256);
    screen();
    prompt(true);
}

unsafe fn prompt(startup: bool) {
    putstr(PROMPT);
    if !startup {drawstr(PROMPT);}

    buffer.reset();
}

unsafe fn parse() {
    let (mut cmdname, args) = buffer.split(' ');
    match cmdname.as_str() {
        "echo" => {
                putcstr(args);
                drawcstr(args);
            },
        "ls" => {
            },
        "cat" => {
            },
        "cd" => {
            },
        "rm" => {
            },
        "mkdir" => {
            },
        "pwd" => {
            },
        "wr" => {
            },
        _ => {
                putstr(&"Unrecognized Command"); 
                drawstr(&"Unrecognized Command");
            }
    }; 
    buffer.reset();
}

