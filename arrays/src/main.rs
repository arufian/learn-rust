fn main() {
    let mut letters: [char; 3]= ['a', 'b', 'c'];
    letters[0] = 'x';
    let first_letter = letters[0];
    println!("first letter is {}", first_letter);

    let numbers: [i32; 5];
    numbers = [0; 5];
    let index: usize = numbers.len();
    println!("last number is {}", numbers[index-1]);

    let parking_lot = [[1,2,3], [4,5,6]];
    let number = parking_lot[0][1];
    println!("number is {}", number);

    let garage: [[[i32; 100]; 20]; 5];
    garage = [[[0; 100]; 20]; 5];
    println!("garage 2nd floor, row 3, spot 1 is {}", garage[1][3][1]);
}
