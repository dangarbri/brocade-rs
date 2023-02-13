use std::error::Error;
use product::Product;
pub mod product;

type Getter = fn(url: &str) -> Result<String, Box<dyn Error>>;

pub struct Brocade {
    httpget: Getter
}

impl Brocade {
    pub fn new(getter: Getter) -> Brocade {
        Brocade { httpget: getter }
    }

    pub fn get(&self, barcode: &str) -> Result<Product, Box<dyn Error>> {
        let url = String::from("https://www.brocade.io/api/items/");
        let result = (self.httpget)(&(url + barcode))?;
        Ok(serde_json::from_str(&result)?)
    }
}
