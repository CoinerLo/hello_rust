fn main() {
    let mut num = 5;
    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        println!("r1 is : {}", *r1);
        println!("r2 is : {}", *r2);
    }

    let address = 0x012345usize;
    let _r = address as *const i32;
}
