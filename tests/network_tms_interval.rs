use propagators::network::{ Network };
use propagators::cell_tms::{ TruthManagementStore };
use propagators::tms::{ TruthManagementSystem };
use propagators::cell::{ Cell };
use std::rc::{Rc};

#[test]
fn test_network_add() {
    let mut network : Network<usize> = Network::new();
    let mut tms : TruthManagementSystem<usize> = TruthManagementSystem::new(&Rc::new(network));

    //let a = network.new_cell();
    //let b = network.new_cell();
    //let c = network.new_cell();

    //network.write_cell(a, 1);
    //network.write_cell(b, 2);

    //network.propagator_add(a, b, c);

    //network.run();

    //let expected = Some(3);
    //let actual = network.read_cell(c);
    
    //assert_eq!(expected, actual);
}
