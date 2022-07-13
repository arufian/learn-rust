fn main() {
    let mut stuff: (u8, f32, char) = (10, 3.14, 'x');
    stuff.0 += 3;
    let first_item = stuff.0;
    println!("first item is {}", first_item);

    let (a, b ,c) = stuff;
    println!("a is {}, b is {}, c is {}", a, b, c);
}
