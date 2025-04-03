#![no_std] // No standard library
#![no_main] // No main function
#![feature(abi_x86_interrupt)] // Enable x86 interrupts

use core::panic::PanicInfo;

/// Entry point (called from bootloader)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Display message
    let vga_buffer = 0xb8000 as *mut u8;
    unsafe {
        *vga_buffer.offset(0) = b'H';
        *vga_buffer.offset(1) = 0x0f;
    }
    loop {}
}

/// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
