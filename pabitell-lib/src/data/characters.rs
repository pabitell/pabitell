use crate::Character;
use serde::{Deserialize, Serialize};

pub trait CharacterData<'a, C>: From<C> + Into<C> + Serialize + Deserialize<'a> + Clone
where
    C: Character,
{
}
