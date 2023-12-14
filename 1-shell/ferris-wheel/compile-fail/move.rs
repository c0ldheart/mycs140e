// FIXME: Prevent this file from compiling! Diff budget: 1 line.
// #[derive(Clone, Copy)] Remove the #derive-s of `MyType', so multiple moves make the program not compiling.
struct MyType(usize);

// Note: do not modify this function.
fn main() {
    let x = MyType(10);
    let y = x;
    let z = x;
}
