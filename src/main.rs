
#![no_std] // Prevents linking the Rust Standard Library
#![no_main] // Disables all Rust-Level Entry Points

use core::panic::PanicInfo;


mod vga_buffer;

#[no_mangle] // Do not use mangling on the name of this function
pub extern "C" fn _start() -> ! {
    // This function is the entry point since the linker looks for a function named _start by default

    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    loop {}
}

// This will be called on panic.
#[panic_handler] 
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
