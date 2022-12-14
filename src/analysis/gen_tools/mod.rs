pub mod name_reg;
pub mod matcher;

use self::matcher::MatcherManager;
pub struct GenTools {
    pub name_map: NameReg,
    pub match_manager: MatcherManager,
}
impl GenTools {
    pub fn new() -> GenTools {
        GenTools {
            name_map: NameReg::new(),
            match_manager: MatcherManager::new(),
        }
    }
}

use std::collections::HashMap;
pub struct NameReg {
    name_map: HashMap<String, u32>,
}