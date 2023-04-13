// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y), // 看了 hint 才知道。https://doc.rust-lang.org/std/keyword.ref.html 里有对比。不知道能不能用 & 做。
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}
