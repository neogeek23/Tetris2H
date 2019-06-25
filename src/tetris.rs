use amethyst::{
    //assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, 
    //ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture
    },
};

// Playable area
pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const BLOCK_HEIGHT: f32 = 4.0;
pub const BLOCK_WIDTH: f32 = 4.0;

pub const L_SHAPE: [[bool; 4]; 2] = [[true, true, true, false], [true, false, false, false]];
pub const L_SHAPE_REVERSED: [[bool; 4]; 2] = [[true, false, false, false], [true, true, true, false]];
pub const Z_SHAPE: [[bool; 4]; 2] = [[true, true, false, false], [false, true, true, false]];
pub const Z_SHAPE_REVERSED: [[bool; 4]; 2] = [[false, true, true, false], [true, true, false, false]];
pub const LINE_SHAPE: [[bool; 4]; 2] = [[true, true, true, true], [false, false, false, false]];
pub const BOCK_SHAPE: [[bool; 4]; 2] = [[true, true, false, false], [true, true, false, false]];
pub const T_SHAPE: [[bool; 4]; 2] = [[false, true, false, false], [true, true, true, false]];

pub struct Block{
    height: f32,
    width: f32,
    red: f32,
    green: f32,
    blue: f32,
    opacity: f32,
}

pub struct Shape{
    blocks: Box<Vec<Box<Vec<Block>>>>,
}

impl Shape{
    fn new(pattern: [[bool; 4]; 2], color_set: [f32; 4]) -> Shape{
        let mut blocks = Box::new(Vec::new());
        for layer in &pattern{
            let mut row = Box::new(Vec::new());
            for point in layer{
                if *point{
                    let temp_block = Block{
                        height: BLOCK_HEIGHT,
                        width: BLOCK_WIDTH,
                        red: color_set[0],
                        green: color_set[1],
                        blue: color_set[2],
                        opacity: color_set[3],
                    };
                    row.push(temp_block);
                } else {
                    let temp_block = Block{
                        height: 0.0,
                        width: 0.0,
                        red: 0.0,
                        green: 0.0,
                        blue: 0.0,
                        opacity: 0.0,
                    };
                    row.push(temp_block);
                }
            }
            blocks.push(row);
        }
        Shape{
            blocks: blocks,
        }
    }
}

impl Component for Shape{
    type Storage = DenseVecStorage<Self>;
}

pub struct Tetris;

impl SimpleState for Tetris {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialise_camera(world);
    }
}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left. 
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}
