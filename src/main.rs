use dnd::{items, character};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::{io, fs};
use std::collections::HashMap;

fn parse_bool(s: &str) -> bool {
    match s {
        "0" => false,
        "1" => true,
        _ => panic!(),
    }
}
fn main() {
   //set_json("data/class.json", || add_class(items::Weapon::get_default(), items::Armor::get_default())); 
}

#[allow(dead_code)]
fn set_json<F, T>(path: &str, f: F)
where 
    T: Serialize + DeserializeOwned,    
    F: Fn() -> Option<T>
{
    loop {
        let file = fs::read_to_string(path).unwrap();
        let mut json: Vec<T> = serde_json::from_str(&file).unwrap();
        let new_json = match f() {
            Some(v) => v,
            None => break,
        };
        json.push(new_json);
        let result = serde_json::to_string_pretty(&json).unwrap();
        fs::write(path, result).unwrap()
    }
}
#[allow(dead_code)]
fn add_armor() -> Option<items::Armor>{
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error");
    println!("{}", input);

    if input == "break\n".to_string(){
        return None
    }
    let input: Vec<&str> = input.trim().split_whitespace().collect();
    
    let strangeneed: Option<u8> = if input.len() != 6{
        None
    } else {
        Some(input[5].parse().unwrap())
    };

    Some(items::Armor {
        item: items::Item {
            name: input[0].to_string(),
            cost: input[1].parse().unwrap(),
            weight: input[2].parse().unwrap(),
        },
        default_AC: input[3].parse().unwrap(),
        plus_dex: input[4].parse().unwrap(),
        strange_need: strangeneed,
    })


}


#[allow(dead_code)]
fn add_weapon() -> Option<items::Weapon> {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error");

    if input == "break\n".to_string(){
        return None;
    }

    let input: Vec<&str> = input.trim().split_whitespace().collect();

    Some(items::Weapon {
        item: items::Item {
            name: input[0].to_string(),
            cost: input[1].parse().unwrap(),
            weight: input[2].parse().unwrap(),
        },
        damage: input[3].parse().unwrap(),

        ammunition: parse_bool(input[4]),
        two_handed: parse_bool(input[5]),
        reaching: parse_bool(input[6]),
        light: parse_bool(input[7]),
        throwing: parse_bool(input[8]),
        special: parse_bool(input[9]),
        recharge: parse_bool(input[10]),
        heavy: parse_bool(input[11]),
        universal: parse_bool(input[12]),
        fencing: parse_bool(input[13]),

        distance_norm: input[14].parse().unwrap(),
        distance_max: input[15].parse().unwrap(),

    })
}

fn add_class(weapon: HashMap<String, items::Weapon>, armor: HashMap<String, items::Armor>) -> Option<character::Class> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");

    if input == "break\n".to_string(){
        return None;
    }

    let input: Vec<&str> = input.trim().split_whitespace().collect();
    
    let mut i: usize = 0;
    let mut class = character::Class {
        name: input[i].to_string(),
        hits_dice: input[i+1].parse().unwrap(),
        spell_1: Some([0; 10]),
        weapon_own: Vec::new(),
        armor_own: Vec::new(),
    };
    i = 2;
    
    if input[2] == "None" {
        class.spell_1 = None;
        i = 3;
    } else {
        let spells = class.spell_1.as_mut().unwrap();
        for spell in spells.iter_mut() {
            *spell = input[i].parse().unwrap();
            i += 1;
        }
    }


    for el in &input[i..] {
        if weapon.contains_key(*el) {
            class.weapon_own.push(el.to_string());
        } else if armor.contains_key(*el) {
            class.armor_own.push(el.to_string());
        } else {println!("Weapon or armor {} not found", *el);}
    }

    Some(class)
    
} 
