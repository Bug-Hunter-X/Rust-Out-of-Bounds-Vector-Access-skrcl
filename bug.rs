fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let val = vec[2]; // This will panic at runtime because index is out of bounds
    println!("Value at index 2: {}", val);
}