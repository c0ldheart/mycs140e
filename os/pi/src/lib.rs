// #![feature(core_intrinsics)]
// #![feature(const_fn)]
#![feature(asm)]
#![feature(decl_macro)]
// #![feature(repr_align)] // stable since 1.25.0
#![feature(attr_literals)]
#![feature(never_type)]

#![cfg_attr(not(feature = "std"), no_std)]
#![no_std]
// #[cfg(feature = "std")]
// extern crate core;
extern crate volatile;

extern crate std;
pub mod timer;
pub mod uart;
pub mod gpio;
pub mod common;
