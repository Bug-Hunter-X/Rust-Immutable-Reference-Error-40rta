fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // z is a mutable reference to x
    *y += 1; // Modifying x through y is allowed
    *z += 1; //Modifying x through z is also allowed because it is mutable 
    println!("x = {}", x); //Prints x = 7
}
