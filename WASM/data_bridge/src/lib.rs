pub use mem::{malloc, free};

pub mem {
    /// Выделяет память заданного размера в байтах и возвращает указатель на первый байт.
    /// Данную функцию нужно использовать из JS для передачи данных через линейную память.
    #[unsafe(no_mangle)]
    pub extern "C" fn malloc(l: usize) -> *mut u8 {
        let mem: Vec<u8> = Vec::with_capacity(l);
        mem.leak().as_mut_ptr()
    }
}
