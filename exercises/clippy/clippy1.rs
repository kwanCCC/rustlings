// clippy1.rs
// The Clippy tool is a collection of lints to analyze your code
// so you can catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy warnings
// Execute `rustlings hint clippy1` for hints :)

fn main() {
    let x = 1u64;
    let y = 1u64;
    if y != x {
        println!("Success!");
    }
}
