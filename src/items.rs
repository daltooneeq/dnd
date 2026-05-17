use std::fs::read_to_string;

use serde::{Serialize, Deserialize};
use serde_json::{to_string, from_str};

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
    pub defaultAC: u8,
    pub plusDEX: bool,
    pub strangeNeed: Option<u8>,
}
impl Armor {
    fn GetArmorClass(&self, owner: &Character) -> u8{
        owner.Modifiers().dex * (self.plusDEX as u8) + self.defaultAC
    }
}

const ARMOR_JSON: &str = include_str!("../data/armor.json");

pub fn GetArmor() -> Vec<Armor>{
    let json = read_to_string(ARMOR_JSON).unwrap();
    serde_json::from_str(&json).unwrap()
}
