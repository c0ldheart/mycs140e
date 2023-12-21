// FIXME: Make me compile! Diff budget: 2 lines.
// Do not modify this definition.
// 在 Rust 1.26 版本及更高版本中，i128 类型已经成为稳定特性
#![feature(i128_type)] 
enum Duration {
    MicroSeconds(u128),
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

fn main() { }
