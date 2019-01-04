extern crate amethyst;
mod pong;
mod systems;

use crate::pong::Pong;

use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline,
                             RenderBundle, Stage},
    ui::{DrawUi, UiBundle}
};

fn main() -> amethyst::Result<()> {
    // Enable logging.
    amethyst::start_logger(amethyst::LoggerConfig {
        stdout: amethyst::StdoutLog::Colored,
        level_filter: amethyst::LogLevelFilter::Warn,
        log_file: None,
        allow_env_override: false
    });

    use amethyst::utils::application_root_dir;
    let app_root = application_root_dir()?;

    // Set up window from config.
    let path = app_root.join("resources/display_config.ron");
    let config = DisplayConfig::load(&path);

    // Rendering code.  Simply renders a black background (clear target).
    let pipe = Pipeline::build()
        .with_stage(Stage::with_backbuffer()
        .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
        .with_pass(DrawFlat2D::new())
        .with_pass(DrawUi::new())
    );

    use amethyst::input::InputBundle;
    let binding_path = app_root.join("resources/bindings_config.ron");

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<String, String>::new())?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallsSystem, "ball_system", &[])
        .with(systems::BounceSystem, "collision_system", &["paddle_system", "ball_system"])
        .with(systems::WinnerSystem, "winner_system", &["ball_system"]);

    let mut game = Application::new("./", Pong, game_data)?;

    game.run();

    Ok(())
}
