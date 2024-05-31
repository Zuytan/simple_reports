use crate::storage::Savable;

// Définir une structure pour un objet acheté
#[derive(Default, Clone, PartialEq)]
pub struct Product {
    id: String,
    description: String,
    reference: String,
}

impl Product {
    pub fn new(id: &str, description: &str, reference: &str) -> Self {
        return Product {
            id: id.to_owned(),
            description: description.to_owned(),
            reference: reference.to_owned(),
        };
    }
    pub fn id(&self) -> &String {
        return &self.id;
    }
    pub fn description(&self) -> &String {
        return &self.description;
    }
    pub fn reference(&self) -> &String {
        return &self.reference;
    }
    pub fn id_mut(&mut self) -> &mut String {
        return &mut self.id;
    }
    pub fn description_mut(&mut self) -> &mut String {
        return &mut self.description;
    }
    pub fn reference_mut(&mut self) -> &mut String {
        return &mut self.reference;
    }
}

impl Savable for Product {
    fn savable_name() -> String
    where
        Self: Sized,
    {
        return "Product".to_owned();
    }

    fn to_fields(&self) -> Vec<(String, String)>
    where
        Self: Sized,
    {
        return vec![
            ("id".to_owned(), self.id.clone()),
            ("description".to_owned(), self.description.clone()),
            ("reference".to_owned(), self.reference.clone()),
        ];
    }

    fn from_fields(fields: Vec<(String, String)>) -> Result<Self, String>
    where
        Self: Sized,
    {
        if fields.len() == 3 {
            let id = fields.get(0).unwrap();
            let description = fields.get(1).unwrap();
            let reference = fields.get(2).unwrap();
            if id.0 == "id" && description.0 == "description" && reference.0 == "reference" {
                return Ok(Self {
                    id: id.1.clone(),
                    description: description.1.clone(),
                    reference: reference.1.clone(),
                });
            }
        }
        return Err("Cannot create Product from fields".to_string());
    }
}
