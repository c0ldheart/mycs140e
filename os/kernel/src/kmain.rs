#![feature(lang_items)]
#![feature(core_intrinsics)]
// #![feature(const_fn)]
#![feature(asm)]
#![feature(auto_traits)]
#![feature(decl_macro)]
// #![feature(repr_align)] // stable since 1.25.0
#![feature(attr_literals)]
#![feature(never_type)]
#![feature(ptr_internals)]
#![feature(negative_impls)]
#![no_std]


// extern crate core;
extern crate pi;
extern crate stack_vec;
extern crate std;
pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;

use core::fmt::Write;


use console::_print;
use pi::{gpio::Gpio, timer, uart};

use crate::console::{kprint, noblock_kprintln};


#[no_mangle]
pub unsafe extern "C" fn kmain() {
    // FIXME: Start the shell.

    let mut gpio_19 = Gpio::new(19).into_output();
    gpio_19.set();
    timer::spin_sleep_ms(200);
    gpio_19.clear();
    let mut uart = uart::MiniUart::new(); // Move the creation of uart outside of the loop
    // uart.write_str("start!").unwrap();
    // loop {

    //     let readed = uart.read_byte();
    //     gpio_19.set();
    //     timer::spin_sleep_ms(100);
    //     gpio_19.clear();
    //     uart.write_byte(readed);
    //     uart.write_str("<->").unwrap();

    //     if readed == b'q' {
    //         uart.write_str("quit and use kprintln\n").unwrap();
    //         break;
    //     }
    // }
    // loop {
    //     // gpio_19.set();
    //     // timer::spin_sleep_ms(200);
    //     // gpio_19.clear();
    //     // timer::spin_sleep_ms(200);
    //     let readed = uart.read_byte();
    //     gpio_19.set();
    //     timer::spin_sleep_ms(100);
    //     gpio_19.clear();
    //     uart.write_byte(readed);
    //     uart.write_str("<-").unwrap();
    //     noblock_kprintln!("no block println");
    //     kprint!("hello\n");
    //     // gpio_19.set();
    // }

    loop {
        shell::shell("$ ");
    }
}
