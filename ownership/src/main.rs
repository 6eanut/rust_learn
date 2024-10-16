// stack and heap
// stack : fast, fixed size
// heap : slow, allocate a space to a pointer
// ownership manage heap

// String -> pointer in stack, value in heap
// sclar, tuple, array in stack
// copy/clone -> stack , move -> copy/no copy
// '=' means stack operation
fn main() {
    let hello = String::from("hello");//hello pointer in stack, hello value in heap
    let hello0 = hello;//hello pointer move to hello0 pointer, hello pointer dead, hello0 pointer points hello value
    // println!("{hello}");
}//drop trait(for heap)
