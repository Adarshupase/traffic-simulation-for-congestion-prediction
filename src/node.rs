
use crate::direction::Direction;
use crate::errors::Result;
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DirectionProbability {
    right_left: f32,
    right_down: f32,
    right_up: f32,
    left_up: f32,
    left_down: f32,
    left_right: f32,
    up_down: f32,
    up_left: f32,
    up_right: f32,
    down_right: f32,
    down_left: f32,
    down_up: f32,
}

impl DirectionProbability {
    pub fn new(rng: &mut impl Rng) -> Self {
        Self {
            right_left:0.3,
            right_down:0.2,
            right_up: 0.4,
            left_up:0.5,
            left_down: 0.8,
            left_right:0.6,
            up_down: 0.8,
            up_left:0.6,
            up_right: 0.3,
            down_right: 0.1,
            down_left: 0.0,
            down_up:0.7,
        }
    }

    pub fn get_probability(&self, from: Direction, to: Direction) -> f32 {
        match (from, to) {
            (Direction::RIGHT, Direction::LEFT) => self.right_left,
            (Direction::RIGHT, Direction::DOWN) => self.right_down,
            (Direction::RIGHT, Direction::UP) => self.right_up,
            (Direction::LEFT, Direction::UP) => self.left_up,
            (Direction::LEFT, Direction::DOWN) => self.left_down,
            (Direction::LEFT, Direction::RIGHT) => self.left_right,
            (Direction::UP, Direction::DOWN) => self.up_down,
            (Direction::UP, Direction::LEFT) => self.up_left,
            (Direction::UP, Direction::RIGHT) => self.up_right,
            (Direction::DOWN, Direction::RIGHT) => self.down_right,
            (Direction::DOWN, Direction::LEFT) => self.down_left,
            (Direction::DOWN, Direction::UP) => self.down_up,
            _ => 0.0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DirectionDensity {
    arriving_up: u32,
    arriving_down: u32,
    leaving_up: u32,
    leaving_down: u32,
    arriving_right: u32,
    arriving_left: u32,
    leaving_right: u32,
    leaving_left: u32,
}

impl DirectionDensity {
    pub fn new(rng: &mut impl Rng) -> Self {
        Self {
            arriving_up:2,
            arriving_down: 3,
            leaving_up:4,
            leaving_down:5,
            arriving_right: 6,
            arriving_left: 7,
            leaving_right: 5,
            leaving_left:3 ,
        }
    }

    pub fn get_arriving(&self, direction: Direction) -> u32 {
        match direction {
            Direction::UP => self.arriving_up,
            Direction::DOWN => self.arriving_down,
            Direction::RIGHT => self.arriving_right,
            Direction::LEFT => self.arriving_left,
            Direction::EndOfMap => 0,
        }
    }

    pub fn get_leaving(&self, direction: Direction) -> u32 {
        match direction {
            Direction::UP => self.leaving_up,
            Direction::DOWN => self.leaving_down,
            Direction::RIGHT => self.leaving_right,
            Direction::LEFT => self.leaving_left,
            Direction::EndOfMap => 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: usize, 
    pub node_prob: DirectionProbability,
    pub directional_traffic: DirectionDensity,
    pub connections: HashMap<Direction, Option<usize>>, // Make connections public
}


impl Node {
    pub fn new(id: usize, rng: &mut impl Rng) -> Self {
        Self {
            id,
            node_prob: DirectionProbability::new(rng),
            directional_traffic: DirectionDensity::new(rng),
            connections: HashMap::new(),
        }
    }

    pub fn get_connection(&self, direction: Direction) -> Option<usize> {
        self.connections.get(&direction).copied().flatten()
    }

    pub fn set_traffic(&mut self, direction: Direction, arriving: u32, leaving: u32) {
        match direction {
            Direction::UP => {
                self.directional_traffic.arriving_up = arriving;
                self.directional_traffic.leaving_up = leaving;
            }
            Direction::DOWN => {
                self.directional_traffic.arriving_down = arriving;
                self.directional_traffic.leaving_down = leaving;
            }
            Direction::RIGHT => {
                self.directional_traffic.arriving_right = arriving;
                self.directional_traffic.leaving_right = leaving;
            }
            Direction::LEFT => {
                self.directional_traffic.arriving_left = arriving;
                self.directional_traffic.leaving_left = leaving;
            }
            Direction::EndOfMap => {}
        }
    }
}
