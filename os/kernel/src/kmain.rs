#![feature(panic_info_message)]
#![no_std]
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
#![feature(allocator_api, global_allocator)]

// extern crate core;
extern crate pi;
extern crate stack_vec;
extern crate std;

pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;
pub mod allocator;

use pi::{gpio::Gpio, timer, uart};


use allocator::Allocator;

#[global_allocator]
pub static ALLOCATOR: allocator::Allocator = Allocator::uninitialized();

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    // FIXME: Start the shell.
    ALLOCATOR.initialize();
    // let mut gpio_19 = Gpio::new(19).into_output();
    // gpio_19.set();
    // timer::spin_sleep_ms(200);
    // gpio_19.clear();

    loop {
        shell::shell("$ ");
    }
}
