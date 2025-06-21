use bevy::{
    app::App,
    asset::{ Assets, RenderAssetUsages },
    ecs::system::{ Res, ResMut, RunSystemOnce },
    image::Image,
    render::render_resource::{ Extent3d, TextureDimension, TextureFormat, TextureUsages },
};
use bevy_ecs_tilemap::map::TilemapTexture;

fn make_image() -> Image {
    let mut image = Image::new_fill(
        Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        TextureDimension::D2,
        &[255, 255, 255, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default()
    );
    image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING;
    image
}

#[test]
fn verify_ready_and_set_copy_src_work() {
    let mut app = App::new();
    app.init_resource::<Assets<Image>>();

    let handle = {
        let mut images = app.world_mut().resource_mut::<Assets<Image>>();
        images.add(make_image())
    };
    let tex = TilemapTexture::Single(handle.clone_weak());

    // 1. `verify_ready` should fail (COPY_SRC not set yet)
    {
        let tex = tex.clone();
        let _ = app.world_mut().run_system_once(move |images: Res<Assets<Image>>| {
            assert!(!tex.verify_ready(&images));
        });
    }

    // 2. add the COPY_SRC usage flag
    {
        let tex = tex.clone();
        let _ = app.world_mut().run_system_once(move |mut images: ResMut<Assets<Image>>| {
            tex.set_images_to_copy_src(&mut images);
        });
    }

    // 3. `verify_ready` should now succeed
    {
        let _ = app.world_mut().run_system_once(move |images: Res<Assets<Image>>| {
            assert!(tex.verify_ready(&images));
        });
    }
}
