// декларативные макрося для метапрограммирования
// они генерируют код на rust
// выполняют сопоставление и замену нашего кода другим кодом на rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ), * ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let _v: Vec<u32> = vec![1, 2, 3];
}
