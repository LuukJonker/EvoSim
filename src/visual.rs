use crate::map::{Map, ReadOnlyMap, Tile};
use bevy::{app::AppExit, prelude::*, sprite::MaterialMesh2dBundle};

const CAMERA_SPEED: f32 = 50.0;

pub fn start(map: &Map) {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Layout {
            map: ReadOnlyMap::from_map(map),
        })
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, render_layout)
        .add_systems(Update, (move_camera, confide_camera).chain())
        .add_systems(Update, exit_app)
        .run()
}

#[derive(Component)]
struct Camera;

#[derive(Component)]
struct Cell;

#[derive(Resource)]
struct Layout {
    map: ReadOnlyMap,
}

fn spawn_camera(mut commands: Commands, layout: Res<Layout>) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(
                layout.map.width as f32 / 2.0,
                layout.map.height as f32 / 2.0,
                0.0,
            ),
            projection: OrthographicProjection {
                far: -1000.0,
                near: 1000.0,
                scale: 0.01,
                ..default()
            },
            ..default()
        },
        Camera,
    ));
}

fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        direction.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        direction.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        direction.y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        direction.y -= 1.0;
    }

    camera.translation += direction.normalize_or_zero() * CAMERA_SPEED * time.delta_seconds();
}

fn confide_camera(mut camera_query: Query<&mut Transform, With<Camera>>, layout: Res<Layout>) {
    let mut camera = camera_query.single_mut();

    if camera.translation.x < 0.0 {
        camera.translation.x = 0.0;
    } else if camera.translation.x > layout.map.width as f32 {
        camera.translation.x = layout.map.width as f32;
    }

    if camera.translation.y < 0.0 {
        camera.translation.y = 0.0;
    } else if camera.translation.y > layout.map.height as f32 {
        camera.translation.y = layout.map.height as f32;
    }
}

fn render_layout(
    mut commands: Commands,
    layout: Res<Layout>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let wall_material = materials.add(Color::rgb(0.0, 0.0, 0.0).into());
    let water_material = materials.add(Color::rgb(0.0, 0.0, 1.0).into());
    let grass_material = materials.add(Color::rgb(0.0, 1.0, 0.0).into());

    let square_mesh = meshes.add(shape::Quad::new(Vec2::new(1.0, 1.0)).into());

    for (y, row) in layout.map.layout.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let material = match *cell {
                Tile::Wall => wall_material.clone(),
                Tile::Grass => grass_material.clone(),
                Tile::Water => water_material.clone(),
            };

            commands.spawn(MaterialMesh2dBundle {
                material,
                mesh: square_mesh.clone().into(),
                transform: Transform::from_xyz(x as f32, y as f32, -1.0),
                ..default()
            });
        }
    }
}

fn update_layout(mut layout: ResMut<Layout>) {}

fn exit_app(keyboard_input: Res<Input<KeyCode>>, mut app_exit: EventWriter<AppExit>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        app_exit.send(AppExit);
    }
}
