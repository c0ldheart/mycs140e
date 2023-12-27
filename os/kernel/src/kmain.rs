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

use pi::gpio::Gpio;


#[no_mangle]
pub unsafe extern "C" fn kmain() {
    // FIXME: Start the shell.


    let mut gpio_19 = Gpio::new(19).into_output();
    loop {
        gpio_19.set();
        pi::timer::spin_sleep_ms(200);
        gpio_19.clear();
        pi::timer::spin_sleep_ms(200);
    }
}
