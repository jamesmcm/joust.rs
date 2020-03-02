use super::types::*;
const JOUSTER_HEIGHT: usize = 20;
const JOUSTER_WIDTH: usize = 30;
const JOUSTER_ACCELERATION_HORIZONTAL: f64 = 0.1;
const MAX_HORIZONTAL_VELOCITY: f64 = 4.0;

const ARENA_WIDTH: usize = 800;
use super::input::*;

pub struct Game {
    pub player: Player,
    pub other_jousters: Vec<Jouster>,
}

enum GameState {
    StartMenu,
    Play,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player: Player::new(Position::from((400.0, 0.0))), //TODO: Handle dynamic canvas
            other_jousters: Vec::new(),
        }
    }

    pub fn tick(&mut self) -> () {
        // Input
        self.player.jouster.acceleration.x = match (left_pressed(), right_pressed()) {
            (false, false) => 0.0,
            (true, false) => -JOUSTER_ACCELERATION_HORIZONTAL,
            (false, true) => JOUSTER_ACCELERATION_HORIZONTAL,
            (true, true) => 0.0,
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

enum JousterState {
    Flying,
    Walking,
    Downed,
    WalkingDismounted,
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

        if self.position.x > ARENA_WIDTH as f64 {
            self.position.x = 0.0;
        } else if self.position.x < 0 as f64 {
            self.position.x = ARENA_WIDTH as f64;
        }

        // TODO: Timer when downed/unmounted etc.
    }
}

struct Platform {}
