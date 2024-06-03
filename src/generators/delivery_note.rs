use crate::models::client::Client;
use crate::models::item::Item;
use crate::Sender;
use image_crate::codecs::png::PngDecoder;
use printpdf::*;
use std::fs::File;

use super::Report;

pub struct DeliveryNote {
    sender: Sender,
    client: Client,
    items: Vec<Item>,
    logo_path: Option<String>,
}

impl DeliveryNote {
    pub fn new(sender: Sender, client: Client, items: &[Item], logo_path: Option<String>) -> Self {
        return Self {
            sender,
            client,
            items: items.to_vec(),
            logo_path,
        };
    }
}

impl Report for DeliveryNote {
    fn generate(&self) -> PdfDocumentReference {
        let (doc, page1, layer1) =
            PdfDocument::new("Bon de Livraison", Mm(210.0), Mm(297.0), "Layer 1");

        let current_layer = doc.get_page(page1).get_layer(layer1);

        // Ajouter des polices
        let font_bold = doc
            .add_external_font(File::open("assets/fonts/Helvetica-Bold.ttf").unwrap())
            .unwrap();
        let font = doc
            .add_external_font(File::open("assets/fonts/Helvetica.ttf").unwrap())
            .unwrap();

        // Ajouter le logo
        if let Some(logo_path) = &self.logo_path {
            let mut image_file = File::open(logo_path).unwrap();
            let image = Image::try_from(PngDecoder::new(&mut image_file).unwrap()).unwrap();

            // translate x, translate y, rotate, scale x, scale y
            // by default, an image is optimized to 300 DPI (if scale is None)
            // rotations and translations are always in relation to the lower left corner
            image.add_to_layer(
                current_layer.clone(),
                ImageTransform {
                    translate_x: Some(Mm(10.)),
                    translate_y: Some(Mm(250.)),
                    rotate: None,
                    scale_x: None,
                    scale_y: None,
                    dpi: None,
                },
            );
        }
        // Ajouter le titre
        current_layer.use_text("Bon de Livraison", 24.0, Mm(110.0), Mm(280.0), &font_bold);
        // Ajouter les informations de l'émetteur
        current_layer.use_text(
            format!("{}", self.sender.name),
            12.0,
            Mm(10.0),
            Mm(240.0),
            &font,
        );
        current_layer.use_text(
            format!("{}", self.sender.addr1),
            12.0,
            Mm(10.0),
            Mm(235.0),
            &font,
        );
        current_layer.use_text(
            format!("{}", self.sender.addr2),
            12.0,
            Mm(10.0),
            Mm(230.0),
            &font,
        );
        current_layer.use_text(
            format!("{} {}", self.sender.postal_code, self.sender.city),
            12.0,
            Mm(10.0),
            Mm(225.0),
            &font,
        );

        // Ajouter les informations du client à droite
        current_layer.use_text(
            format!("Pour: {}", self.client.name),
            12.0,
            Mm(120.0),
            Mm(230.0),
            &font,
        );
        current_layer.use_text(
            format!("{}", self.client.address1),
            12.0,
            Mm(120.0),
            Mm(225.0),
            &font,
        );
        current_layer.use_text(
            format!("{}", self.client.address2),
            12.0,
            Mm(120.0),
            Mm(220.0),
            &font,
        );
        current_layer.use_text(
            format!("{} {}", self.client.postal_code, self.client.city),
            12.0,
            Mm(120.0),
            Mm(215.0),
            &font,
        );

        // Ajouter une ligne de séparation
        current_layer.add_line(Line {
            points: vec![
                (Point::new(Mm(10.0), Mm(200.0)), false),
                (Point::new(Mm(200.0), Mm(200.0)), false),
            ],
            is_closed: false,
        });

        // Ajouter un tableau pour les objets achetés
        let mut y_position = 195.0;
        current_layer.use_text("Référence", 12.0, Mm(10.0), Mm(y_position), &font_bold);
        current_layer.use_text("Description", 12.0, Mm(50.0), Mm(y_position), &font_bold);
        current_layer.use_text("Quantité", 12.0, Mm(150.0), Mm(y_position), &font_bold);
        // Ajouter des lignes de séparation
        current_layer.add_line(Line {
            points: vec![
                (Point::new(Mm(10.0), Mm(y_position - 2.0)), false),
                (Point::new(Mm(200.0), Mm(y_position - 2.0)), false),
            ],
            is_closed: false,
        });

        y_position -= 10.0;

        for item in &self.items {
            current_layer.use_text(
                item.product().reference(),
                12.0,
                Mm(10.0),
                Mm(y_position),
                &font,
            );
            current_layer.use_text(
                item.product().description(),
                12.0,
                Mm(50.0),
                Mm(y_position),
                &font,
            );
            current_layer.use_text(
                item.quantity().to_string(),
                12.0,
                Mm(150.0),
                Mm(y_position),
                &font,
            );

            // Ajouter des lignes de séparation
            current_layer.add_line(Line {
                points: vec![
                    (Point::new(Mm(10.0), Mm(y_position - 2.0)), false),
                    (Point::new(Mm(200.0), Mm(y_position - 2.0)), false),
                ],
                is_closed: false,
            });

            y_position -= 10.0;
        }

        current_layer.add_rect(Rect {
            ll: Point::new(Mm(10.0), Mm(y_position - 20.)),
            ur: Point::new(Mm(100.0), Mm(y_position)),
            mode: path::PaintMode::Stroke,
            winding: path::WindingOrder::EvenOdd,
        });
        current_layer.use_text("Nom :", 12.0, Mm(15.0), Mm(y_position - 5.), &font);
        y_position -= 25.;
        current_layer.add_rect(Rect {
            ll: Point::new(Mm(10.0), Mm(y_position - 20.)),
            ur: Point::new(Mm(100.0), Mm(y_position)),
            mode: path::PaintMode::Stroke,
            winding: path::WindingOrder::EvenOdd,
        });
        current_layer.use_text("Visa :", 12.0, Mm(15.0), Mm(y_position - 5.), &font);
        return doc;
    }
}
