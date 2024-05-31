use std::sync::{Arc, Mutex};

use engine::Engine;
use gui::app::MyApp;
use models::{client::Client, product::Product};
use storage::{Storage, StorageType};

mod engine;
mod generators;
mod gui;
mod models;
mod storage;
// Définir une structure pour l'émetteur
struct Sender {
    name: String,
    addr1: String,
    addr2: String,
    postal_code: u32,
    city: String,
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let storage_type = StorageType::FileStorage("db");
    let engine = match Engine::new(storage_type) {
        Ok(e) => e,
        Err(e) => {
            println!("Engine failed to load: {}", e);
            return;
        }
    };
    let app = MyApp::new(Arc::new(Mutex::new(engine)));

    eframe::run_native("My App", native_options, Box::new(|cc| Box::new(app)));
    /*
    let sender = Sender {
        name: String::from("Émetteur Example"),
        addr1: String::from("456 Rue Exemple"),
        addr2: String::from(""),
        postal_code: 75002,
        city: "Paris".to_owned(),
    };

    let client = Client {
        id: "John Doe".to_string(),
        name: String::from("John Doe"),
        address1: String::from("123 Rue Exemple"),
        address2: String::from(""),
        postal_code: "75001".to_string(),
        city: "Paris".to_owned(),
    };

    let mcb = Product::new("002.166", "Carte Mère MyColisBox", "002.166");
    let ext = Product::new("002.167", "Carte d'extension - Carte Caisson", "002.167");

    let items = vec![Item::new(&mcb, 199., 5), Item::new(&ext, 150., 5)];

    match generate_delivery_note_pdf(
        &sender,
        &client,
        &items,
        Some("assets/logo.png"),
        "bon_de_livraison.pdf",
    ) {
        Ok(_) => println!("PDF généré avec succès !"),
        Err(e) => eprintln!("Erreur lors de la génération du PDF : {}", e),
    }*/
}
