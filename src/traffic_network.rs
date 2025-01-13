

use crate::direction::Direction;
use crate::node::Node;
use crate::errors::{Result, TrafficError};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct TrafficNetwork {
    nodes: Vec<Node>,
    rows: usize,
    cols: usize,
}
impl TrafficNetwork {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut rng = rand::thread_rng();
        let nodes = (0..rows * cols)
            .map(|id| Node::new(id, &mut rng))
            .collect();

        Self { nodes, rows, cols }
    }

    pub fn connect_nodes(&mut self, from_id: usize, to_id: usize, direction: Direction) -> Result<()> {
        if from_id >= self.nodes.len() || to_id >= self.nodes.len() {
            return Err(TrafficError::IndexOutOfBounds);
        }

        // Validate the connection based on grid layout
        let from_row = from_id / self.cols;
        let from_col = from_id % self.cols;
        let to_row = to_id / self.cols;
        let to_col = to_id % self.cols;

        match direction {
            Direction::RIGHT if to_col != from_col + 1 => return Err(TrafficError::InvalidRoadConnection),
            Direction::LEFT if from_col == 0 || to_col != from_col - 1 => {
                return Err(TrafficError::InvalidRoadConnection)
            }
            Direction::DOWN if to_row != from_row + 1 => return Err(TrafficError::InvalidRoadConnection),
            Direction::UP if from_row == 0 || to_row != from_row - 1 => {
                return Err(TrafficError::InvalidRoadConnection)
            }
            Direction::EndOfMap => return Ok(()),
            _ => {}
        }

        // Create bidirectional connection
        if let Some(node) = self.nodes.get_mut(from_id) {
            node.connections.insert(direction, Some(to_id));
        }
        if let Some(node) = self.nodes.get_mut(to_id) {
            node.connections.insert(direction.opposite(), Some(from_id));
        }
        Ok(())
    }
    pub fn is_linked(&self, from_id: usize, to_id: usize, direction: Direction) -> bool {
        if from_id >= self.nodes.len() || to_id >= self.nodes.len() {
            return false; // Invalid node ID
        }
        
        if let Some(connected_id) = self.nodes[from_id].get_connection(direction) {
            return connected_id == to_id;
        }
        
        false // No connection found in the specified direction
    }
    pub fn predict_traffic(
        &self,
        start_node: usize,
        direction: Direction,
        view_level: u32,
    ) -> Result<u32> {
        const MAX_VIEW_LEVEL: u32 = 5;
        if view_level > MAX_VIEW_LEVEL {
            return Err(TrafficError::MaxViewLevelReached);
        }

        let mut visited = HashSet::new();
        self.calculate_traffic_recursive(start_node, direction, view_level, &mut visited)
    }

    fn calculate_traffic_recursive(
        &self,
        node_id: usize,
        direction: Direction,
        view_level: u32,
        visited: &mut HashSet<usize>,
    ) -> Result<u32> {
        if view_level == 0 || visited.contains(&node_id) {
            return Ok(0);
        }

        visited.insert(node_id);
        let node = &self.nodes[node_id];
        let mut total_traffic = node.directional_traffic.get_arriving(direction);

        // Calculate incoming traffic based on direction-specific rules
        match direction {
            Direction::RIGHT => {
                // Traffic coming from left, down, and up
                for &incoming_dir in &[Direction::LEFT, Direction::DOWN, Direction::UP] {
                    if let Some(connected_id) = node.get_connection(incoming_dir) {
                        let prob = node.node_prob.get_probability(incoming_dir, direction);
                        let incoming = self.calculate_traffic_recursive(
                            connected_id,
                            incoming_dir,
                            view_level - 1,
                            visited,
                        )?;
                        total_traffic += (prob * incoming as f32) as u32;
                    }
                }
            }
            Direction::LEFT => {
                // Traffic coming from right, down, and up
                for &incoming_dir in &[Direction::RIGHT, Direction::DOWN, Direction::UP] {
                    if let Some(connected_id) = node.get_connection(incoming_dir) {
                        let prob = node.node_prob.get_probability(incoming_dir, direction);
                        let incoming = self.calculate_traffic_recursive(
                            connected_id,
                            incoming_dir,
                            view_level - 1,
                            visited,
                        )?;
                        total_traffic += (prob * incoming as f32) as u32;
                    }
                }
            }
            Direction::UP => {
                // Traffic coming from down, right, and left
                for &incoming_dir in &[Direction::DOWN, Direction::RIGHT, Direction::LEFT] {
                    if let Some(connected_id) = node.get_connection(incoming_dir) {
                        let prob = node.node_prob.get_probability(incoming_dir, direction);
                        let incoming = self.calculate_traffic_recursive(
                            connected_id,
                            incoming_dir,
                            view_level - 1,
                            visited,
                        )?;
                        total_traffic += (prob * incoming as f32) as u32;
                    }
                }
            }
            Direction::DOWN => {
                // Traffic coming from up, right, and left
                for &incoming_dir in &[Direction::UP, Direction::RIGHT, Direction::LEFT] {
                    if let Some(connected_id) = node.get_connection(incoming_dir) {
                        let prob = node.node_prob.get_probability(incoming_dir, direction);
                        let incoming = self.calculate_traffic_recursive(
                            connected_id,
                            incoming_dir,
                            view_level - 1,
                            visited,
                        )?;
                        total_traffic += (prob * incoming as f32) as u32;
                    }
                }
            }
            Direction::EndOfMap => {}
        }

        visited.remove(&node_id);
        Ok(total_traffic)
    }

    pub fn get_traffic_density(&self, view_level: u32) -> Vec<Vec<u32>> {
        let mut density = vec![vec![0; self.cols]; self.rows];
        for row in 0..self.rows {
            for col in 0..self.cols {
                let node_id = row * self.cols + col;
                for direction in [Direction::RIGHT, Direction::LEFT, Direction::UP, Direction::DOWN] {
                    if let Ok(traffic) = self.predict_traffic(node_id, direction, view_level) {
                        density[row][col] = density[row][col].max(traffic);
                    }
                }
            }
        }
        density
    }

    pub fn get_node_traffic(&self, node_id: usize, view_level: u32) -> HashMap<Direction, u32> {
        let mut traffic = HashMap::new();
        for direction in [Direction::RIGHT, Direction::LEFT, Direction::UP, Direction::DOWN] {
            if let Ok(value) = self.predict_traffic(node_id, direction, view_level) {
                traffic.insert(direction, value);
            }
        }
        traffic
    }
    pub fn print_network(&self) {
      
        for row in 0..self.rows {
            
            let mut row_output = String::new();
    
          
            for col in 0..self.cols {
                let node_id = row * self.cols + col;
                let node = &self.nodes[node_id];
    
                row_output.push_str("·");
    

                let mut right_connection = String::new();
                let mut down_connection = String::new();
    
                if let Some(connected_id) = node.get_connection(Direction::RIGHT) {

                    if self.nodes[connected_id].get_connection(Direction::LEFT).is_some() {
                        right_connection.push_str("→←");
                    } else {
                        right_connection.push_str("→");
                    }
                }
    

                if let Some(connected_id) = node.get_connection(Direction::DOWN) {
                    
                    if self.nodes[connected_id].get_connection(Direction::UP).is_some() {
                        down_connection.push_str("↓↑");
                    } else {
                        down_connection.push_str("↓");
                    }
                }
    
          
                row_output.push_str(&right_connection);
                if col < self.cols - 1 {
                    row_output.push_str(" "); 
                }
    
            }
    
          
            println!("{}", row_output);
        }
    }
    
    
    
    
}