use amethyst::{
    core::transform::Transform,
    ecs::prelude::*,
    renderer::Camera,
    window::ScreenDimensions,
};

#[derive(Default)]
pub struct CameraSystem {
    camera: Option<Entity>,
}

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, ScreenDimensions>,
        WriteStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, dimensions, mut cameras, mut transforms): Self::SystemData) {
        let entity = self.camera.get_or_insert_with(|| {
            let mut transform = Transform::default();
            transform.set_translation_xyz(0.0, 0.0, 1.0);
            entities.build_entity()
                .with(transform, &mut transforms)
                .build()
        });
        cameras.insert(*entity, Camera::standard_2d(dimensions.width(), dimensions.height())).unwrap();
    }
}
