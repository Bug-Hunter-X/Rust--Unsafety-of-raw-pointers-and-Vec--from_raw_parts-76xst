fn main() {
    let mut v = vec![1, 2, 3];
    // Avoid raw pointers
    v.push(4);
    v.push(5);
    println!("{:?}", v);
} 