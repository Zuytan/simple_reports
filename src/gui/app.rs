use eframe::{egui, App, Frame};
use egui::{CentralPanel, TopBottomPanel};
use std::sync::{Arc, Mutex};

use crate::{
    engine::Engine,
    models::{client::Client, item::Item, product::Product},
};

#[derive(PartialEq)]
enum Tab {
    CreateProduct,
    CreateClient,
    GenerateDeliveryNote,
}
pub struct MyApp<'a> {
    tab: Tab,
    engine: Arc<Mutex<Engine>>,
    client: Arc<Mutex<Client>>,
    product: Arc<Mutex<Product>>,
    selected_items: Vec<Option<Item<'a>>>,
}

impl<'a> MyApp<'a> {
    pub fn new(engine: Arc<Mutex<Engine>>) -> Self {
        return Self {
            tab: Tab::GenerateDeliveryNote,
            engine,
            client: Arc::new(Mutex::new(Client::default())),
            product: Arc::new(Mutex::new(Product::default())),
            selected_items: Vec::new(),
        };
    }
    fn show_create_client(&mut self, ui: &mut egui::Ui) {
        ui.heading("Créer un Client");

        let mut client = self.client.lock().unwrap();

        ui.horizontal(|ui| {
            ui.label("ID:");
            ui.text_edit_singleline(&mut client.id);
        });
        ui.horizontal(|ui| {
            ui.label("Nom:");
            ui.text_edit_singleline(&mut client.name);
        });
        ui.horizontal(|ui| {
            ui.label("Adresse 1:");
            ui.text_edit_singleline(&mut client.address1);
        });
        ui.horizontal(|ui| {
            ui.label("Adresse 2:");
            ui.text_edit_singleline(&mut client.address2);
        });
        ui.horizontal(|ui| {
            ui.label("Code Postal:");
            ui.text_edit_singleline(&mut client.postal_code);
        });
        ui.horizontal(|ui| {
            ui.label("Ville:");
            ui.text_edit_singleline(&mut client.city);
        });

        if ui.button("Sauvegarder Client").clicked() {
            let mut engine = self.engine.lock().unwrap();
            engine.add_client(client.clone());
            *client = Client::default();
        }
    }
    fn show_create_product(&mut self, ui: &mut egui::Ui) {
        ui.heading("Créer un Produit");

        let mut product = self.product.lock().unwrap();

        ui.horizontal(|ui| {
            ui.label("ID:");
            ui.text_edit_singleline(product.id_mut());
        });
        ui.horizontal(|ui| {
            ui.label("Description:");
            ui.text_edit_singleline(product.description_mut());
        });
        ui.horizontal(|ui| {
            ui.label("Reference:");
            ui.text_edit_singleline(product.reference_mut());
        });

        if ui.button("Sauvegarder un produit").clicked() {
            let mut engine = self.engine.lock().unwrap();
            engine.add_product(product.clone());
            *product = Product::default();
        }
    }
    fn show_generate_report(&mut self, ui: &mut egui::Ui) {
        ui.heading("Créer un Rapport");

        let mut engine = self.engine.lock().unwrap();
        let products = engine.get_products();

        ui.label("Select Products:");
        for item in self.selected_items {
            ui.horizontal(|ui| {
                egui::ComboBox::from_id_source(format!("product_{}", item.product().id()))
                    .selected_text(item.product().description())
                    .show_ui(ui, |ui| {
                        for (index, product) in products.iter().enumerate() {
                            ui.selectable_value(
                                &mut item.product(),
                                &item.product(),
                                product.description(),
                            );
                        }
                    });
                if ui.button("+").clicked() {
                    self.selected_items.push(Item::default());
                }
                if ui.button("-").clicked() {
                    if self.selected_products.len() > 1 {
                        self.selected_products.remove(i);
                    }
                }
            });
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Create Product").clicked() {
                    self.tab = Tab::CreateProduct;
                }
                if ui.button("Create Client").clicked() {
                    self.tab = Tab::CreateClient;
                }
                if ui.button("Generate Delivery Note").clicked() {
                    self.tab = Tab::GenerateDeliveryNote;
                }
            });
        });

        CentralPanel::default().show(ctx, |ui| match self.tab {
            Tab::CreateProduct => self.show_create_product(ui),
            Tab::CreateClient => self.show_create_client(ui),
            Tab::GenerateDeliveryNote => self.show_generate_report(ui),
        });
    }
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}
}
