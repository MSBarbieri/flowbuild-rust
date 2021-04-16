use engine::Engine;
use engine::PersistMode;
fn main() {
    let mut engine = Engine::new(PersistMode::Database, None);

    engine.create_process();
}
