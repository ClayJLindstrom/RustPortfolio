use bevy::prelude::*;
// use bevy::render::render_resource::Texture;
// use bevy_xpbd_2d::prelude::*;

mod rigidbody2d;
// use crate::rigidbody2d::basics;
// use crate::basics::*;
// use RustPortfolio::basics;//doesn't help

//note for ECS (Entity Compoenet System) Paradigm
// Entities are objects,
struct Entity(u64);
// // Components are, well, the components of an entity(like position and velocity).
// #[derive(Component)]

// Systems are what process the components 
//      (like a movement system that runs on all entities with a position and velocity component).
fn print_position_system(query: Query<&Transform>){
    for position in &query{
        println!("Position: {} by {}", position.translation.x, position.translation.y);
    }
}

//test trial
fn hello_world() {
    println!("Hello, world!");
}

//create some people
#[derive(Component)]
struct Person{
    name: String,
    //None of the below work, or are even necessary.
    // transform: Transform,
    // sprite: SpriteBundle->texture
    // sprite: bevy::render::texture::Image
    // sprite: Handle<Texture>
    // sprite: SpriteBundle
}

#[derive(Component)]
struct Enemy{
    speed: f64,
}

#[derive(Component)]
struct Velocity {x: f32, y: f32}
#[derive(Component)]
enum Direction{
    Up,
    Down,
    Left,
    Right,
}
//a name for everything
#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands, asset_server: Res<AssetServer>){
    // commands.spawn((Person, Name("Thorn Rachen".to_string())));
    // commands.spawn((
    //     Person{
    //         name: "Reu".to_string(), 
    //         // transform: Transform{x: 2.0, y: 2.0}
    //         // transform: Transform::from_xyz(100.0, 2.0, 0.),
    //         sprite: SpriteBundle{
    //                     texture:asset_server.load("sprites/ReuFront.png"),
    //                     transform: Transform::from_xyz(100., 0., 0.),
    //                     ..default()
    //                 }
    //     }, Direction::Right
    // ));
    commands.spawn((
        Person{
            name: "Reu".to_string(), 
        },
        SpriteBundle{
            texture:asset_server.load("sprites/ReuFront.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        }, 
        Direction::Right
    ));
    commands.spawn((
        Enemy{
            speed: 50.0, 
        },
        SpriteBundle{
            texture:asset_server.load("sprites/ReuFront.png"),
            transform: Transform::from_xyz(50., 0., 0.),
            ..default()
        }
    ));
    // commands.spawn((Person, Name("Aaron Cutlass".to_string())));
    // commands.spawn((Person, Name("Kayl Abaddon".to_string())));
}
//GREET our people
fn greet_people(query: Query<&Name, With<Person>>){
    for name in &query{
        println!("Hello, {}", name.0);
    }
}

fn control_player(
    time: Res<Time>, 
    mut sprite_position: Query<(&mut Direction, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>
){
    for(mut player, mut transform) in &mut sprite_position{
        match *player{
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Right => transform.translation.x += 150. * time.delta_seconds(),
            Direction::Left => transform.translation.x -= 150. * time.delta_seconds(),
        }

        if keyboard.pressed(KeyCode::Right){
            *player = Direction::Right;
        }else if keyboard.pressed(KeyCode::Down){
            *player = Direction::Down;
        }else if keyboard.pressed(KeyCode::Left){
            *player = Direction::Left;//good.
        }else if keyboard.pressed(KeyCode::Up){
            *player = Direction::Up;
        }
    }
}

fn enemy_AI(
    time: Res<Time>, 
    mut enemies: Query<(&mut Enemy, &mut Transform)>,
    mut target: Query<(&mut Direction, &mut Transform)>
){
    for(mut pawn, mut transform) in &mut enemies{
        for(mut direction, mut targetpos) in &mut target{
            if targetpos.translation.y > transform.translation.y{
                transform.translation.y += 50. * time.delta_seconds();
            }else if targetpos.translation.y < transform.translation.y{
                transform.translation.y -= 50. * time.delta_seconds();
            }else if targetpos.translation.x > transform.translation.x{
                transform.translation.x += 50. * time.delta_seconds();
            }else if targetpos.translation.x < transform.translation.x{
                transform.translation.x -= 50. * time.delta_seconds();
            }
        }
    }
}

fn start(mut commands: Commands, 
        asset_server: Res<AssetServer>
){
    commands.spawn(Camera2dBundle::default());
    //background image
    commands.spawn(SpriteBundle{
        texture: asset_server.load("sprites/background1.png"),
        ..default()
    });
}

fn main() {
    App::new()
        //adding plugins
        .add_plugins((
            DefaultPlugins//2D/3D renderer, asset-loading, UI system, window, input, etc.
            //PhysicsPlugins::default()
        ))
        //void start()
        .add_systems(Startup, (add_people, start))
        //add our system to the update schedule.
        //sounds line Update()
        .add_systems(Update, (greet_people, control_player, enemy_AI))
        .run();
}
