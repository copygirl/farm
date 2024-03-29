use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    prelude::*,
    renderer::{
        sprite::SpriteGrid,
        ImageFormat, SpriteRender, SpriteSheet, Texture
    },
};

use crate::components::Position;
use crate::controls::Player;

pub struct MainState;

impl SimpleState for MainState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let texture = load_texture(world, "textures/character.png");
        let sprite_sheet = load_sprite_sheet_from_grid(world, texture, 64, 64, 4, 4);

        world
            .create_entity()
            .with(Player::default())
            .with(Position::default())
            .with(Transform::default())
            .with(SpriteRender {
                sprite_sheet: sprite_sheet,
                sprite_number: 0,
            })
            .build();
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        use amethyst::winit::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
        match event {
            StateEvent::Window(event) => match event {
                Event::WindowEvent { event, .. } => match event {
                    // Quit if the escape button was pressed ..
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape), ..
                        }, ..
                    }
                    // .. or the window was requested to be closed.
                    | WindowEvent::CloseRequested => Trans::Quit,

                    _ => Trans::None
                },
                _ => Trans::None
            },
            _ => Trans::None
        }
    }
}

fn load_texture(world: &mut World, path: &'static str) -> Handle<Texture> {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(path, ImageFormat::default(), (), &texture_storage)
}

fn load_sprite_sheet_from_grid(
    world: &mut World,
    texture: Handle<Texture>,
    texture_width: u32, 
    texture_height: u32,
    columns: u32,
    rows: u32,
) -> Handle<SpriteSheet> {
    let sheet = SpriteSheet {
        texture: texture,
        sprites: SpriteGrid {
            texture_width,
            texture_height,
            columns,
            rows: Some(rows),
            ..Default::default()
        }
        .build_sprites(),
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load_from_data(sheet, (), &sprite_sheet_storage)
}
