pub mod name_reg;
pub mod matcher;

use self::matcher::MatcherManager;
pub struct GenTools {
    pub name_map: NameReg,
    pub match_manager: MatcherManager,
    pub edge_list: Vec<(String, String)>,
}

impl GenTools {
    pub fn new() -> GenTools {
        GenTools {
            name_map: NameReg::new(),
            match_manager: MatcherManager::new(),
            edge_list: Vec::new(),
        }
    }
}

use std::collections::HashMap;
pub struct NameReg {
    name_map: HashMap<String, u32>,
}