use khai_first_rts_engine::*;
use crate::graphics::initialize_camera;

pub struct GameplayState;

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        initialize_camera(world);
    }
}
