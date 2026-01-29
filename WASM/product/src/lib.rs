use gloo_utils::format::JsValueSerdeExt;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use chrono::{Utc, SecondsFormat};

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

#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
struct Order {
    order_id: u64,
    customer_name: String,
    created_at: String,   // ISO 8601, например "2025-01-14T13:45:00Z"
    items: Vec<Product>,
}

#[wasm_bindgen]
impl Order {
    /// Конструктор — создаёт новый заказ
    #[wasm_bindgen(constructor)]
    pub fn new(order_id: u64, customer_name: String) -> Order {
        Order {
            order_id,
            customer_name: customer_name.to_string(),
            created_at: Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
            items: Vec::new(),
        }
    }

    /// Добавляет товар в заказ
    #[wasm_bindgen(js_name = addProduct)]
    pub fn add_product(&mut self, sku: String, price: f64, quantity: u32, in_stock: bool) {
        self.items.push(Product::new(sku.to_string(), price, quantity, in_stock));
    }

    /// Количество позиций в заказе
    #[wasm_bindgen(js_name = totalItemsCount)]
    pub fn total_items_count(&self) -> usize {
        self.items.len()
    }

    /// Общая стоимость (∑ price × quantity)
    #[wasm_bindgen(js_name = totalValue)]
    pub fn total_value(&self) -> f64 {
        self.items.iter().map(|item| item.price).sum()
    }

    /// Вариант А — возвращает массив объектов JS
    #[wasm_bindgen(js_name = getItemsJS)]
    pub fn get_items_js(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.items).unwrap()
    }

    #[wasm_bindgen(js_name = getItemsJSON)]
    pub fn get_items_json(&self) -> JsValue {
        JsValue::from_serde(&self.items).unwrap()
    }

    /// Вариант Б — возвращает бинарные данные
    #[wasm_bindgen(js_name = getItemsBinary)]
    pub fn get_items_binary(&self) -> Vec<u8> {
        self.items.iter().flat_map(|item| item.encode()).collect()
    }

    #[wasm_bindgen(js_name = getItemsBinaryRaw)]
    pub fn get_items_binary_raw(&self) -> Vec<usize> {
        vec![self.items.as_ptr() as usize, self.items.len() * size_of::<Product>()]
    } 
}
