
#![no_std] // Prevents linking the Rust Standard Library
#![no_main] // Disables all Rust-Level Entry Points

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

mod vga_buffer;

#[no_mangle] // Do not use mangling on the name of this function
pub extern "C" fn _start() -> ! {
    // This function is the entry point since the linker looks for a function named _start by default

    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// This will be called on panic.
#[panic_handler] 
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
