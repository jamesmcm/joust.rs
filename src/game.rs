use super::ai::JousterAI;
use super::types::*;
pub const JOUSTER_HEIGHT: usize = 20;
pub const JOUSTER_WIDTH: usize = 30;
pub const JOUSTER_ACCELERATION_HORIZONTAL: f64 = 0.1;
pub const MAX_HORIZONTAL_VELOCITY: f64 = 3.0;
pub const MAX_VERTICAL_VELOCITY: f64 = 3.0;
pub const GRAVITY: f64 = 0.03;
pub const EPSILON: f64 = 0.01;

pub const ARENA_WIDTH: usize = 800;
pub const ARENA_HEIGHT: usize = 600;
use super::input::*;

#[derive(Clone)]
pub struct Game {
    pub player: Player,
    pub other_jousters: Vec<Jouster>,
    pub platforms: Vec<Platform>,
}

enum GameState {
    StartMenu,
    Play,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            player: Player::new(Position::from((400.0, 20.0))), //TODO: Handle dynamic canvas
            other_jousters: vec![
                Jouster::new(Position::from((200.0, 50.0))),
                Jouster::new(Position::from((50.0, 30.0))),
                Jouster::new(Position::from((600.0, 80.0))),
                Jouster::new(Position::from((700.0, 230.0))),
                Jouster::new(Position::from((350.0, 320.0))),
            ],
            platforms: vec![
                Platform::new(Position::from((0.0, 0.0)), 800.0, 5.0),
                Platform::new(Position::from((350.0, 300.0)), 100.0, 5.0),
                Platform::new(Position::from((0.0, ARENA_HEIGHT as f64 - 5.0)), 800.0, 5.0),
            ],
        };
        game.other_jousters[0].set_state(Flying);
        game.other_jousters[0].set_ai(Some(JousterAI::Hunter));
        game.other_jousters[1].set_state(Flying);
        game.other_jousters[1].set_ai(Some(JousterAI::Hunter));
        game.other_jousters[2].set_state(Flying);
        game.other_jousters[2].set_ai(Some(JousterAI::Hunter));
        game.other_jousters[3].set_state(Flying);
        game.other_jousters[3].set_ai(Some(JousterAI::Hunter));
        game.other_jousters[4].set_state(Flying);
        game.other_jousters[4].set_ai(Some(JousterAI::Hunter));
        game
    }

    fn process_player(&mut self) -> () {
        // Input
        self.player.jouster.acceleration.x =
            match (left_pressed(), right_pressed(), self.player.jouster.state) {
                (false, false, _) => 0.0,
                (true, false, JousterState::Walking) => -JOUSTER_ACCELERATION_HORIZONTAL,
                (false, true, JousterState::Walking) => JOUSTER_ACCELERATION_HORIZONTAL,
                (true, false, JousterState::Flying) => -0.2 * JOUSTER_ACCELERATION_HORIZONTAL,
                (false, true, JousterState::Flying) => 0.2 * JOUSTER_ACCELERATION_HORIZONTAL,
                (true, true, _) => 0.0,
                (_, _, _) => 0.0,
            };

        // Set Braking if moving in opposite direction
        match (
            right_pressed(),
            left_pressed(),
            self.player.jouster.velocity.x > EPSILON,
            self.player.jouster.velocity.x < -EPSILON,
            self.player.jouster.state,
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

        if self.player.jouster.velocity.x.abs() < EPSILON
            && self.player.jouster.state == JousterState::Braking
        {
            self.player.jouster.state = JousterState::Walking;
        }

        if self.player.jouster.state == JousterState::Braking {
            self.player.jouster.acceleration.x = match self.player.jouster.velocity.x > EPSILON {
                false => 1.5 * JOUSTER_ACCELERATION_HORIZONTAL,
                true => -1.5 * JOUSTER_ACCELERATION_HORIZONTAL,
            }
        }

        if space_pressed() {
            self.player.jouster.jump();
        }
    }

    fn restart(&mut self) -> () {
        let dummygame = Self::new();

        *self = dummygame.clone();
    }

    pub fn tick(&mut self) -> () {
        Self::process_player(self);

        // AI
        for jouster in self.other_jousters.iter_mut() {
            if let Some(ai) = jouster.ai {
                ai.tick(jouster, &self.player);
            }
        }

        // Collision between Jousters
        let mut to_remove = vec![];

        for (i, j1) in self.other_jousters.iter_mut().enumerate() {
            match jouster_collision(&mut self.player.jouster, j1) {
                Some(false) => {
                    return self.restart();
                } // Kill player
                Some(true) => {
                    to_remove.push(i);
                    self.player.kills += 1;
                }
                None => {}
            }
        }

        for i in 0..self.other_jousters.len() {
            let (done, remaining) = self.other_jousters.split_at_mut(i + 1);
            if let Some(to_check) = done.iter_mut().last() {
                for (j, j2) in remaining.iter_mut().enumerate() {
                    match jouster_collision(to_check, j2) {
                        Some(false) => {
                            to_remove.push(i);
                        }
                        Some(true) => {
                            to_remove.push(j);
                        }
                        None => {}
                    }
                }
            }
        }

        for i in to_remove.into_iter() {
            self.other_jousters.remove(i);
        }

        // Calculations
        self.player.jouster.update(&self.platforms);
        for jouster in self.other_jousters.iter_mut() {
            jouster.update(&self.platforms);
        }
    }
}

fn jouster_collision(j1: &mut Jouster, j2: &mut Jouster) -> Option<bool> {
    let new_position2 = j2.position + j2.velocity;
    let new_position1 = j1.position + j1.velocity;
    if new_position1.y <= new_position2.y + (j2.height as f64)
        && new_position1.y + (j1.height as f64) >= new_position2.y
        && new_position1.x + (j1.width as f64) - new_position2.x > EPSILON
        && new_position1.x - (new_position2.x + (j2.width as f64)) < -EPSILON
    {
        if new_position1.y - new_position2.y > (JOUSTER_HEIGHT as f64) * 0.75 {
            Some(true)
        } else if new_position2.y - new_position1.y > (JOUSTER_HEIGHT as f64) * 0.75 {
            Some(false)
        } else {
            let v = j1.velocity.x.abs() + j2.velocity.x.abs();

            if j1.position.x > j2.position.x {
                j1.velocity.x = v;
                j2.velocity.x = -v;
            } else {
                j1.velocity.x = -v;
                j2.velocity.x = v;
            }
            None
        }
    } else {
        None
    }
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub struct Jouster {
    pub position: Position,
    velocity: Velocity,
    pub acceleration: Acceleration,
    mounted: bool,
    pub width: usize,
    pub height: usize,
    pub state: JousterState,
    pub direction: Direction,
    pub jump_delay: u32,
    ai: Option<JousterAI>,
}

#[derive(PartialEq, Clone, Copy)]
pub enum JousterState {
    Flying,
    Walking,
    Downed,
    WalkingDismounted,
    Braking,
}

#[derive(PartialEq, Clone, Copy)]
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
            jump_delay: 0,
            ai: None,
        }
    }
    fn set_state(&mut self, state: JousterState) {
        self.state = state;
    }
    fn set_ai(&mut self, ai: Option<JousterAI>) {
        self.ai = ai;
    }

    /// Physics for Jouster
    /// Constant acceleration and max velocity works well for acceleration
    /// Braking has different controls - one button press will initiate breaking
    /// Braking State: Constant deceleration to 0 velocity - unless button pressed to cancel
    /// by continuing in same direction.
    /// Flight:
    /// Acceleration needs to last several frames.
    /// Then trigger delay before next possible flight trigger.
    fn update(&mut self, platforms: &Vec<Platform>) -> () {
        // TODO: Do collision, then position, then velocity ?
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

        // Collision detection
        let new_position = self.position + self.velocity;
        let mut on_platform = false;
        for platform in platforms.iter() {
            if self.position.y >= platform.position.y
                && new_position.y <= platform.position.y + platform.width
                && self.position.x + (self.width as f64) >= platform.position.x
                && self.position.x <= platform.position.x + platform.length
            {
                on_platform = true
            }
            if self.position.y < platform.position.y
                && new_position.y + (self.height as f64) > platform.position.y
                && self.position.x + (self.width as f64) >= platform.position.x
                && self.position.x <= platform.position.x + platform.length
                && self.state == Flying
            {
                self.position.y = platform.position.y - (self.height as f64);
                self.velocity.y = -self.velocity.y;
            } else if self.position.y > platform.position.y
                && new_position.y < platform.position.y + platform.width
                && self.position.x + (self.width as f64) >= platform.position.x
                && self.position.x <= platform.position.x + platform.length
                && self.state == Flying
            {
                self.position.y = platform.position.y + platform.width;
                self.velocity.y = 0.0;
                self.acceleration.y = 0.0;
                self.state = Walking;
            }
        }

        if !on_platform {
            self.state = Flying
        }

        self.position = self.position + self.velocity;

        if self.position.x > ARENA_WIDTH as f64 {
            self.position.x = 0.0;
        } else if self.position.x < 0.0 {
            self.position.x = ARENA_WIDTH as f64;
        }

        if self.state == Flying {
            self.acceleration.y = -GRAVITY;
        }
        if self.jump_delay > 0 {
            self.jump_delay -= 1;
        }
    }

    pub fn jump(&mut self) -> () {
        if self.jump_delay > 0 {
            return;
        }

        self.jump_delay = 30;
        if self.state != Flying {
            self.velocity.y += 6.0;
        } else {
            self.velocity.y += 1.0;
        }
        self.state = Flying;
    }
    // TODO: Timer when downed/unmounted etc.
}

#[derive(Clone, Copy)]
pub struct Platform {
    pub position: Position,
    pub length: f64,
    pub width: f64,
}

impl Platform {
    fn new(position: Position, length: f64, width: f64) -> Platform {
        Platform {
            position,
            length,
            width,
        }
    }
}
