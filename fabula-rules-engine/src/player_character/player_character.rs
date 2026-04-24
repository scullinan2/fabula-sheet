
use crate::dice::AttributeDice;

struct PlayerCharacter { // pg 154 of core rulebook
    player: String,
    name: String,
    identity: String,
    theme: String,
    origin: String,
}

pub struct Attributes { // pg 36 of the rulebook
    // dexterity, insight, might, willpower
    pub dexterity: AttributeDice,
    pub insight: AttributeDice,
    pub might: AttributeDice,
    pub willpower: AttributeDice
}

// --- tests ---
#[cfg(test)]
mod tests {

    /*use super::*;

    #[test] // marks this function as a test case
    fn test_player_character() {
        let test_player_character = PlayerCharacter { 
            name: String::from("Test Name"),
            player: String::from("Test Player"),
            identity: String::from("Test Identity"),
            theme: String::from("Test Theme"),
            origin: String::from("Test Origin")
        };

        assert_eq!(test_player_character.name.to_lowercase(), "test name");
        assert_eq!(test_player_character.player.to_lowercase(), "test player");
        assert_eq!(test_player_character.identity.to_lowercase(), "test identity");
        assert_eq!(test_player_character.theme.to_lowercase(), "test theme");
        assert_eq!(test_player_character.origin.to_lowercase(), "test origin");
    }

    #[test]
    fn test_attributes() {
        let test_attributes = Attributes {
            dexterity: AttributeDice::D6,
            insight: AttributeDice::D6,
            might: AttributeDice::D6,
            willpower: AttributeDice::D6
        };

        assert_eq!(test_attributes.dexterity, DieSize::Attribute(D6));
        assert_eq!(test_attributes.insight, DieSize::Attribute(D6));
        assert_eq!(test_attributes.might, DieSize::Attribute(D6));
        assert_eq!(test_attributes.willpower, DieSize::Attribute(D6));
    }*/
}