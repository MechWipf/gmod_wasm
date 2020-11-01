use std::{cell::RefCell, collections::HashMap};

use wasmer::Instance;

static mut GLOBAL_STATE: RefCell<Option<GlobalState>> = RefCell::new(None);

pub(crate) fn init_global_state() {
    let mut cache = unsafe { GLOBAL_STATE.borrow_mut() };

    if cache.is_none() {
        cache.replace(GlobalState::new());
    }
}

pub(crate) fn drop_global_state() {
    let mut cache = unsafe { GLOBAL_STATE.borrow_mut() };

    if cache.is_some() {
        let drop_me = cache.take();
        drop(drop_me);
    }
}

struct GlobalState {
    instances: HashMap<String, Instance>,
}

impl GlobalState {
    pub(crate) fn new() -> Self {
        GlobalState {
            instances: HashMap::new(),
        }
    }

    pub(crate) fn add_instance(&mut self) -> String {
        self.instances.insert(, v)
    }
}
