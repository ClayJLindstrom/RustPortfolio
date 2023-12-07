use bevy::prelude::*;
// use bevy::render::render_resource::Texture;
// use bevy_xpbd_2d::prelude::*; //doesn't work.
use bevy::app::AppExit;

mod rigidbody2d;

//create some people
#[derive(Component)]
struct Person {
    name: String,
}

#[derive(Component)]
struct Enemy {
    speed: f32,
    acceleration: f32,
    velocity: (f32, f32),
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

//for spawning enemies at a regular basis.
fn add_people(
    time: Res<Time>,                //time
    mut commands: Commands,         //for spawning enemies
    asset_server: Res<AssetServer>, //for spawning from assets.
) {
    //speed and acceleration of enemies as we go,
    // as well as how far away we want to spawn our enemies.
    let range: f32 = 700.0;
    let local_speed: f32 = 50.0 + time.elapsed_seconds();
    let local_accel: f32 = local_speed * 2.0 + time.elapsed_seconds() / 20.0;

    //spawn in 4 enemies every few seconds.
    if time.elapsed_seconds() % 4. <= time.delta_seconds() {
        commands.spawn((
            Enemy {
                speed: local_speed,
                acceleration: local_accel,
                velocity: (0., 0.),
            },
            SpriteBundle {
                texture: asset_server.load("sprites/EnemySprite.png"),
                transform: Transform::from_xyz(range, range, 0.),
                ..default()
            },
        ));
        commands.spawn((
            Enemy {
                speed: local_speed,
                acceleration: local_accel,
                velocity: (0., 0.),
            },
            SpriteBundle {
                texture: asset_server.load("sprites/EnemySprite.png"),
                transform: Transform::from_xyz(-range, range, 0.),
                ..default()
            },
        ));
        commands.spawn((
            Enemy {
                speed: local_speed,
                acceleration: local_accel,
                velocity: (0., 0.),
            },
            SpriteBundle {
                texture: asset_server.load("sprites/EnemySprite.png"),
                transform: Transform::from_xyz(range, -range, 0.),
                ..default()
            },
        ));
        commands.spawn((
            Enemy {
                speed: local_speed,
                acceleration: local_accel,
                velocity: (0., 0.),
            },
            SpriteBundle {
                texture: asset_server.load("sprites/EnemySprite.png"),
                transform: Transform::from_xyz(-range, -range, 0.),
                ..default()
            },
        ));
    }
}

//player controls.
fn control_player(
    time: Res<Time>,                                              // to keep track of time
    mut sprite_position: Query<(&mut Direction, &mut Transform)>, //player position
    keyboard: Res<Input<KeyCode>>,                                // player input
) {
    for (mut player, mut transform) in &mut sprite_position {
        match *player {
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Right => transform.translation.x += 150. * time.delta_seconds(),
            Direction::Left => transform.translation.x -= 150. * time.delta_seconds(),
        }
        //don't let the player go out of bounds!
        if transform.translation.length() > 700.0 {
            // transform.translation = Vec3{x:0., y:0., z:0.};
            transform.translation = -transform.translation;
        }

        if keyboard.pressed(KeyCode::Right) {
            *player = Direction::Right;
            transform.rotation = Quat::from_rotation_z(3.14 * 1.5); //works swimmingly!
        } else if keyboard.pressed(KeyCode::Down) {
            *player = Direction::Down;
            transform.rotation = Quat::from_rotation_z(3.14 * 1.); //works swimmingly!
        } else if keyboard.pressed(KeyCode::Left) {
            *player = Direction::Left;
            transform.rotation = Quat::from_rotation_z(3.14 * 0.5); //works swimmingly!
        } else if keyboard.pressed(KeyCode::Up) {
            *player = Direction::Up;
            transform.rotation = Quat::from_rotation_z(3.14 * 0.); //works swimmingly!
        }
    }
}

//for collision between characters
fn circle_collision(enemy: Vec3, player: Vec3) -> bool {
    return enemy.distance(player) < 5.;
}

//for handling the rotation of enemies.
fn enemy_rotation(velocity: (f32, f32)) -> Quat {
    //do our rotation
    // transform.rotation = Quat::from_rotation_z(3.14 * 0.5);//works swimmingly!
    //if y.abs > x.abs //it will face up or down
    if velocity.1.abs() > velocity.0.abs() {
        //if y > 0 //it faces up
        if velocity.1 > 0. {
            //if y.abs < x.abs * 2 //it will be diagonal
            if velocity.1.abs() < velocity.0.abs() * 2.0 {
                //if x > 0 //up-right
                if velocity.0 > 0. {
                    return Quat::from_rotation_z(3.14 * 1.75);
                }
                //else //up-left
                return Quat::from_rotation_z(3.14 * 0.25);
            }
            //else //faces up
            return Quat::from_rotation_z(0.);
        } else {
            //if y.abs < x.abs * 2 //it will be diagonal
            if velocity.1.abs() < velocity.0.abs() * 2.0 {
                //if x > 0 //up-right
                if velocity.0 > 0. {
                    return Quat::from_rotation_z(3.14 * 1.25);
                }
                //else //up-left
                return Quat::from_rotation_z(3.14 * 0.75);
            }
            //else //faces down
            return Quat::from_rotation_z(3.14 * 1.);
        }
    } else {
        //if x > 0 //it faces right
        if velocity.0 > 0. {
            //if x.abs < y.abs * 2 //it will be diagonal
            if velocity.0.abs() < velocity.1.abs() * 2.0 {
                //if y > 0 //up-right
                if velocity.1 > 0. {
                    return Quat::from_rotation_z(3.14 * 1.75);
                }
                //else //down-right
                return Quat::from_rotation_z(3.14 * 1.25);
            }
            //else //faces right
            return Quat::from_rotation_z(3.14 * 1.5);
        } else {
            //if x.abs < y.abs * 2 //it will be diagonal
            if velocity.0.abs() < velocity.1.abs() * 2.0 {
                //if y > 0 //up-left
                if velocity.1 > 0. {
                    return Quat::from_rotation_z(3.14 * 0.25);
                }
                //else //down-left
                return Quat::from_rotation_z(3.14 * 0.75);
            }
            //else //faces left
            return Quat::from_rotation_z(3.14 * 0.5);
        }
    }
}
//note: we'll give the ai a tuple for their velocity and have the below code affect the velocity.
//then we have the ai move according to their velocity (meaning that they accelerate toward their targets.)
//also, depending on which absolute value in the tuple is larger, and whether it's positive or negative, we will rotate the sprite accordingly!
fn enemy_ai(
    time: Res<Time>,
    mut enemies: Query<(&mut Enemy, &mut Transform)>, //the enemies.
    mut target: Query<(&mut Person, &mut Transform, Without<Enemy>)>, //the player
    mut exit: EventWriter<AppExit>,                   //for exiting the system
) {
    for (mut pawn, mut transform) in &mut enemies {
        for (person, targetpos, _) in &mut target {
            println!("{}", person.name);
            // if transform.translation.distance(targetpos.translation) < 5.{
            if circle_collision(transform.translation, targetpos.translation) {
                exit.send(AppExit); //this works!
            }
            if targetpos.translation.y > transform.translation.y {
                // transform.translation.y += pawn.speed * time.delta_seconds();
                if pawn.velocity.1 > pawn.speed {
                    pawn.velocity.1 = pawn.speed;
                } else {
                    pawn.velocity.1 += pawn.acceleration * time.delta_seconds();
                }
            } else if targetpos.translation.y < transform.translation.y {
                // transform.translation.y -= pawn.speed * time.delta_seconds();
                if pawn.velocity.1 < -pawn.speed {
                    pawn.velocity.1 = -pawn.speed;
                } else {
                    pawn.velocity.1 -= pawn.acceleration * time.delta_seconds();
                }
            }
            if targetpos.translation.x > transform.translation.x {
                // transform.translation.x += pawn.speed * time.delta_seconds();
                if pawn.velocity.0 > pawn.speed {
                    pawn.velocity.0 = pawn.speed;
                } else {
                    pawn.velocity.0 += pawn.acceleration * time.delta_seconds();
                }
            } else if targetpos.translation.x < transform.translation.x {
                // transform.translation.x -= pawn.speed * time.delta_seconds();
                if pawn.velocity.0 < -pawn.speed {
                    pawn.velocity.0 = -pawn.speed;
                } else {
                    pawn.velocity.0 -= pawn.acceleration * time.delta_seconds();
                }
            }

            //do our rotation
            transform.rotation = enemy_rotation(pawn.velocity);

            //move our character
            transform.translation.x += pawn.velocity.0 * time.delta_seconds();
            transform.translation.y += pawn.velocity.1 * time.delta_seconds();
        }
    }
}
//also for ai sprites, let's give them the shape of manta rays, with a black and white color scheme,
//where some parts of them hide in the white and it appears larger in the dark.
//We could even spook the player with this potentially with a large enough sprite that appears small at the beginning!

fn start(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    //background image
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/background1.png"),
        transform: Transform::from_xyz(0., 0., -100.),
        ..default()
    });
    commands.spawn((
        Person {
            name: "Reu".to_string(),
        },
        SpriteBundle {
            texture: asset_server.load("sprites/PlayerSprite.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Direction::Up,
    ));
}

fn start_game() {
    App::new()
        //adding plugins
        .add_plugins(
            DefaultPlugins, //2D/3D renderer, asset-loading, UI system, window, input, etc.
                            //PhysicsPlugins::default()
        )
        //void start()
        .add_systems(Startup, start)
        //add our system to the update schedule.
        //sounds line Update()
        .add_systems(Update, (add_people, control_player, enemy_ai))
        // .add_systems(Update, (control_player, enemy_ai))
        .run();
}

fn main() {
    start_game();
}

//test collision
#[test]
fn test_collision() {
    //a false collision,
    // assert_eq!(circle_collision(Vec3{x: 0., y: 0., z: 0.}, Vec3{x: 0., y: 10., z: 0.}), false);
    assert_eq!(
        circle_collision(
            Vec3 {
                x: 0.,
                y: 0.,
                z: 0.
            },
            Vec3 {
                x: 0.,
                y: 10.,
                z: 0.
            }
        ),
        false
    );
    //a true collision.
    assert_eq!(
        circle_collision(
            Vec3 {
                x: 0.,
                y: 0.,
                z: 0.
            },
            Vec3 {
                x: 0.,
                y: 0.,
                z: 0.
            }
        ),
        true
    );
}
//test rotation
#[test]
fn test_rotation() {
    //if x > 0 //up-right
    assert_eq!(Quat::from_rotation_z(3.14 * 1.75), enemy_rotation((5., 5.)));
    //else //faces left
    assert_eq!(Quat::from_rotation_z(3.14 * 0.5), enemy_rotation((-5., 0.)));
    //faces down
    assert_eq!(Quat::from_rotation_z(3.14 * 1.), enemy_rotation((0., -5.)));
}
