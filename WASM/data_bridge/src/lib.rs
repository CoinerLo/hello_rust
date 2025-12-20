pub use mem::{malloc, free};

mod mem {
    /// Выделяет память заданного размера в байтах и возвращает указатель на первый байт.
    /// Данную функцию нужно использовать из JS для передачи данных через линейную память.
    #[unsafe(no_mangle)]
    pub extern "C" fn malloc(l: usize) -> *mut u8 {
        let mem: Vec<u8> = Vec::with_capacity(l);
        mem.leak().as_mut_ptr()
    }

    /// Очищает память по заданному указателю.
    /// Данную функцию нужно использовать из JS для очистки данных в линейной памяти.
    #[unsafe(no_mangle)]
    pub extern "C" fn free(ptr: *mut usize) {
        let h: Vec<usize> = unsafe { Vec::from_raw_parts(ptr, 2, 2) };

        assert_eq!(h.len(), 2);

        let _: Vec<u8> = unsafe {
            let ptr = h[0] as *mut u8;
            Vec::from_raw_parts(ptr, h[1], h[1])
        };
    }
    
    /// Упаковывает заданный вектор для передачи через Wasm Bridge.
    /// Данную функцию нужно использовать в коде на Rust для передачи векторов в JS.
    pub fn pack_vec<T>(val: Vec<T>) -> *const usize {
        let mut h = vec![val.as_ptr() as usize, val.len()];
        h.shrink_to_fit(); // минимально необходимая память
        std::mem::forget(val);
        h.leak().as_ptr()
    }
    
    /// Распаковывает значение вектора по указателю и длине.
    /// Данную функцию нужно использовать в коде на Rust
    /// для преобразования данных из линейной памяти в Rust вектор.
    pub fn unpack_vec<T>(ptr: *mut T, len: usize) -> Vec<T> {
        unsafe { Vec::from_raw_parts(ptr, len, len) }
    }
    
    /// Распаковывает значение вектора по указателю.
    /// Данную функцию нужно использовать в коде на Rust
    /// для преобразования данных из линейной памяти в Rust вектор.
    pub fn unpack_vec_header<T>(ptr: *mut usize) -> Vec<T> {
        let h = unsafe { Vec::from_raw_parts(ptr, 2, 2) };
        unpack_vec(h[0] as *mut T, h[1])
    }

    /// Упаковывает заданную строку для передачи через Wasm Bridge.
    /// Данную функцию нужно использовать в коде на Rust для передачи строк в JS.
    pub fn pack_str(val: &str) -> *const usize {
        let s = val.to_string();
        let mut h = vec![s.as_ptr() as usize, val.len()];
        h.shrink_to_fit();
        std::mem::forget(s);
        h.leak().as_ptr()
    }
    
    /// Распаковывает значение строки по указателю и длине.
    /// Данную функцию нужно использовать в коде на Rust
    /// для преобразования данных из линейной памяти в Rust строку.
    pub fn unpack_str(ptr: *mut u8, len: usize) -> String {
        unsafe { String::from_raw_parts(ptr, len, len) }
    }
    
    /// Распаковывает значение строки по указателю.
    /// Данную функцию нужно использовать в коде на Rust
    /// для преобразования данных из линейной памяти в Rust строку.
    pub fn unpack_str_header(ptr: *mut usize) -> String {
        let h = unsafe { Vec::from_raw_parts(ptr, 2, 2) };
        unpack_str(h[0] as *mut u8, h[1])
    }
    
    /// Упаковывает заданный вектор строк для передачи через Wasm Bridge.
    /// Данную функцию нужно использовать в коде на Rust для передачи вектора строк в JS.
    pub fn pack_vec_str(v: Vec<&str>) -> *const usize {
        let mut r = Vec::with_capacity(v.len());
        for s in v {
            r.push(pack_str(s));
        }
        pack_vec(r)
    }
}

/// Передает строку в JS
#[unsafe(no_mangle)]
pub extern "C" fn get_str() -> *const usize {
    mem::pack_str("Hello, word!")
}

/// Принимает строку из JS, модифицирует и возвращает результат
#[unsafe(no_mangle)]
pub extern "C" fn mod_str(ptr: *mut u8, len: usize) -> *const usize {
    let str = mem::unpack_str(ptr, len) + " (it works!)";
    mem::pack_str(&str)
}

/// Передает вектор строк в JS
#[unsafe(no_mangle)]
pub extern "C" fn get_strs() -> *const usize {
    mem::pack_vec_str(vec!["hello", "world"])
}

/// Передает вектор f32 чисел в JS
#[unsafe(no_mangle)]
pub extern "C" fn get_slice_f32() -> *const usize {
    mem::pack_vec(vec![1f32, 2.42, 3.13])
}

/// Принимает вектор f32 чисел из JS, модифицирует и возвращает результат
#[unsafe(no_mangle)]
pub extern "C" fn mod_vec_f32(ptr: *mut f32, len: usize) -> *const usize {
    let mut vec = mem::unpack_vec(ptr, len);
    vec.push(42f32);
    mem::pack_vec(vec)
}

/// Передает вектор i64 чисел в JS
#[unsafe(no_mangle)]
pub extern "C" fn get_slice_i64() -> *const usize {
    mem::pack_vec(vec![1i64, 2, i64::MAX])
}
