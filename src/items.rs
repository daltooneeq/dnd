use crate::character::Character;

struct Item {
    name: String,
    cost: u16,
    weight: u8,
    owner: Character,

}

struct Armor {
    item: Item,
    defaultAC: u8,
    plusDEX: bool,

}
impl Armor {
    fn GetArmorClass(&self) -> u8{
        self.item.owner.abilities.dex * (self.plusDEX as u8) + self.defaultAC
    }
}