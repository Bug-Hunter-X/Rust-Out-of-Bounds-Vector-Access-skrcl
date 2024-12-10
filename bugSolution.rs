fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    // Check for index validity before accessing
    if vec.len() > 2 {
        let val = vec[2];
        println!("Value at index 2: {}", val);
    } else {
        println!("Index out of bounds");
    }
    //Alternative solution using get() method
    match vec.get(2) {
        Some(val) => println!("Value at index 2: {}", val),
        None => println!("Index out of bounds"),
    }
} 