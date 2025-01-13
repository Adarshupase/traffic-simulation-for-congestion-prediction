

mod direction;
mod node;
mod traffic_network;
mod errors;

use traffic_network::TrafficNetwork;
use direction::Direction;
fn main() {
    let mut network = TrafficNetwork::new(5, 5); // Create a 5x5 grid
    
    // Connect nodes in the network (adjust based on your layout)
    network.connect_nodes(0, 1, Direction::RIGHT).unwrap();
    network.connect_nodes(1, 2, Direction::RIGHT).unwrap();
    network.connect_nodes(1, 6, Direction::DOWN).unwrap();
    network.connect_nodes(7, 2, Direction::UP).unwrap();

    // Get traffic for node 1 at view level 0
    let node_traffic = network.get_node_traffic(1, 0);
     // Get traffic for node 1 at view level 3
    let node_traffic2 = network.get_node_traffic(1, 3);

    
    // Print the traffic for node 1
    println!("Traffic for node 1: {:?}", node_traffic);
    println!("Traffic for node 1: {:?}", node_traffic2);
}
