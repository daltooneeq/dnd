use std::collections::{HashSet, HashMap};
use serde::{Serialize, Deserialize};
use std::fs::read_to_string;

//SPELL_INCREASE - приращение ячеек заклинаний с каждым уровнем начиная с 2
const SPELL_INCREASE: [[u8; 10]; 19] = [
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 2, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 2, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
];

pub struct Abilities{
    pub str: u8,
    pub dex: u8,
    pub con: u8,
    pub int: u8,
    pub wis: u8,
    pub cha: u8,

}
pub struct Modifiers{
    pub str: u8,
    pub dex: u8,
    pub con: u8,
    pub int: u8,
    pub wis: u8,
    pub cha: u8,
}
pub enum Skills{
    //DEX
    Acrobatics,
    SleitOfHand,
    Stealth,
    //STR
    Athletics,
    //INT
    Arcana,
    History,
    Investigation,
    Nature,
    Religion,
    //WIS
    AnimalHandling,
    Insight,
    Medicine,
    Perception,
    Survival,
    //CHA
    Deception,
    Intimidation,
    Performance,
    Persuasion,
}

pub struct DeathSaves {
    pub success: u8,
    pub failure: u8,
}

pub struct Character {
    pub name: String,
    pub race: String,
    pub class: Class,
    pub xp: u32,
    pub lvl: u8,

    pub abilities: Abilities,

    pub hd_now: u8,

    pub death_saves: DeathSaves,

    pub skills: HashSet<Skills>,
}

impl Character {
    pub fn modifiers(&self) -> Modifiers{
        let abilities = &self.abilities;
        Modifiers {
            str: (abilities.str-10)/2,
            dex: (abilities.dex-10)/2,
            con: (abilities.con-10)/2,
            int: (abilities.int-10)/2,
            wis: (abilities.wis-10)/2,
            cha: (abilities.cha-10)/2,
        }
    }
    pub fn master_bonus(&self) -> u8 {
        2 + (&self.lvl - 1) / 4
    }   
    pub fn max_hits(&self) -> u8 {
        (&self.class.hits_dice) * &self.lvl + &self.modifiers().con
    }
    pub fn max_spells(&self) -> [u8; 10] {
        let mut spells = self.class.spell_1.clone();
        for i in 2..=20 {
            if self.lvl < i as u8 {
                break;
            }
            for j in 0..9 {
                spells[j] += SPELL_INCREASE[i][j];   
            }
        }
        spells
    }
}

#[derive(Serialize, Deserialize)]
pub struct Class {
    pub name: String,

    pub hits_dice: u8,

    pub spell_1: [u8; 10],

    pub weapon_own: Vec<String>,
    pub armor_own: Vec<String>,
}
impl Class {
    pub fn get_default() -> HashMap<String, Class>{
        let json = read_to_string(crate::CLASS_JSON).unwrap();
        let json: Vec<Class> = serde_json::from_str(&json).unwrap();
        
        let mut default: HashMap<String, Class> = HashMap::new();
        for el in json {
            default.insert((el.name).clone(), el);
        }

        default
    } 
}
