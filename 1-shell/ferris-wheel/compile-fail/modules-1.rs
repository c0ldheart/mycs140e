// FIXME: Prevent this file from compiling! Diff budget: 1 line.

mod a {
    fn f() { } // Make `a::f' private prevents the program from compiling.
}

// Do not modify this function.
fn main() {
    a::f();
}
