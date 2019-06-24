//! Pong Tutorial 1

use amethyst::{
    assets::Processor,
    ecs::{ReadExpect, Resources, SystemData},
    prelude::*,
    renderer::{
        pass::DrawFlat2DDesc, types::DefaultBackend, Factory, Format, GraphBuilder, GraphCreator,
        Kind, RenderGroupDesc, RenderingSystem, SpriteSheet, SubpassBuilder,
    },
    utils::application_root_dir,
    window::{ScreenDimensions, Window, WindowBundle},
};

pub struct Tetris;

impl SimpleState for Tetris{}

fn main() -> amethyst::Result<()>{
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("resources").join("display_config.ron");

    let game_data = GameDataBuilder::default()
    // The WindowBundle provides all the scaffolding for opening a window
    .with_bundle(WindowBundle::from_config_path(display_config_path))?;

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Tetris, game_data)?;
    game.run();

    Ok(())
}
