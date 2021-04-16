use custom_derive::custom_derive;
use dyn_clone::DynClone;
use enum_derive::*;
use std::any::Any;
use std::collections::hash_map::HashMap;
use std::fmt::Debug;
use std::mem;
use std::sync::{Arc, Mutex, Once};

custom_derive! {
    #[derive(Debug,Clone, EnumFromStr,Eq,PartialEq)]

pub enum PersistMode {
    Database,
    Memory,
}
}

pub trait Provider: Any + Debug + DynClone + Send + Sync {}
dyn_clone::clone_trait_object!(Provider);

#[derive(Clone, Debug)]
pub struct PersistorProvider {
    // Since we will be used in many threads, we need to protect
    // concurrent access
    mode: PersistMode,
    pub providers: Arc<Mutex<HashMap<String, String>>>,
}

impl PersistorProvider {
    pub fn instance() -> PersistorProvider {
        PersistorProvider::new(None)
    }

    pub fn new(mode: Option<PersistMode>) -> PersistorProvider {
        static mut SINGLETON: *const PersistorProvider = 0 as *const PersistorProvider;
        static ONCE: Once = Once::new();
        // Initialize it to a null value
        let persist_mode = match mode {
            Some(_mode) => _mode,
            None => PersistMode::Memory,
        };

        unsafe {
            ONCE.call_once(|| {
                // Make it
                let mut providers = HashMap::new();
                providers.insert("123".to_string(), "456".to_string());
                let singleton = PersistorProvider {
                    mode: persist_mode,
                    providers: Arc::new(Mutex::new(providers)),
                };
                // Put it in the heap so it can outlive this call
                SINGLETON = mem::transmute(Box::new(singleton));
            });
            // Now we give out a copy of the data that is safe to use concurrently.
            (*SINGLETON).clone()
        }
    }
}

pub trait DatabasePorvider: Provider {}

pub trait MemoryProvider: Provider {}
