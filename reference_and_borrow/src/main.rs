fn main() {
    println!("Hello, world!");
}

// reference 
// String : pointer in stack, value in heap, reference in stack(point to pointer)
// &
// reference : mut and immut
// mut only one
// immut many is ok
// dangling reference -> none pointer !! never happen(because of rust's compiler )

// borrow
// happen in function argument