use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::fmt::Debug;

pub trait DerefNode {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

impl<T> DerefNode for T
where
    T: 'static + Node + Any,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}
pub trait Node: DerefNode + Any + DynClone + erased_serde::Serialize + Debug + Send {
    fn pre_processing(&mut self);
    fn run(&mut self);
}

// This Macros transform Node into a clonable and serializable trait DON'T Remove
erased_serde::serialize_trait_object!(Node);
dyn_clone::clone_trait_object!(Node);

#[derive(Debug, Serialize, Deserialize, Clone, Default, Eq, PartialEq)]
pub struct HelloNode {
    pub hello: String,
}

impl HelloNode {
    pub fn new() -> Self {
        let a = HelloNode {
            hello: "undefined".to_string(),
        };
        return a;
    }
}

impl Node for HelloNode {
    fn pre_processing(&mut self) {
        self.hello = "Hello World!".to_string();
        println!("pre processing");
    }

    fn run(&mut self) {
        println!("hello {}", self.hello);
    }
}
