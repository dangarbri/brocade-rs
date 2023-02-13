pub mod brocade;
pub mod error;
use std::error::Error;
use crate::brocade::Brocade;

use crate::error::ResponseError;
#[cfg(feature = "reqwest" )]
pub fn instance() -> Brocade {
    fn getter(url: &str) -> Result<String, Box<dyn Error>> {
        let result = reqwest::blocking::get(url)?;
        if result.status().is_success() {
            match result.text() {
                Ok(data) => Ok(data),
                Err(_) => Err(Box::new(ResponseError::new("An error occurred")))
            }
        } else {
            return Err(Box::from(ResponseError::new(format!("No product information found").as_str())));
        }
    }
    Brocade::new(getter)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reqwest() {
        let api = instance();
        let result = api.get("00076950450233");
        assert_eq!(result.unwrap().brand_name.unwrap(), "Yogi");
    }

    #[test]
    fn test_no_product() {
        let api = instance();
        let result = api.get("00076950450232");
        dbg!(result);
    }
}