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

    // TODO: Only update Camera component when window dimensions change?
    fn run(&mut self, (entities, dimensions, mut cameras, mut transforms): Self::SystemData) {
        let entity = self.camera.get_or_insert_with(|| {
            let mut transform = Transform::default();
            transform.set_translation_xyz(0.0, 0.0, 1.0);
            entities.build_entity()
                .with(transform, &mut transforms)
                .build()
        });

        let zoom = (dimensions.width().min(dimensions.height()) / 200.0).floor().max(1.0);
        let width = dimensions.width() / zoom;
        let height = dimensions.height() / zoom;
        cameras.insert(*entity, Camera::standard_2d(width, height)).unwrap();
    }
}
