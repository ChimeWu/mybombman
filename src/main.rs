use bevy::prelude::*;

fn main(){
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Cat".into(),
            name: Some("bevy.app".into()),
            resolution: (800.0, 600.0).into(),
            ..default()
        },),
        ..default()
    }))
    .add_systems(Startup,setup)
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn(Camera2dBundle::default());

    for i in 0..9 {
        for j in 0..7 {
            commands.spawn(SpriteBundle{
                texture: asset_server.load("green_wall.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0,100.0)),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(-400.0 + ((i+1) as f32 * 50.0), 300.0+ ((j+1) as f32 * 50.0), 0.0)),
                ..Default::default()
            });
        
        }
    }
    commands.spawn(SpriteBundle{
        texture: asset_server.load("cat.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0,100.0)),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0.0,0.0,1.0)),
        ..Default::default()
    });
}


