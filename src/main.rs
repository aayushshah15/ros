// Do not use the normal entry point chain (no C runtime)
#![no_main]
// Do not try to link the standard library 
#![no_std]
#![feature(asm)]

use core::panic::PanicInfo;

#[panic_handler]
// Note: '!' indicates this is a diverging function (it never returns)
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello from Aayush";
// Note: the 'no_mangle' attribute tells the compiler to output the function
// symbol without mangling with its name. We need this in order to tell the linker
// about our new entry point. 
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buf = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        // The cells inside the vga buffer have two properties, the byte and its color.
        // '0xb' stands for cyan.
        unsafe {
            *vga_buf.offset(i as isize * 2) = byte;
            *vga_buf.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    
    loop {}
}