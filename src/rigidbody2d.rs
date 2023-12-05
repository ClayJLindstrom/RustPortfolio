use bevy::prelude::*;
use bevy::ecs::component::Component;


//Global variables
static GRAVITY: f64 = 9.8;

pub mod basics{
    //Transform is already implemented by something. It is not needed.

    /////////////////////////////////////////////////
    /////////////////////RIGIDBODY///////////////////
    /////////////////////////////////////////////////
    pub struct Rigidbody2D{
        velocity: (f32, f32),
        angular_velocity: (f32, f32)
    }

    impl Rigidbody2D{
        pub fn new() -> Rigidbody2D{
            Rigidbody2D{
                velocity:(0.,0.),
                angular_velocity: (0.,0.)
            }
        }
    }
}