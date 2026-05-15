// DOMAIN LAYER: stdlib imports only.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AbilityScores {
    pub str: i32,
    pub dex: i32,
    pub con: i32,
    pub int: i32,
    pub wis: i32,
    pub cha: i32,
}

impl AbilityScores {
    pub fn dex_modifier(&self) -> i32 {
        (self.dex - 10) / 2
    }
}