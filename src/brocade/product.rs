use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Product {
    pub gtin14: String,
    pub brand_name: Option<String>,
    pub name: Option<String>,
    pub fat: Option<String>,
    pub fiber: Option<String>,
    pub sodium: Option<String>,
    pub sugars: Option<String>,
    pub protein: Option<String>,
    pub calories: Option<String>,
    pub trans_fat: Option<String>,
    pub saturated_fat: Option<String>,
    pub cholesterol: Option<String>,
    pub ingredients: Option<String>,
    pub carbohydrate: Option<String>,
    pub fat_calories: Option<String>,
    pub serving_size: Option<String>,
    pub servings_per_container: Option<String>
}