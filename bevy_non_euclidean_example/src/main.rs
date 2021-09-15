#[allow(unused_imports, clippy::single_component_path_imports)]
use bevy_dylib;

use bevy_app::prelude::*;
use bevy_asset::prelude::*;
use bevy_core::prelude::*;
use bevy_ecs::prelude::*;
use bevy_input::prelude::*;
use bevy_math::prelude::*;
use bevy_pbr_spherical::prelude::*;
use bevy_render_spherical::prelude::*;
use bevy_transform_spherical::prelude::*;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugin(bevy_log::LogPlugin::default())
        .add_plugin(bevy_core::CorePlugin::default())
        .add_plugin(bevy_transform_spherical::TransformPlugin::default())
        .add_plugin(bevy_input::InputPlugin::default())
        .add_plugin(bevy_window::WindowPlugin::default())
        .add_plugin(bevy_asset::AssetPlugin::default())
        .add_plugin(bevy_render_spherical::RenderPlugin::default())
        .add_plugin(bevy_pbr_spherical::PbrPlugin::default())
        .add_plugin(bevy_gilrs::GilrsPlugin::default())
        .add_plugin(bevy_winit::WinitPlugin::default())
        .add_plugin(bevy_wgpu::WgpuPlugin::default())
        .add_startup_system(setup.system())
        .add_system(motion.system())
        .add_system(rotation.system())
        .add_system(bevy_input::system::exit_on_esc_system.system())
        .run();
}

struct Camera;

fn motion(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let mut direction = Vec3::ZERO;

    for key in keys.get_pressed() {
        direction += match key {
            KeyCode::A => -Vec3::X,
            KeyCode::D => Vec3::X,
            KeyCode::S => Vec3::Z,
            KeyCode::W => -Vec3::Z,
            KeyCode::Space => Vec3::Y,
            KeyCode::LShift => -Vec3::Y,
            _ => Vec3::ZERO,
        };
    }

    for mut transform in query.iter_mut() {
        *transform =
            *transform * Transform::from_small_translation(direction * 0.3 * time.delta_seconds());
    }
}

fn rotation(
    mut mouse: EventReader<bevy_input::mouse::MouseMotion>,
    mut query: Query<&mut Transform, With<Camera>>,
    windows: Res<bevy_window::Windows>,
) {
    let mut delta: Vec2 = mouse.iter().map(|motion| &motion.delta).sum();
    delta /= windows.get_primary().unwrap().height() / (std::f32::consts::PI / 4.0);

    for mut transform in query.iter_mut() {
        *transform = *transform
            * Transform::from_rotation(Quat::from_xyzw(-delta.y, -delta.x, 0., 1.).normalize());
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 0.1 }));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..Default::default()
    });

    // cubes
    for transform in [
        Transform::identity(),
        Transform::from_translation(Vec3::X * std::f32::consts::FRAC_PI_2),
    ] {
        for rotation in [
            Quat::IDENTITY,
            Quat::from_xyzw(0.5, 0.5, 0.5, 0.5),
            Quat::from_xyzw(0.5, 0.5, 0.5, -0.5),
        ] {
            let transform = Transform::from_rotation(rotation) * transform;
            for t in 0..5 {
                commands.spawn_bundle(PbrBundle {
                    mesh: cube_handle.clone(),
                    material: cube_material_handle.clone(),
                    transform: transform
                        * Transform::from_translation(
                            Vec3::Z * std::f32::consts::FRAC_PI_8 * t as f32,
                        ),
                    ..Default::default()
                });
            }
        }
    }

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(0.25, 0.25, 0.75)),
        light: Light {
            intensity: 0.5,
            ..Default::default()
        },
        ..Default::default()
    });
    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(-0.2, 0.2, -0.2))
                .looking_at(Vec4::W + 0.2 * Vec4::Y, Vec4::Y),

            perspective_projection: bevy_render_spherical::camera::PerspectiveProjection {
                tan_near: 0.01,
                tan_far: -0.01,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Camera);
}
