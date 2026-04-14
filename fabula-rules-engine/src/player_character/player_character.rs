struct PlayerCharacter { // pg 154 of core rulebook
    player: String,
    name: String,
    identity: String,
    theme: String,
    origin: String,
}

// --- tests ---
#[cfg(test)]
mod tests {

    use super::*;

    #[test] // marks this function as a test case
    fn test_player_character() {
        let test_player_chracter = PlayerCharacter { 
            name: String::from("Test Name"),
            player: String::from("Test Player"),
            identity: String::from("Test Identity"),
            theme: String::from("Test Theme")
            origin: String::from("Test Origin")
        };

        assert_eq!(test_player_chracter.name.to_lowercase(), "test name");
        assert_eq!(test_player_chracter.player.to_lowercase(), "test player");
        assert_eq!(test_player_chracter.identity.to_lowercase(), "test identity");
        assert_eq!(test_player_chracter.theme.to_lowercase(), "test theme");
        assert_eq!(test_player_character.origin.to_lowercase(), "test origin");
    }
}