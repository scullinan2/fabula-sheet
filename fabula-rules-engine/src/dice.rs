pub enum DiceSize {
    Attribute(AttributeDice),
    Other(OtherDice),
}

pub enum AttributeDice {
    D6,
    D8,
    D10,
    D12,
}

pub enum OtherDice {
    D4,
    D20,
}