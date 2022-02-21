use std::sync::RwLock;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use model;

pub type Data = HashMap<model::ItemType, HashMap<i64, bool>>;

lazy_static! {
    static ref IDENTIFICATION: RwLock<Data> = RwLock::new(HashMap::new());
}

pub fn save() -> Data {
    *IDENTIFICATION.try_read().unwrap()
}

pub fn load(data: Data) {
    *IDENTIFICATION.try_write().unwrap() = data
}

pub fn is_identified(item_type: model::ItemType, subval: i64) -> bool {
    if let Entry::Occupied(o) = (*IDENTIFICATION.try_read().unwrap()).entry(item_type) {
        if let Entry::Occupied(p) = o.get().entry(subval) {
            return *p.get();
        }
    }

    false
}

pub fn set_identified(item_type: model::ItemType, subval: i64, is_identified: bool) {
    IDENTIFICATION.try_write().unwrap()
        .entry(item_type)
        .or_insert(HashMap::new())
        .entry(subval)
        .and_modify(|val| *val = is_identified)
        .or_insert(is_identified);
}
