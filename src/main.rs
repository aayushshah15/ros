// Do not use the normal entry point chain (no C runtime)
#![no_main]
// Do not try to link the standard library
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
// Note: '!' indicates this is a diverging function (it never returns)
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Note: the 'no_mangle' attribute tells the compiler to output the function
// symbol without mangling with its name. We need this in order to tell the linker
// about our new entry point. 
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}