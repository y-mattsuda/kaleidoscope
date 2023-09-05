use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(& mut n).expect("failed to read line");
    println!("define i32 @main() {{\n   ret i32 {n}}}")
}
