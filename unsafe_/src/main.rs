use core::slice;

static HELLO: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

unsafe fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

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

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v [..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    split_at_mut(r,2);

    // extern - взаимодействие с кодом на другом языке
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
    // Если функция помечена как safe в блоке extern
    // println!("Absolute value of -3 according to C: {}", abs(-3));

    // global var
    println!("name is: {HELLO}");
    unsafe {
        add_to_counter(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}

unsafe extern "C" {
    fn abs(input: i32) -> i32;
    // safe fn abs(input: i32) -> i32;
}

unsafe fn dangerous() {}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// создание кода который может использоваться в других языках
#[unsafe(no_mangle)] // эта нотация говорит компилятору не менять название функции при копилировании
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
