use serde::ser::{Serialize, Serializer, SerializeStruct};

pub struct Player {
    pub igns: Vec<String>,
    pub tag: String,
    pub score: u32,
}

impl Default for Player {
    fn default () -> Player {
        Player {
            igns: Vec::new(),
            tag: String::new(),
            score: 0,
        }
    }
}

impl Serialize for Player {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer, 
    {
        let mut state = serializer.serialize_struct("player", 3)?;
        state.serialize_field("igns", &self.igns)?;
        state.serialize_field("tag", &self.tag[1..&self.tag.len() - 1])?;
        state.serialize_field("score", &self.score)?;
        state.end()
    }
}

pub struct Game {
    pub date: String,
    pub name: String,
    pub id: String,
    pub players: Vec<Player>,
}

impl Default for Game {
    fn default () -> Game {
        Game {
            date: String::new(),
            name: String::new(),
            id: String::new(),
            players: Vec::new(),
        }
    }
}
