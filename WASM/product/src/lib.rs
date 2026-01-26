use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[repr(C)]
pub struct Product {
    sku: String,          // Артикул, например "TSH-BLU-M"
    price: f64,           // Цена за единицу
    quantity: u32,        // Количество
    in_stock: bool,       // Есть ли на складе
}

impl Product {
    pub fn new(sku: String, price: f64, quantity: u32, in_stock: bool) -> Product {
        Product { sku, price, quantity, in_stock }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        
        bytes.extend_from_slice(as_bytes(&self.sku.as_ptr()));
        bytes.extend_from_slice(as_bytes(&self.sku.len()));
        bytes.extend_from_slice(as_bytes(&self.price));
        bytes.extend_from_slice(as_bytes(&self.quantity));
        bytes.push(self.in_stock as u8);

        return bytes;

        fn as_bytes<T>(value: &T) -> &[u8] {
            let ptr = value as *const T as *const u8;
            let len = size_of::<T>();
            unsafe { std::slice::from_raw_parts(ptr, len) }
        }
    }
}

struct Order {
    order_id: u64,
    customer_name: String,
    created_at: String,   // ISO 8601, например "2025-01-14T13:45:00Z"
    items: Vec<Product>,
}

impl Order {
    /// Конструктор — создаёт новый заказ
    pub fn new(order_id: u64, customer_name: String) -> Order;

    /// Добавляет товар в заказ
    pub fn add_product(&mut self, sku: String, price: f64, quantity: u32, in_stock: bool);

    /// Количество позиций в заказе
    pub fn total_items_count(&self) -> usize;

    /// Общая стоимость (∑ price × quantity)
    pub fn total_value(&self) -> f64;

    /// Вариант А — возвращает массив объектов JS
    pub fn get_items_js(&self) -> JsValue;

    /// Вариант Б — возвращает бинарные данные
    pub fn get_items_binary(&self) -> Vec<u8>;
}
