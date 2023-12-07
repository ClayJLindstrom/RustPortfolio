use bevy::prelude::*;
// use bevy::render::render_resource::Texture;
// use bevy_xpbd_2d::prelude::*;
use bevy::app::AppExit;

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
    speed: f32,
    acceleration: f32,
    velocity: (f32, f32)
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

fn add_people(
    time: Res<Time>, 
    mut commands: Commands, 
    asset_server: Res<AssetServer>
){
    let range: f32 = 700.0;
    let localSpeed: f32 = 50.0 + time.elapsed_seconds();
    let localAccel: f32 = localSpeed * 2.0 + time.elapsed_seconds()/20.0;
    // let phaseShift: f32 = 
    // let currentTime: f32 = time.elapsed();
    if time.elapsed_seconds() % 4. <= time.delta_seconds(){
        commands.spawn((
            Enemy{
                speed: localSpeed,
                acceleration: localAccel,
                velocity: (0.,0.) 
            },
            SpriteBundle{
                texture:asset_server.load("sprites/EnemySprite.png"),
                transform: Transform::from_xyz(range, range, 0.),
                ..default()
            }
        ));
        // if(time.elapsed_seconds())
        commands.spawn((
            Enemy{
                speed: localSpeed,
                acceleration: localAccel,
                velocity: (0.,0.) 
            },
            SpriteBundle{
                texture:asset_server.load("sprites/EnemySprite.png"),
                transform: Transform::from_xyz(-range, range, 0.),
                ..default()
            }
        ));
        commands.spawn((
            Enemy{
                speed: localSpeed,
                acceleration: localAccel,
                velocity: (0.,0.) 
            },
            SpriteBundle{
                texture:asset_server.load("sprites/EnemySprite.png"),
                transform: Transform::from_xyz(range, -range, 0.),
                ..default()
            }
        ));
        commands.spawn((
            Enemy{
                speed: localSpeed,
                acceleration: localAccel,
                velocity: (0.,0.) 
            },
            SpriteBundle{
                texture:asset_server.load("sprites/EnemySprite.png"),
                transform: Transform::from_xyz(-range, -range, 0.),
                ..default()
            }
        ));
    }
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
        //don't let the player go out of bounds!
        if transform.translation.length() > 700.0{
            // transform.translation = Vec3{x:0., y:0., z:0.};
            transform.translation = -transform.translation;
        }

        if keyboard.pressed(KeyCode::Right){
            *player = Direction::Right;
            transform.rotation = Quat::from_rotation_z(3.14 * 1.5);//works swimmingly!
        }else if keyboard.pressed(KeyCode::Down){
            *player = Direction::Down;
            transform.rotation = Quat::from_rotation_z(3.14 * 1.);//works swimmingly!
        }else if keyboard.pressed(KeyCode::Left){
            *player = Direction::Left;
            transform.rotation = Quat::from_rotation_z(3.14 * 0.5);//works swimmingly!
        }else if keyboard.pressed(KeyCode::Up){
            *player = Direction::Up;
            transform.rotation = Quat::from_rotation_z(3.14 * 0.);//works swimmingly!
        }
    }
}

//note: we'll give the ai a tuple for their velocity and have the below code affect the velocity.
//then we have the ai move according to their velocity (meaning that they accelerate toward their targets.)
//also, depending on which absolute value in the tuple is larger, and whether it's positive or negative, we will rotate the sprite accordingly!
fn enemy_ai(
    time: Res<Time>, 
    mut enemies: Query<(&mut Enemy, &mut Transform)>,//the enemies.
    mut target: Query<(&mut Person, &mut Transform, Without<Enemy>)>,//the player
    mut exit: EventWriter<AppExit>//for exiting the system
){
    for(mut pawn, mut transform) in &mut enemies{
        for(mut direction, mut targetpos, _) in &mut target{
            if transform.translation.distance(targetpos.translation) < 5.{
                exit.send(AppExit);//this works!
            }
            // transform.look_at(targetpos.translation, Vec3::Z);
            // transform.look_at(Vec3::NEG_Y, targetpos.translation);
            if targetpos.translation.y > transform.translation.y{
                // transform.translation.y += pawn.speed * time.delta_seconds();
                if pawn.velocity.1 > pawn.speed{pawn.velocity.1 = pawn.speed;}
                else{pawn.velocity.1 += pawn.acceleration * time.delta_seconds();}
            }else if targetpos.translation.y < transform.translation.y{
                // transform.translation.y -= pawn.speed * time.delta_seconds();
                if pawn.velocity.1 < -pawn.speed{pawn.velocity.1 = -pawn.speed;}
                else{pawn.velocity.1 -= pawn.acceleration * time.delta_seconds();}
            }
            if targetpos.translation.x > transform.translation.x{
                // transform.translation.x += pawn.speed * time.delta_seconds();
                if pawn.velocity.0 > pawn.speed{pawn.velocity.0 = pawn.speed;}
                else{pawn.velocity.0 += pawn.acceleration * time.delta_seconds();}
            }else if targetpos.translation.x < transform.translation.x{
                // transform.translation.x -= pawn.speed * time.delta_seconds();
                if pawn.velocity.0 < -pawn.speed{pawn.velocity.0 = -pawn.speed;}
                else{pawn.velocity.0 -= pawn.acceleration * time.delta_seconds();}
            }

            //do our rotation
            // transform.rotation = Quat::from_rotation_z(3.14 * 0.5);//works swimmingly!
            //if y.abs > x.abs //it will face up or down
            if pawn.velocity.1.abs() > pawn.velocity.0.abs(){
                //if y > 0 //it faces up
                if pawn.velocity.1 > 0.{
                    //if y.abs < x.abs * 2 //it will be diagonal
                    if pawn.velocity.1.abs() < pawn.velocity.0.abs() * 2.0 {
                        //if x > 0 //up-right
                        if pawn.velocity.0 > 0. {
                            transform.rotation = Quat::from_rotation_z(3.14 * 1.75);
                        }
                        //else //up-left
                        else{transform.rotation = Quat::from_rotation_z(3.14 * 0.25);}
                    }
                    //else //faces up
                    else{transform.rotation = Quat::from_rotation_z(0.);}
                }
                else{
                    //if y.abs < x.abs * 2 //it will be diagonal
                    if pawn.velocity.1.abs() < pawn.velocity.0.abs() * 2.0 {
                        //if x > 0 //up-right
                        if pawn.velocity.0 > 0. {
                            transform.rotation = Quat::from_rotation_z(3.14 * 1.25);
                        }
                        //else //up-left
                        else{transform.rotation = Quat::from_rotation_z(3.14 * 0.75);}
                    }
                    //else //faces down
                    else{transform.rotation = Quat::from_rotation_z(3.14 * 1.);}
                }
            }
            else{
                //if x > 0 //it faces right
                if pawn.velocity.0 > 0.{
                    //if x.abs < y.abs * 2 //it will be diagonal
                    if pawn.velocity.0.abs() < pawn.velocity.1.abs() * 2.0 {
                        //if y > 0 //up-right
                        if pawn.velocity.1 > 0. {
                            transform.rotation = Quat::from_rotation_z(3.14 * 1.75);
                        }
                        //else //down-right
                        else{transform.rotation = Quat::from_rotation_z(3.14 * 1.25);}
                    }
                    //else //faces right
                    else{transform.rotation = Quat::from_rotation_z(3.14 * 1.5);}
                }
                else{
                    //if x.abs < y.abs * 2 //it will be diagonal
                    if pawn.velocity.0.abs() < pawn.velocity.1.abs() * 2.0 {
                        //if y > 0 //up-left
                        if pawn.velocity.1 > 0. {
                            transform.rotation = Quat::from_rotation_z(3.14 * 0.25);
                        }
                        //else //down-left
                        else{transform.rotation = Quat::from_rotation_z(3.14 * 0.75);}
                    }
                    //else //faces left
                    else{transform.rotation = Quat::from_rotation_z(3.14 * 0.5);}
                }
            }
            //move our character
            transform.translation.x += pawn.velocity.0 * time.delta_seconds();
            transform.translation.y += pawn.velocity.1 * time.delta_seconds();
        }
    }
}
//also for ai sprites, let's give them the shape of manta rays, with a black and white color scheme,
//where some parts of them hide in the white and it appears larger in the dark. 
//We could even spook the player with this potentially with a large enough sprite that appears small at the beginning!

fn start(mut commands: Commands, 
        asset_server: Res<AssetServer>
){
    commands.spawn(Camera2dBundle::default());
    //background image
    commands.spawn(SpriteBundle{
        texture: asset_server.load("sprites/background1.png"),
        transform: Transform::from_xyz(0., 0., -100.),
        ..default()
    });
    commands.spawn((
        Person{
            name: "Reu".to_string(), 
        },
        SpriteBundle{
            texture:asset_server.load("sprites/PlayerSprite.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        }, 
        Direction::Up
    ));
}

fn main() {
    App::new()
        //adding plugins
        .add_plugins((
            DefaultPlugins//2D/3D renderer, asset-loading, UI system, window, input, etc.
            //PhysicsPlugins::default()
        ))
        //void start()
        .add_systems(Startup, (start))
        //add our system to the update schedule.
        //sounds line Update()
        .add_systems(Update, (greet_people, add_people, control_player, enemy_ai))
        // .add_systems(Update, (control_player, enemy_ai))
        .run();
}
