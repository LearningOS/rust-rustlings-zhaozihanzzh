// move_semantics1.rs
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand for a hint.


fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);
    // println!("{}", vec0.len());
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> { // 返回只返回类型，不返回是否可变
    let mut vec = vec; // 可以把普通绑定移动到可变绑定上？

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
