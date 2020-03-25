/*
use propagators::network::{ Network };
use propagators::cell_tms::{ TruthManagementStore };
use propagators::tms::{ TruthManagementSystem };
use propagators::cell_interval::{ Interval };
use propagators::cell::{ Cell };
use std::cell::RefCell;
use std::rc::{Rc};

type Content = TruthManagementStore<Interval>;

#[test]
fn test_network_add() {
    let mut network_rc : Rc<RefCell<Network<Content>>> = Rc::new(RefCell::new(Network::new()));
    let mut tms : Rc<TruthManagementSystem<Content>> = Rc::new(TruthManagementSystem::new(&network_rc));

    let mut network = network_rc.borrow_mut();

    let barometer_height = network.make_cell();
    let barometer_shadow = network.make_cell();
    let building_height  = network.make_cell();
    let building_shadow  = network.make_cell();

    network.constraint_similar_triangles(
        barometer_shadow, 
        barometer_height, 
        building_shadow, 
        building_height
    );

    network.write_cell(
        building_height, 
        TruthManagementStore::new(&tms, Interval::new(5490, 5510), &[String::from("shadows")])
    );

    network.write_cell(
        building_height, 
        TruthManagementStore::new(&tms, Interval::new(30, 32), &[String::from("shadows")])
    );

    network.write_cell(
        barometer_shadow, 
        TruthManagementStore::new(&tms, Interval::new(36, 37), &[String::from("shadows")])
    );

    //network.write_cell(b, TruthManagementStore::new(&tms, Interval::new(2, 6)));

    //network.propagator_add(a, b, c);

    network.run();

    //let expected = Some(3);
    //let actual = network.read_cell(c);
    
    //assert_eq!(expected, actual);
}
*/
