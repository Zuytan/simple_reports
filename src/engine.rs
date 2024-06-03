use std::{fs::File, io::BufWriter};

use crate::{
    generators::Report,
    models::{client::Client, product::Product},
    storage::{Storage, StorageType},
};

pub struct Engine {
    storage: StorageType,
    clients: Vec<Client>,
    products: Vec<Product>,
}

impl Engine {
    pub fn new(storage_type: StorageType) -> Result<Self, String> {
        let storage = storage_type.build();
        storage.init()?;
        let clients = storage.load::<Client>()?;
        let products = storage.load::<Product>()?;
        return Ok(Engine {
            storage: storage_type,
            clients,
            products,
        });
    }

    pub fn get_products(&self) -> &Vec<Product> {
        &self.products
    }

    pub fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    pub fn get_clients(&self) -> &Vec<Client> {
        &self.clients
    }

    pub fn add_client(&mut self, client: Client) {
        self.clients.push(client);
    }

    pub fn save(&mut self) {
        for client in &self.clients {
            self.storage.build().save(client);
        }
        for product in &self.products {
            self.storage.build().save(product);
        }
    }
    pub fn generate_report(&self, report: impl Report) {
        let pdf = report.generate();
        let mut buffer = BufWriter::new(File::create("report").unwrap());
        pdf.save(&mut buffer);
    }
}
