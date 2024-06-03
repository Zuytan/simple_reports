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
pub struct Sender {
    pub name: String,
    pub addr1: String,
    pub addr2: String,
    pub postal_code: u32,
    pub city: String,
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
}
