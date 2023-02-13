pub mod brocade;
pub mod error;
use std::error::Error;
use brocade::Brocade;

use crate::error::ResponseError;
#[cfg(feature = "reqwest" )]
pub fn instance() -> Brocade {
    fn getter(url: &str) -> Result<String, Box<dyn Error>> {
        let result = reqwest::blocking::get(url)?;
        match result.text() {
            Ok(data) => Ok(data),
            Err(_) => Err(Box::new(ResponseError::new("An error occurred")))
        }
    }
    Brocade::new(getter)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reqwest() {
        let api = reqwest();
        let result = api.get("00076950450233");
        dbg!(result);
    }
}