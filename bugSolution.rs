fn main() {
    let mut x = 5;
    { // create a new scope
        let y = &mut x; // y is a mutable reference to x
        *y = 6; 
    }
    { // another new scope
        let z = &mut x; // z is a mutable reference to x
        *z = 7; 
    }
    println!("x = {}", x);
}
