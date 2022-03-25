use self::provider::PersistorProvider;

pub mod provider;
pub(crate) mod workflow;
pub(crate) mod process;
pub(crate) mod activity_manager;
pub(crate) mod blueprint;


pub(crate) trait Provider {
    fn get_persistor(&self) -> PersistorProvider {
        PersistorProvider::get_provider()
    }
}