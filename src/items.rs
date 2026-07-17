use std::fs::read_to_string;

use serde::{Serialize, Deserialize};

use crate::character::Character;

const ARMOR_JSON: &str = include_str!("../data/armor.json");
const WEAPON_JSON: &str = include_str!("../data/weapon.json");




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
    pub plus_dex: bool,
    pub strange_need: Option<u8>,
}
impl Armor {
    pub fn get_armor() -> Vec<Armor>{
        let json = read_to_string(ARMOR_JSON).unwrap();
        serde_json::from_str(&json).unwrap()
    }
    fn get_armor_class(&self, owner: &Character) -> u8{
        owner.modifiers().dex * (self.plus_dex as u8) + self.default_AC
    }
}

#[derive(Serialize, Deserialize)]
pub struct Weapon {
    pub item: Item,
    pub damage: u8,
    
    pub ammunition: bool, //1
    pub two_handed: bool, //2
    pub reaching: bool, //3 
    pub light: bool, //4
    pub throwing: bool, //5
    pub special: bool, //6
    pub recharge: bool, //7
    pub heavy: bool, //8 
    pub universal: bool, //9
    pub fencing: bool, //10
    
    pub distance_norm: u16,
    pub distance_max: u16,
}
impl Weapon {
    pub fn get_weapon() -> Vec<Weapon>{
        let json = read_to_string(WEAPON_JSON).unwrap();
        serde_json::from_str(&json).unwrap()
    }
}
