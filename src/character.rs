use std::collections::HashSet;

struct Abilities{
    str: u8,
    dex: u8,
    con: u8,
    int: u8,
    wis: u8,
    cha: u8,

}
struct Modifiers{
    str: u8,
    dex: u8,
    con: u8,
    int: u8,
    wis: u8,
    cha: u8,
}
enum Skills{
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

struct DeathSaves {
    success: u8,
    failure: u8,
}

struct Character {
    name: String,
    race: String,
    class: String,
    xp: u32,
    lvl: u8,

    abilities: Abilities,

    hp_max: u8,
    hp_now: u8,

    death_saves: DeathSaves,

    skills: HashSet<Skills>,
}

impl Character {
    fn Modifiers(&self) -> Modifiers{
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
    fn ProficiencyBonus(&self) -> u8 {
        &self.lvl/4+1
    }
    
}
