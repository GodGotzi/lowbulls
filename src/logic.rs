use anyhow::Result;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub trait LogicKey {
    fn to_index(&self) -> usize;
    fn len() -> usize;
}

pub trait LowBullApplication<K: Eq + LogicKey> {
    fn handle_key(&mut self, key: K) -> Result<()>;
}

impl<K: Eq + LogicKey + std::fmt::Debug> LowBullMaster<K, A> {
    pub fn new() -> Self {
        Self {
            application,
            logic_actions: Arc::new(RwLock::new(vec![Some(0); K::len()])),
            ui_values: HashMap::new(),
        }
    }

    pub fn register_logic(&mut self, key: K, action: Action<A, R>) {
        self.logic_actions.write().unwrap()[key.to_index()] = Some(action);
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
