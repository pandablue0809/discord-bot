use serde::Deserialize;

#[derive(Deserialize)]
pub struct Items {
    pub items : Vec<Item>
}

#[derive(Deserialize, Clone)]
pub struct Item {
    pub image_url : String,
    pub id : u32
}