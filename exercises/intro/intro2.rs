// intro2.rs
// Make the code print a greeting to the world.
// Execute `rustlings hint intro2` or use the `hint` watch subcommand for a hint.


fn main() {
    let arg2:&str = "world"; // 最开始没加 &。实际上要是切片
    println!("Hello {}!", arg2);
}
