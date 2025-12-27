use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use scene_loader::load_scene;
use scene_types::{MaterialDef, SceneFile};

use ray_tracer::lighting::{Light, lighting};
use ray_tracer::math::{Ray, Tuple};
use ray_tracer::shape::{Color, Sphere};

fn main() {
    let scene = match load_scene("scene/example.toml") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to load scene: {e}");
            std::process::exit(1);
        }
    };

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scene(scene))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Resource)]
struct Scene(SceneFile);

fn setup(mut commands: Commands, scene: Res<Scene>, mut images: ResMut<Assets<Image>>) {
    let n: usize = 400;

    let scene = &scene.0;

    let ray_orig = Tuple::point(
        scene.camera.position[0] as f64,
        scene.camera.position[1] as f64,
        scene.camera.position[2] as f64,
    );
    let mut sphere = Sphere::new();
    let color_base = Color::new(
        scene.objects[0].color[0] as f64 / 255.0,
        scene.objects[0].color[1] as f64 / 255.0,
        scene.objects[0].color[2] as f64 / 255.0,
    );
    sphere.material.color = color_base;
    match &scene.objects[0].material {
        MaterialDef::Default(_) => {
            sphere.material.ambient = 0.1;
            sphere.material.diffuse = 0.9;
            sphere.material.specular = 0.9;
            sphere.material.shininess = 90.0;
        }
        MaterialDef::Custom(m) => {
            sphere.material.ambient = m.ambient_coeff as f64;
            sphere.material.diffuse = m.diffuse_coeff as f64;
            sphere.material.specular = m.specular_coeff as f64;
            sphere.material.shininess = m.shininess as f64;
        }
    };

    let light = Light::point_light(
        Tuple::point(
            scene.lights[0].position[0] as f64,
            scene.lights[0].position[1] as f64,
            scene.lights[0].position[2] as f64,
        ),
        Color::new(
            scene.lights[0].color[0] as f64 / 255.0,
            scene.lights[0].color[1] as f64 / 255.0,
            scene.lights[0].color[2] as f64 / 255.0,
        ),
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
