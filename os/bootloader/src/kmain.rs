
#![feature(asm, lang_items)]
#![no_std]
extern crate core;
extern crate xmodem;
extern crate pi;
use core::arch::asm; // Import the asm macro from core::arch module
pub mod lang_items;

/// Start address of the binary to load and of the bootloader.
const BINARY_START_ADDR: usize = 0x80000;
const BOOTLOADER_START_ADDR: usize = 0x4000000;

/// Pointer to where the loaded binary expects to be laoded.
const BINARY_START: *mut u8 = BINARY_START_ADDR as *mut u8;

/// Free space between the bootloader and the loaded binary's start address.
const MAX_BINARY_SIZE: usize = BOOTLOADER_START_ADDR - BINARY_START_ADDR;

/// Branches to the address `addr` unconditionally.
fn jump_to(addr: *mut u8) -> ! {
    unsafe {
        // asm!("br $0" : : "r"(addr as usize));
        // loop { asm!("nop" :::: "volatile")  }

        // Rust 1.57.0 or higher
        asm!("br {0}", in(reg) addr);
        loop { asm!("nop", options(nomem, nostack, preserves_flags)) }
    }
}

#[no_mangle]
pub extern "C" fn kmain() {
    // FIXME: Implement the bootloader.
}
