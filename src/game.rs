use super::types::*;
const JOUSTER_HEIGHT: usize = 20;
const JOUSTER_WIDTH: usize = 30;
const JOUSTER_ACCELERATION_HORIZONTAL: f64 = 0.1;
const MAX_HORIZONTAL_VELOCITY: f64 = 3.0;
const MAX_VERTICAL_VELOCITY: f64 = 3.0;
const GRAVITY: f64 = 0.03;

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

        // Set Braking if moving in opposite direction
        // TODO: Use epsilon
        match (
            right_pressed(),
            left_pressed(),
            self.player.jouster.velocity.x > 0.0,
            self.player.jouster.velocity.x < 0.0,
            &self.player.jouster.state,
        ) {
            (true, true, _, _, _) => {}
            (true, false, true, false, JousterState::Braking) => {
                self.player.jouster.state = JousterState::Walking;
            }
            (false, true, false, true, JousterState::Braking) => {
                self.player.jouster.state = JousterState::Walking;
            }
            (false, true, true, false, JousterState::Walking) => {
                self.player.jouster.state = JousterState::Braking;
            }
            (true, false, false, true, JousterState::Walking) => {
                self.player.jouster.state = JousterState::Braking;
            }
            _ => {}
        }

        if self.player.jouster.velocity.x == 0.0
            && self.player.jouster.state == JousterState::Braking
        {
            self.player.jouster.state = JousterState::Walking;
        }

        // TODO: Handle skipping past 0
        if self.player.jouster.state == JousterState::Braking {
            self.player.jouster.acceleration.x =
                match self.player.jouster.velocity.x.is_sign_positive() {
                    false => JOUSTER_ACCELERATION_HORIZONTAL,
                    true => -JOUSTER_ACCELERATION_HORIZONTAL,
                }
        }

        if space_pressed() {
            self.player.jouster.jump();
        }

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
    pub state: JousterState,
    pub direction: Direction,
}

#[derive(PartialEq)]
pub enum JousterState {
    Flying,
    Walking,
    Downed,
    WalkingDismounted,
    Braking,
}

#[derive(PartialEq)]
pub enum Direction {
    Left,
    Right,
    Still,
}

use Direction::*;
use JousterState::*;
impl Jouster {
    fn new(position: Position) -> Jouster {
        Jouster {
            position,
            velocity: Velocity::zero(),
            acceleration: Acceleration::zero(),
            mounted: true,
            width: JOUSTER_WIDTH,
            height: JOUSTER_HEIGHT,
            state: Walking,
            direction: Still,
        }
    }

    /// Physics for Jouster
    /// Constant acceleration and max velocity works well for acceleration
    /// Braking has different controls - one button press will initiate breaking
    /// Braking State: Constant deceleration to 0 velocity - unless button pressed to cancel
    /// by continuing in same direction.
    /// Flight:
    /// Acceleration needs to last several frames.
    /// Then trigger delay before next possible flight trigger.
    fn update(&mut self) -> () {
        self.velocity += self.acceleration;

        if self.velocity.x > MAX_HORIZONTAL_VELOCITY {
            self.velocity.x = MAX_HORIZONTAL_VELOCITY;
        } else if self.velocity.x < -MAX_HORIZONTAL_VELOCITY {
            self.velocity.x = -MAX_HORIZONTAL_VELOCITY;
        }

        if self.velocity.y > MAX_VERTICAL_VELOCITY {
            self.velocity.y = MAX_VERTICAL_VELOCITY;
        } else if self.velocity.y < -MAX_VERTICAL_VELOCITY {
            self.velocity.y = -MAX_VERTICAL_VELOCITY;
        }

        self.position += self.velocity;

        if self.position.x > ARENA_WIDTH as f64 {
            self.position.x = 0.0;
        } else if self.position.x < 0.0 {
            self.position.x = ARENA_WIDTH as f64;
        }

        if self.position.y < 0.0 {
            // TODO: Handle any platforms
            self.position.y = 0.0;
            self.velocity.y = 0.0;
            self.acceleration.y = 0.0;
            if self.state == Flying {
                self.state = Walking;
            }
        }

        if self.state == Flying {
            self.acceleration.y -= GRAVITY;
        }
    }

    fn jump(&mut self) -> () {
        if self.state == Walking {
            self.position.y += JOUSTER_HEIGHT as f64;
        }
        self.acceleration.y += 0.04;
        self.velocity.y += 0.05;
        self.state = Flying;
    }
    // TODO: Timer when downed/unmounted etc.
}

struct Platform {}
