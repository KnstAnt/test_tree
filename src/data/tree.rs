use std::{cell::RefCell, collections::HashMap};

use super::Path;

pub struct DataTree {
    data: RefCell<HashMap<String, f64>>,
}
///
impl DataTree {
    ///
    pub fn new() -> Self {
        Self {
            data: RefCell::new(HashMap::new()),
        }
    }
    ///
    pub fn get(&self, path: &Path) -> Option<f64> {
        let path = path.to_string();
        if let Some(value) = self.data.borrow().get(&path) {
            Some(*value)
        } else {
            DataTree::read_from_db(path)
        }
    }
    ///
    pub fn set(&self, path: &Path, value: f64) {
        let path = path.to_string();
        self.data.borrow_mut().insert(path, value);
    }
    ///
    pub fn remove(&self, path: &Path) {
        self.data.borrow_mut().remove(&path.to_string());
    }
    ///
    fn read_from_db(_: String) -> Option<f64> {
        Some(42.0)
    }
}
