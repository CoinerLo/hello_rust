struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Droping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let _a = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _b = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created!");
}
