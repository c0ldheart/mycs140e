// FIXME: Make me compile! Diff budget: 1 line.
// 在 Rust 中，{:?} 是用于调试输出的格式化打印指令。它可以用于打印出结构体、枚举、数组等类型的调试信息。
//  #[derive(Debug)] 注解自动实现了 Debug trait
#[derive(Debug, Clone, Copy)]
enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

pub fn main() {
    println!("Duration: {:?}", Duration::MilliSeconds(1200));

    let x = Duration::Minutes(10);
    let y = x;
    let z = x;
}
