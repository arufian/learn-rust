fn main() {
    say_hello();
    let x = 2;
    let y = 5;
    print_sum(x, y);
    say_anumber(x as i32);

    // challenge
    let celcius_temp = 23.0;
    let fahrenheit_tmp = celcius_to_fahrenheit(celcius_temp);

    assert_eq!(fahrenheit_tmp, 73.4);
    println!("Test passed!");
}

fn celcius_to_fahrenheit(celcius: f64) -> f64 {
    return (celcius * 1.8) + 32.0;
}

fn say_hello() {
    println!("Hello you!");
    say_anumber(17);
}

fn say_anumber(number: i32) {
    println!("number is {} ", number);
}

fn print_sum(x: u8, y: u8) {
    println!("sum is {}", (x+y));
}
