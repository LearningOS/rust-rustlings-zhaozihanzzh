// variables2.rs
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a hint.


fn main() {
    let x: i32 = 0;
    let x = 0;
    // let x: i8 = 256; // 这样会报错超出范围
    let x : i16 = 256;
    let x : i8 = x as i8;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
