fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    // ... some operations that modify the vector ...
    unsafe {
        let len = v.len();
        let cap = v.capacity();
        let new_v = Vec::from_raw_parts(ptr, len, cap);
        println!("{:?}", new_v);
    }
}