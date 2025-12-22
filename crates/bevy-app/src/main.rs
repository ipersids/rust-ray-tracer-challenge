use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::asset::RenderAssetUsages;

use ray_tracer::core::Tuple;
use ray_tracer::geometry::Sphere;
use ray_tracer::graphics::{Color, Ray};

use ray_tracer::lighting::{lighting, Light};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let n: usize = 400;

    // scene
    let ray_orig = Tuple::point(0.0, 0.0, -5.0);
    let mut sphere = Sphere::new();
    sphere.material.color = Color::new(1.0, 0.2, 0.5);
    sphere.material.shininess = 90.0;

    let light = Light::point_light(
        Tuple::point(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    );

    // camera/wall
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / n as f64;
    let half = wall_size / 2.0;

    // pixels (RGBA8)
    let mut rgba = vec![0u8; n * n * 4];

    for y in 0..n {
        let world_y = half - pixel_size * y as f64;
        for x in 0..n {
            let world_x = -half + pixel_size * x as f64;

            let pos = Tuple::point(world_x, world_y, wall_z);
            let dir = (pos - ray_orig).normalize();
            let ray = Ray::new(ray_orig, dir);

            let hit = sphere.intersect(ray).into_iter().find(|t| *t >= 0.0);

            let c = if let Some(t) = hit {
                let p = ray.position(t);
                let normal = sphere.normal_at(p);
                let eye = -ray.direction;
                lighting(&sphere.material, &light, &p, &eye, &normal)
            } else {
                Color::new(0.0, 0.0, 0.0)
            };

            let i = (y * n + x) * 4;
            rgba[i] = c.get_clamped_red_u8();
            rgba[i + 1] = c.get_clamped_green_u8();
            rgba[i + 2] = c.get_clamped_blue_u8();
            rgba[i + 3] = 255;
        }
    }

    let image = Image::new(
        Extent3d {
            width: n as u32,
            height: n as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        rgba,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );

    let image_handle = images.add(image);

    commands.spawn(Camera2d);
    commands.spawn(Sprite {
        image: image_handle,
        custom_size: Some(Vec2::new(n as f32, n as f32)),
        ..default()
    });
}
