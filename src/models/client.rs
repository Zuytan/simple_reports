use crate::storage::Savable;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Client {
    pub id: String,
    pub name: String,
    pub address1: String,
    pub address2: String,
    pub postal_code: String,
    pub city: String,
}

impl Savable for Client {
    fn to_fields(&self) -> Vec<(String, String)> {
        vec![
            ("id".to_string(), self.id.clone()),
            ("name".to_string(), self.name.clone()),
            ("address1".to_string(), self.address1.clone()),
            ("address2".to_string(), self.address2.clone()),
            ("postal_code".to_string(), self.postal_code.clone()),
            ("city".to_string(), self.city.clone()),
        ]
    }

    fn from_fields(fields: Vec<(String, String)>) -> Result<Self, String> {
        let mut id = None;
        let mut name = None;
        let mut address1 = None;
        let mut address2 = None;
        let mut postal_code = None;
        let mut city = None;

        for (key, value) in fields {
            match key.as_str() {
                "id" => id = Some(value),
                "name" => name = Some(value),
                "address1" => address1 = Some(value),
                "address2" => address2 = Some(value),
                "postal_code" => postal_code = Some(value),
                "city" => city = Some(value),
                _ => {}
            }
        }

        if let (
            Some(id),
            Some(name),
            Some(address1),
            Some(address2),
            Some(postal_code),
            Some(city),
        ) = (id, name, address1, address2, postal_code, city)
        {
            Ok(Client {
                id,
                name,
                address1,
                address2,
                postal_code,
                city,
            })
        } else {
            Err("Missing fields".to_string())
        }
    }

    fn savable_name() -> String {
        "client".to_string()
    }
}
