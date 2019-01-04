use amethyst::{
    prelude::*,
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage, Entity},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
    renderer::{
        Camera, PngFormat, Projection, SpriteRender, SpriteSheet,
        SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata, Flipped
    }
};

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;
pub const BALL_VELOCITY_X: f32 = 75.0;
pub const BALL_VELOCITY_Y: f32 = 50.0;
pub const BALL_RADIUS: f32 = 2.0;

#[derive(Default)]
pub struct ScoreBoard {
    pub score_left: i32,
    pub score_right: i32
}

pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left, Right
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32
}

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side, width: 1.0, height: 1.0
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Ball>();

        initialise_ball(world, sprite_sheet_handle.clone());
        initialise_paddles(world, sprite_sheet_handle);
        initialise_camera(world);
        initialise_scoreboard(world);
    }
}

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}

// Initialises the scoreboard.
fn initialise_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource()
    );

    let p1_transform = UiTransform::new(
        "P1".to_string(), Anchor::TopMiddle,
        -50., 50., 1., 200., 50.
    );
    let p2_transform = UiTransform::new(
        "P2".to_string(), Anchor::TopMiddle,
        50., -50., 1., 200., 50.
    );

    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.
        ))
        .build();

    let p2_score = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.
        ))
        .build();

    world.add_resource(ScoreText { p1_score, p2_score });
}

// Initialises one ball in the middle-ish of the arena.
fn initialise_ball(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    // Create the translation.
    let mut local_transform = Transform::default();
    local_transform.set_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    // Assign the sprite for the ball
    let sprite_renderer = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1
    };

    world
        .create_entity()
        .with(sprite_renderer)
        .with(Ball {
            radius: BALL_RADIUS,
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y]
        })
        .with(local_transform)
        .build();
}

// Init paddles; one on the left, one on the right.
fn initialise_paddles(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    let sprite_renderer = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0
    };

    // Correctly position the paddles.
    let y = ARENA_HEIGHT / 2.0;
    left_transform.set_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    world
        .create_entity()
        .with(sprite_renderer.clone())
        .with(Paddle::new(Side::Left))
        .with(left_transform)
        .build();

    world
        .create_entity()
        .with(sprite_renderer.clone())
        .with(Flipped::Horizontal)
        .with(Paddle::new(Side::Right))
        .with(right_transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data.
    // 'texture_handle' is a cloneable reference to the texture.
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/pong_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store
    )
}
