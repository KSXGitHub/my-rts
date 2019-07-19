//! # Main Application
//!
//! This crate compiles to game executable

use khai_first_rts_engine::amethyst;
use khai_first_rts_ecs::states::main::MainState;
use amethyst::prelude::*;
use amethyst::window::*;

fn main() -> amethyst::Result<()> {
    let app_root = amethyst::utils::application_root_dir()?;
    let display_config_path = app_root.join("resources").join("config").join("display.ron");
    let assets_dir = app_root.join("resources").join("assets");

    let game_data = amethyst::GameDataBuilder::default()
        .with_bundle(WindowBundle::from_config_path(display_config_path))?;

    let mut game = Application::new(assets_dir, MainState, game_data)?;
    game.run();

    Ok(())
}
