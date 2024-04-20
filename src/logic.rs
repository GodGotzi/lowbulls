use anyhow::Result;
use std::{
    collections::HashMap,
    hash::Hash,
    sync::{Arc, RwLock},
};

type Action<A, R> = Box<dyn FnMut(&mut A, R) -> Result<()>>;

pub enum UiValue {
    Bool(bool),
    I32(i32),
    U32(u32),
    F32(f32),
    I64(i64),
    U64(u64),
    F64(f64),
    String(String),
}

pub struct LowBullMaster<K: Eq + Hash, A, R> {
    application: A,
    logic_actions: Arc<RwLock<HashMap<K, Action<A, R>>>>,
    ui_values: HashMap<K, UiValue>,
}

impl<K: Eq + Hash, A, R> LowBullMaster<K, A, R> {
    pub fn new(application: A) -> Self {
        Self {
            application,
            logic_actions: Arc::new(RwLock::new(HashMap::new())),
            ui_values: HashMap::new(),
        }
    }

    pub fn register_logic(&mut self, key: K, action: Action<A, R>) {
        self.logic_actions.write().unwrap().insert(key, action);
    }

    pub fn run_logic(&mut self, key: K, response: R) -> Result<()> {
        let mut logic_actions = self.logic_actions.write().unwrap();
        let action = logic_actions.get_mut(&key).unwrap();
        action(&mut self.application, response)
    }

    pub fn set_ui_value(&mut self, key: K, value: UiValue) {
        self.ui_values.insert(key, value);
    }

    pub fn get_mut_ui_value(&mut self, key: K) -> Option<&mut UiValue> {
        self.ui_values.get_mut(&key)
    }
}
