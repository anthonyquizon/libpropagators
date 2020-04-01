use propagators::network::{ Network };
use propagators::tms::TruthManagementSystem;
use propagators::cell_tms::TruthManagementStore;
use propagators::cell_float::Float;
use std::cell::RefCell;
use std::rc::{Rc};

type Content = TruthManagementStore<Float>;

#[test]
fn test_network_tms_float_add() {
    let mut network_rc : Rc<RefCell<Network<Content>>> = Rc::new(RefCell::new(Network::new()));
    let mut tms_rc : Rc<TruthManagementSystem<Content>> = Rc::new(TruthManagementSystem::new(&network_rc));
    let mut network = network_rc.borrow_mut();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();

    network.write_cell(
        a, 
        TruthManagementStore::new(&tms_rc, &[
            (Float::new(1.), &[String::from("foo")])
        ])
    );

    network.write_cell(
        b, 
        TruthManagementStore::new(&tms_rc, &[
            (Float::new(2.), &[String::from("bar")])
        ])
    );

    network.propagator_add(a, b, c);

    network.run();

    let expected = TruthManagementStore::new(&tms_rc, &[
        (Float::new(3.), &[String::from("foo"), String::from("bar")])
    ]);

    let actual = network.read_cell(c).unwrap();
    
    assert_eq!(expected, actual);
}

#[test]
fn test_network_tms_float_constraint_add() {
    let mut network_rc : Rc<RefCell<Network<Content>>> = Rc::new(RefCell::new(Network::new()));
    let mut tms_rc : Rc<TruthManagementSystem<Content>> = Rc::new(TruthManagementSystem::new(&network_rc));
    let mut network = network_rc.borrow_mut();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();

    network.write_cell(
        b, 
        TruthManagementStore::new(&tms_rc, &[
            (Float::new(2.), &[String::from("foo")])
        ])
    );

    network.write_cell(
        c, 
        TruthManagementStore::new(&tms_rc, &[
            (Float::new(3.), &[String::from("bar")])
        ])
    );

    network.constraint_add(a, b, c);

    network.run();

    let expected = TruthManagementStore::new(&tms_rc, &[
        (Float::new(1.), &[String::from("foo"), String::from("bar")])
    ]);

    let actual = network.read_cell(a).unwrap();
    
    assert_eq!(expected, actual);
}

#[test]
fn test_network_tms_float_constraint_add_many_supports() {
    let mut network_rc : Rc<RefCell<Network<Content>>> = Rc::new(RefCell::new(Network::new()));
    let mut tms_rc : Rc<TruthManagementSystem<Content>> = Rc::new(TruthManagementSystem::new(&network_rc));
    let mut network = network_rc.borrow_mut();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();

    network.label_cell(a, "a");
    network.label_cell(b, "b");
    network.label_cell(c, "c");

    network.write_cell(
        b, 
        TruthManagementStore::new(&tms_rc, &[
            (Float::new(2.), &[String::from("foo")])
        ])
    );

    network.write_cell(
        c, 
        TruthManagementStore::new(&tms_rc, &[
            (Float::new(3.), &[String::from("bar")])
        ])
    );

    network.write_cell(
        c, 
        TruthManagementStore::new(&tms_rc, &[
            (Float::new(5.), &[String::from("baz")])
        ])
    );

    network.constraint_add(a, b, c);

    network.run();

    let expected = TruthManagementStore::new(&tms_rc, &[
        (Float::new(1.), &[String::from("foo"), String::from("bar")]),
        (Float::new(3.), &[String::from("foo"), String::from("baz")])
    ]);
    let actual = network.read_cell(a).unwrap();

    assert_eq!(expected, actual);
}

fn test_network_tms_float_constraint_add_contradiction() {
    let mut network_rc : Rc<RefCell<Network<Content>>> = Rc::new(RefCell::new(Network::new()));
    let mut tms_rc : Rc<TruthManagementSystem<Content>> = Rc::new(TruthManagementSystem::new(&network_rc));
    let mut network = network_rc.borrow_mut();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();

    network.label_cell(a, "a");
    network.label_cell(b, "b");
    network.label_cell(c, "c");

    network.write_cell(
        b, 
        TruthManagementStore::new(&tms_rc, &[
            (Float::new(2.), &[String::from("foo")])
        ])
    );

    network.write_cell(
        c, 
        TruthManagementStore::new(&tms_rc, &[
            (Float::new(3.), &[String::from("bar")])
        ])
    );

    network.write_cell(
        c, 
        TruthManagementStore::new(&tms_rc, &[
            (Float::new(5.), &[String::from("baz")])
        ])
    );

    network.constraint_add(a, b, c);

    network.run();

    let expected = TruthManagementStore::new(&tms_rc, &[
        (Float::new(1.), &[String::from("foo"), String::from("bar")]),
        (Float::new(3.), &[String::from("foo"), String::from("baz")])
    ]);
    let actual = network.read_cell(a).unwrap();

    assert_eq!(expected, actual);
}


