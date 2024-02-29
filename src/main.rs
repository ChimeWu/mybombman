use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;

const ELEMENT_SIZE: f32 = 20.0;
const SPEED : f32 = 50.0;

#[derive(Component)]
struct Player(String);

#[derive(Component)]
struct Wall;


fn main(){
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Cat".into(),
            ..default()
        },),
        ..default()
    }))
    .add_systems(Startup,setup)
    .add_systems(Update,move_player)
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Query<&Window>){
    commands.spawn(Camera2dBundle::default());
    let window = windows.single();

    for i in 0..(window.width() as i32 / ELEMENT_SIZE as i32) {
        for j in 0..(window.height() as i32 / ELEMENT_SIZE as i32) {
            commands.spawn(SpriteBundle{
                texture: asset_server.load("stone_floor.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(ELEMENT_SIZE,ELEMENT_SIZE)),
                    ..Default::default()
                },
                transform: Transform::from_translation(
                    Vec3::new(get_x(window, i),get_y(window, j), 0.0)),
                ..Default::default()
            });
        
        }
    }

    for i in 0..(window.width() as i32 / ELEMENT_SIZE as i32) {
        commands.spawn((SpriteBundle{
            texture: asset_server.load("green_wall.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(ELEMENT_SIZE,ELEMENT_SIZE)),
                ..Default::default()
            },
            transform: Transform::from_translation(
                Vec3::new(get_x(window, i),get_y(window, 0), 10.0)),
            ..Default::default()
        },Wall));
    }

    commands.spawn((SpriteBundle{
        texture: asset_server.load("mango.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(ELEMENT_SIZE,ELEMENT_SIZE)),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0.0,0.0,1.0)),
        ..Default::default()
    },Player("Cat".to_string())));
}

fn get_x(window: &Window, i: i32) -> f32 {
    -window.width() as f32/2.0 + ELEMENT_SIZE/2.0 + (i as f32 * ELEMENT_SIZE)
}

fn get_y(window: &Window, j: i32) -> f32 {
    window.height() as f32 /2.0 - ELEMENT_SIZE/2.0 - (j as f32 * ELEMENT_SIZE)
}

fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Transform, (With<Player>,Without<Wall>)>,
    wall : Query<&Transform, (Without<Player>,With<Wall>)>
){
    for mut player_transform in &mut player {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        if input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }

        let new_position = player_transform.translation + direction.normalize() * SPEED * time.delta_seconds();
        let mut not_collision = true;
        for wall_transform in wall.iter() {
            if (new_position.x < wall_transform.translation.x + ELEMENT_SIZE)
            & (new_position.x > wall_transform.translation.x - ELEMENT_SIZE)
            & (new_position.y < wall_transform.translation.y + ELEMENT_SIZE)
            & (new_position.y > wall_transform.translation.y - ELEMENT_SIZE){
                not_collision = false;
            }
        }

        if (direction != Vec3::ZERO) & not_collision {
            player_transform.translation = new_position;
        }
    }
}

/*
        let obstacle:Vec<_> =  wall.iter().
        filter(|&t|{(t.translation.x <= player_transform.translation.x + ELEMENT_SIZE/2.0 as f32)
        & (t.translation.x >= player_transform.translation.x - ELEMENT_SIZE/2.0 as f32)})
        .filter(|&t|{(t.translation.y - player_transform.translation.y).abs() <= ELEMENT_SIZE as f32}).collect();
    
        let x_ok = obstacle.is_empty();

        let obstacle:Vec<_> =  wall.iter().
        filter(|&t|{(t.translation.y <= player_transform.translation.y + ELEMENT_SIZE/2.0 as f32)
        & (t.translation.y >= player_transform.translation.y - ELEMENT_SIZE/2.0 as f32)})
        .filter(|&t|{(t.translation.x - player_transform.translation.x).abs() <= ELEMENT_SIZE as f32}).collect();
    
        let y_ok = obstacle.is_empty();

                let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::ArrowLeft) {
            if x_ok {direction.x -= 1.0;}
        }
        if input.pressed(KeyCode::ArrowRight) {
            if x_ok {direction.x += 1.0;}
        }
        if input.pressed(KeyCode::ArrowUp) {
            if y_ok {direction.y += 1.0;}
        }
        if input.pressed(KeyCode::ArrowDown) {
            if y_ok {direction.y -= 1.0;}
        }

fn x_ok(
    wall : Query<&Transform, (Without<Player>,With<Wall>)>,
    player_transform: &Transform,
) -> bool{
    let obstacle:Vec<_> =  wall.iter().
    filter(|&t|{(t.translation.x <= player_transform.translation.x + ELEMENT_SIZE as f32)
    & (t.translation.x >= player_transform.translation.x - ELEMENT_SIZE as f32)})
    .filter(|&t|{(t.translation.y - player_transform.translation.y).abs() <= ELEMENT_SIZE as f32}).collect();

    obstacle.is_empty()
}

fn y_ok(
    wall : Query<&Transform, (Without<Player>,With<Wall>)>,
    player_transform: &Transform,
) -> bool{
    let obstacle:Vec<_> =  wall.iter().
    filter(|&t|{(t.translation.y <= player_transform.translation.y + ELEMENT_SIZE as f32)
    & (t.translation.y >= player_transform.translation.y - ELEMENT_SIZE as f32)})
    .filter(|&t|{(t.translation.x - player_transform.translation.x).abs() <= ELEMENT_SIZE as f32}).collect();

    obstacle.is_empty()
}

enum Axis {
    X,
    Y,
}

fn axis_ok(
    axis: Axis,
    wall: Query<&Transform, (Without<Player>, With<Wall>)>,
    player_transform: &Transform,
) -> bool {
    let obstacle: Vec<_> = wall
        .iter()
        .filter(|&t| match axis {
            Axis::X => {
                (t.translation.x <= player_transform.translation.x + ELEMENT_SIZE as f32)
                    & (t.translation.x >= player_transform.translation.x - ELEMENT_SIZE as f32)
                    & ((t.translation.y - player_transform.translation.y).abs() <= ELEMENT_SIZE as f32)
            }
            Axis::Y => {
                (t.translation.y <= player_transform.translation.y + ELEMENT_SIZE as f32)
                    & (t.translation.y >= player_transform.translation.y - ELEMENT_SIZE as f32)
                    & ((t.translation.x - player_transform.translation.x).abs() <= ELEMENT_SIZE as f32)
            }
        })
        .collect();

    obstacle.is_empty()
}

        let new_position = player_transform.translation + direction.normalize() * SPEED * time.delta_seconds();
        let mut not_collision = true;
        for wall_transform in wall.iter() {
            if new_position.distance(wall_transform.translation) < ELEMENT_SIZE{
                not_collision = false;
            }
        }

        if (direction != Vec3::ZERO) & not_collision {
            player_transform.translation = new_position;
        }



*/