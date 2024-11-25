// Unsafe
//
// 1. Dereference raw pointers
// 2. Call unsafe methods/functions
// 3. Access/modify a mutable static variable
// 4. Implement unsafe trait
// 5. Access fields of unions
//
// Does not disable the borrow checker or rust analizer
fn main() {
    let mut num = 5;

    // these ignore the borrowing rules
    // are not guarunteed to point to valid memory
    // are not automatically dropped
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // definitly not valid adderess
    //let random_ptr = 0x12345usize as *const i32;

    unsafe {
        println!("r1 has a value of {}", *r1);
        println!("r2 has a value of {}", *r2);
        *r2 = 9;
        println!("r2 has a value of {}", *r2);
    }
    println!("num has a value of {}", num);

    // 2. Call unsafe functions
    unsafe {
        dangerous();
    }

    // 3. Create a safe abstraction
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

unsafe fn dangerous() {
    // this is risky
    // basically an unsafe block inside here
}

use std::slice;
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len: usize = slice.len();
    let ptr: *mut i32 = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        return (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        );
    }
}
/*fn split_at_mut<'a>(slice: &'a mut [i32], mid: usize) -> (&'a mut [i32], &'a mut [i32]) {
    let length: usize = slice.len();
    assert!(mid <= length);
    return (&mut slice[..mid], &mut slice[mid..]);
}*/
