use bevy::prelude::*;
use bevy::render::render_resource::Texture;

//note for ECS (Entity Compoenet System) Paradigm
// Entities are objects,
struct Entity(u64);
// Components are, well, the components of an entity(like position and velocity).
#[derive(Component)]
struct Position {x: f32, y: f32}
// Systems are what process the components 
//      (like a movement system that runs on all entities with a position and velocity component).
fn print_position_system(query: Query<&Position>){
    for position in &query{
        println!("Position: {} by {}", position.x, position.y);
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
    transform: Position
    // sprite: SpriteBundle->texture
    // sprite: bevy::render::texture::Image
    // sprite: Handle<Texture>
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
    commands.spawn(Person{
        name: "Reu".to_string(), 
        transform: Position{x: 2.0, y: 2.0}
        // sprite: 
    });
    commands.spawn((
        SpriteBundle{
            texture:asset_server.load("sprites/ReuFront.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        }, Direction::Right));
    // commands.spawn((Person, Name("Aaron Cutlass".to_string())));
    // commands.spawn((Person, Name("Kayl Abaddon".to_string())));
}
//GREET our people
fn greet_people(query: Query<&Name, With<Person>>){
    for name in &query{
        println!("Hello, {}", name.0);
    }
}

fn control_player(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>){
    for(mut player, mut transform) in &mut sprite_position{
        match *player{
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Right => transform.translation.x += 150. * time.delta_seconds(),
            Direction::Left => transform.translation.x -= 150. * time.delta_seconds(),
        }

        if transform.translation.y > 220.{
            *player = Direction::Right;
        }else if transform.translation.x > 220.{
            *player = Direction::Down;
        }else if transform.translation.y < -200.{
            *player = Direction::Left;//good.
        }else if transform.translation.x < -200.{
            *player = Direction::Up;
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
        ))
        //void start()
        .add_systems(Startup, (add_people, start))
        //add our system to the update schedule.
        //sounds line Update()
        .add_systems(Update, (hello_world, greet_people, control_player))
        .run();
}
