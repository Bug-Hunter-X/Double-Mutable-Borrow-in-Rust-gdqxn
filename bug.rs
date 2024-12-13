fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // this is also mutable reference to x which should cause error
    *y = 6;
    *z = 7;
    println!("x = {}", x);
}