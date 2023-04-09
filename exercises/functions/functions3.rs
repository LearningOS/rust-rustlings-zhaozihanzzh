// functions3.rs
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a hint.


fn main() {
    let neg = -200;
    call_me(neg as u8 as u32); // 直接传入 -1 不行，不能自动进行隐式转换；同时不能写 -200 as u8，原因未知
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
