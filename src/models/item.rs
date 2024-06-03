use super::product::Product;

#[derive(Clone, Default)]
pub struct Item {
    product: Product,
    price: String,
    quantity: String,
}

impl Item {
    pub fn new(product: Product, price: String, quantity: String) -> Self {
        return Self {
            product,
            price,
            quantity,
        };
    }
    pub fn product(&self) -> &Product {
        return &self.product;
    }
    pub fn product_mut(&mut self) -> &mut Product {
        return &mut self.product;
    }
    pub fn price(&self) -> &String {
        return &self.price;
    }
    pub fn quantity(&self) -> &String {
        return &self.quantity;
    }
    pub fn quantity_mut(&mut self) -> &mut String {
        return &mut self.quantity;
    }
}
