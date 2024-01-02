
#![feature(negative_impls)]
#![feature(decl_macro)]
#![feature(allocator_api)]
#![feature(lang_items)]
#![no_std]
extern crate std;

extern crate pi;
extern crate xmodem;

use pi::uart::MiniUart;

pub mod mutex;
pub mod console;

pub mod lang_items;

use core::arch::asm;
use std::io::ErrorKind;

use std::io::Cursor;

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
        // loop {
        //     asm!("nop" :::: "volatile")
        // }
        asm!("br {0}", in(reg) addr);
        loop { asm!("nop", options(nomem, nostack, preserves_flags)) }
    }
    
}

#[no_mangle]
pub extern "C" fn kmain() {
    // FIXME: Implement the bootloader.
    // ALLOCATOR.initialize();
    let mut uart = MiniUart::new();
    uart.set_read_timeout(750);

    loop {
        let dest = unsafe { std::slice::from_raw_parts_mut(BINARY_START, MAX_BINARY_SIZE) };
        match xmodem::Xmodem::receive(&mut uart, Cursor::new(dest)) {
            Ok(_) => {
                // Succeed
                jump_to(BINARY_START)
            }
            Err(err) => match err.kind() {
                ErrorKind::TimedOut => continue,
                ErrorKind::InvalidData => continue, // might receive 0x00 when no input
                // _ => uart.write_str(format_args!("Error: {:?}\r\n", err))
                //     .unwrap(),
                _ => {}
            },
        }
    }
}
