pub mod name_reg;
pub mod matcher;
pub mod type_reg;

use self::matcher::MatcherManager;
pub struct GenTools {
    pub name_map: NameReg,
    pub match_manager: MatcherManager,
    pub type_reg: TypeReg,
}

impl GenTools {
    pub fn new() -> GenTools {
        GenTools {
            name_map: NameReg::new(),
            match_manager: MatcherManager::new(),
            type_reg: TypeReg::new(),
        }
    }
}

use std::collections::HashMap;
pub struct NameReg {
    name_map: HashMap<String, u32>,
}

pub struct TypeReg {
    type_list: Vec<String>,
}