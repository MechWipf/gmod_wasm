use crate::wasm::WasmInstance;
use lazy_static::lazy_static;
use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

lazy_static! {
    pub static ref GLOBAL_STATE: Arc<Mutex<GlobalState>> = {
        let gs = GlobalState::new();
        let gs = Mutex::new(gs);
        Arc::new(gs)
    };
}

pub struct GlobalState {
    instances: RefCell<HashMap<String, Arc<WasmInstance>>>,
}

impl GlobalState {
    pub(crate) fn new() -> Self {
        GlobalState {
            instances: RefCell::new(HashMap::new()),
        }
    }

    pub(crate) fn instance_add(&self, instance: WasmInstance) -> String {
        let mut inst_map = self.instances.borrow_mut();

        let key = Uuid::new_v4();
        let key = key.to_hyphenated().to_string();

        inst_map.insert(key.clone(), Arc::new(instance));

        key
    }

    pub(crate) fn instance_exists(&self, key: &str) -> bool {
        let inst_map = self.instances.borrow();

        inst_map.contains_key(key)
    }

    pub(crate) fn instance_remove(&self, key: &str) -> bool {
        let mut inst_map = self.instances.borrow_mut();

        matches!(inst_map.remove_entry(key), Some(_))
    }

    pub(crate) fn instance(&self, key: &str) -> Option<Arc<WasmInstance>> {
        let inst_map = self.instances.borrow();

        if let Some(x) = inst_map.get(key) {
            Some(Arc::clone(x))
        } else {
            None
        }
    }
}
