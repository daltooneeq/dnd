use std::collections::HashSet;
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
    pub class: String,
    pub xp: u32,
    pub lvl: u8,

    pub abilities: Abilities,

    pub hp_max: u8,
    pub hp_now: u8,

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
    pub fn proficiency_bonus(&self) -> u8 {
        &self.lvl/4+1
    }    
}

