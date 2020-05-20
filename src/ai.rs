use super::game::{Jouster, Player, EPSILON, JOUSTER_ACCELERATION_HORIZONTAL, JOUSTER_HEIGHT};

#[derive(Clone, Copy)]
pub enum JousterAI {
    Hunter, // Follow player and aim to keep height at player_height + JOUSTER_HEIGHT
}

impl JousterAI {
    pub fn tick(&self, jouster: &mut Jouster, player: &Player) -> () {
        match self {
            Self::Hunter => {
                if jouster.position.x < player.jouster.position.x {
                    jouster.acceleration.x = JOUSTER_ACCELERATION_HORIZONTAL;
                } else {
                    jouster.acceleration.x = -JOUSTER_ACCELERATION_HORIZONTAL;
                }

                if jouster.position.y < player.jouster.position.y + (JOUSTER_HEIGHT as f64) {
                    jouster.jump();
                }
            }
        }
    }
}
