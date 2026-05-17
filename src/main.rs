use dnd::items;
use std::io;
use std::fs;

use serde_json::{to_string, from_str};
fn main() {
    SetArmor();
}

fn SetArmor() {
    let json = fs::read_to_string("data/armor.json").unwrap();
    let mut armors: Vec<items::Armor> = serde_json::from_str(&json).unwrap();
    loop{
        let new_armor = match AddArmor() {
            Some(v) => v,
            None => break,
        };
        armors.push(new_armor);
    }
    let new_json = serde_json::to_string_pretty(&armors).unwrap();
    fs::write("data/armor.json", new_json).unwrap();
}
fn AddArmor() -> Option<items::Armor>{
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
        defaultAC: input[3].parse().unwrap(),
        plusDEX: input[4].parse().unwrap(),
        strangeNeed: strangeneed,
    })


}
