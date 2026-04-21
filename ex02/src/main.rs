fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

fn main() {
    println!("{}", gray_code(0));
    println!("{}", gray_code(1));
    println!("{}", gray_code(2));
    println!("{}", gray_code(3));
    println!("{}", gray_code(4));
    println!("{}", gray_code(5));
    println!("{}", gray_code(6));
    println!("{}", gray_code(7));
    println!("{}", gray_code(8));
}
