use super::product::Product;

#[derive(Clone, Default)]
pub struct Item<'a> {
    product: &'a Product,
    price: f64,
    quantity: u64,
}

impl<'a> Item<'a> {
    pub fn new(product: &'a Product, price: f64, quantity: u64) -> Self {
        return Self {
            product,
            price,
            quantity,
        };
    }
    pub fn product(&self) -> &'a Product {
        return self.product;
    }
    pub fn price(&self) -> &f64 {
        return &self.price;
    }
    pub fn quantity(&self) -> &u64 {
        return &self.quantity;
    }
}
