pub mod entity_manager;
mod name_reg;
pub mod matcher;
pub mod type_manager;

use self::matcher::MatcherManager;
use self::type_manager::StreamType;
pub struct GenTools {
    pub entity_manager: EntityManager,
    pub match_manager: MatcherManager,
    pub type_manager: TypeManager,
}

impl GenTools {
    pub fn new() -> GenTools {
        GenTools {
            entity_manager: EntityManager::new(),
            match_manager: MatcherManager::new(),
            type_manager: TypeManager::new(),
        }
    }
}

use std::collections::HashMap;

use super::types::TilComponent;
pub struct EntityManager {
    entity_list: Vec<HashMap<String, TilComponent>>,
    name_reg: NameReg,
}

struct NameReg {
    name_map: HashMap<String, u32>,
}

pub struct TypeManager {
    type_list: Vec<StreamType>,
}