use super::types::*;
use crate::log;
const JOUSTER_HEIGHT: usize = 20;
const JOUSTER_WIDTH: usize = 30;
const JOUSTER_ACCELERATION: i32 = 1;
const MAX_HORIZONTAL_VELOCITY: i32 = 6;
use super::input::*;

pub struct Game {
    pub player: Player,
    pub other_jousters: Vec<Jouster>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player: Player::new(Position::from((400, 0))), //TODO: Handle dynamic canvas
            other_jousters: Vec::new(),
        }
    }

    pub fn tick(&mut self) -> () {
        unsafe {
            log(&format!("keystate: {:?}", KEYSTATE));
            log(&format!("player: {:?}", self.player.jouster.velocity));
        }
        // Input
        self.player.jouster.acceleration.x = match (left_pressed(), right_pressed()) {
            (false, false) => 0,
            (true, false) => -JOUSTER_ACCELERATION,
            (false, true) => JOUSTER_ACCELERATION,
            (true, true) => 0,
        };

        // Calculations
        self.player.jouster.update();
        for jouster in self.other_jousters.iter_mut() {
            jouster.update();
        }
    }
}

pub struct Player {
    pub jouster: Jouster,
    kills: i32,
}

impl Player {
    pub fn new(position: Position) -> Player {
        Player {
            jouster: Jouster::new(position),
            kills: 0,
        }
    }
}

pub struct Jouster {
    pub position: Position,
    velocity: Velocity,
    acceleration: Acceleration,
    mounted: bool,
    pub width: usize,
    pub height: usize,
}

impl Jouster {
    fn new(position: Position) -> Jouster {
        Jouster {
            position,
            velocity: Velocity::zero(),
            acceleration: Acceleration::zero(),
            mounted: true,
            width: JOUSTER_WIDTH,
            height: JOUSTER_HEIGHT,
        }
    }

    fn update(&mut self) -> () {
        self.velocity += self.acceleration;
        if self.velocity.x > MAX_HORIZONTAL_VELOCITY {
            self.velocity.x = MAX_HORIZONTAL_VELOCITY;
        } else if self.velocity.x < -MAX_HORIZONTAL_VELOCITY {
            self.velocity.x = -MAX_HORIZONTAL_VELOCITY;
        }
        self.position += self.velocity;

        // TODO: Timer when downed/unmounted etc.
    }
}

struct Platform {}
