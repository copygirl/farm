use amethyst::{
    assets::Processor,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        sprite_visibility::SpriteVisibilitySortingSystem,
        types::DefaultBackend,
        RenderingSystem, SpriteSheet
    },
    utils::application_root_dir,
    window::WindowBundle,
};

mod camera;
mod components;
mod controls;
mod farm;
mod movement;
mod render;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let bindings_config_path = config_dir.join("bindings.ron");

    let render_graph = render::RenderGraph::default();
    let render_system = RenderingSystem::<DefaultBackend, _>::new(render_graph);

    let game_data = GameDataBuilder::default()
        .with_bundle(WindowBundle::from_config_path(display_config_path))?
        .with_bundle(InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_config_path)?)?
        .with_bundle(TransformBundle::new())?

        .with(SpriteVisibilitySortingSystem::new(), "sprite_visibility_system", &["transform_system"])
        .with(Processor::<SpriteSheet>::new(), "sprite_sheet_processor", &[])

        .with(camera::CameraSystem::default(), "camera_system", &[])
        .with(controls::ControlsSystem::default(), "controls_system", &[])
        .with(movement::MovementSystem::default(), "movement_system", &[])

        .with_thread_local(render_system);

    let mut game = Application::new(assets_dir, farm::MainState, game_data)?;
    game.run();

    Ok(())
}
