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

    let a = network.new_cell();
    let b = network.new_cell();
    let c = network.new_cell();

    network.write_cell(a, TruthManagementStore::new(&tms, Interval::new(0, 5)));
    network.write_cell(b, TruthManagementStore::new(&tms, Interval::new(2, 6)));

    //network.propagator_add(a, b, c);

    network.run();

    //let expected = Some(3);
    //let actual = network.read_cell(c);
    
    //assert_eq!(expected, actual);
}
