use std::fs::read_to_string;

use serde::{Serialize, Deserialize};

use crate::character::Character;

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub cost: u32,
    pub weight: u8,

}

#[derive(Serialize, Deserialize)]
pub struct Armor {
    pub item: Item,
    pub default_AC: u8,
    pub plus_DEX: bool,
    pub strange_need: Option<u8>,
}
impl Armor {
    fn get_armor_class(&self, owner: &Character) -> u8{
        owner.modifiers().dex * (self.plus_DEX as u8) + self.default_AC
    }
}

const ARMOR_JSON: &str = include_str!("../data/armor.json");

pub fn get_armor() -> Vec<Armor>{
    let json = read_to_string(ARMOR_JSON).unwrap();
    serde_json::from_str(&json).unwrap()
}
