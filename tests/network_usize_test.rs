
use propagators::network::{ Network };
use propagators::cell::{ Cell };

#[test]
fn test_network_add() {
    let mut network : Network<usize> = Network::new();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();

    network.write_cell(a, 1);
    network.write_cell(b, 2);

    network.propagator_add(a, b, c);

    network.run();

    let expected = 3;
    let actual = network.read_cell(c).unwrap();
    
    assert_eq!(expected, actual);
}

#[test]
fn test_network_constraint_add() {
    let mut network:Network<usize> = Network::new();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();

    network.write_cell(b, 2);
    network.write_cell(c, 3);

    network.constraint_add(a, b, c);

    network.run();

    let expected = 1;
    let actual = network.read_cell(a).unwrap();
    
    assert_eq!(expected, actual);
}

#[test]
fn test_network_constraint_product() {
    let mut network:Network<usize> = Network::new();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();

    network.write_cell(b, 2);
    network.write_cell(c, 6);

    network.constraint_product(a, b, c);

    network.run();

    let expected = 3;
    let actual = network.read_cell(a).unwrap();
    
    assert_eq!(expected, actual);
}

/*
#[test]
fn test_network_expr() {
    let mut network:Network<usize> = Network::new();

    let a = network.make_cell();
    let b = network.make_cell();
    
    network.write_cell(a, 3);
    network.write_cell(b, 2);

    let answer = network.expr_add(a, b);

    network.run();

    let expected = Cell::wrap(&5);
    let actual = network.read_cell(answer);
    
    assert_eq!(expected, actual);
}

#[test]
fn test_network_switch_control_on() {
    let mut network:Network<usize> = Network::new();

    let control = network.make_cell();
    let input = network.make_cell();
    let output = network.make_cell();
    
    network.write_cell(control, 3);
    network.write_cell(input, 2);

    network.propagator_switch(control, input, output);

    network.run();

    let expected = Cell::wrap(&2);
    let actual = network.read_cell(output);
    
    assert_eq!(expected, actual);
}

#[test]
#[should_panic]
fn test_network_switch_control_off() {
    let mut network:Network<usize> = Network::new();

    let control = network.make_cell();
    let input = network.make_cell();
    let output = network.make_cell();
    
    network.write_cell(control, 0);
    network.write_cell(input, 2);

    network.propagator_switch(control, input, output);

    network.run();

    let _ = network.read_cell(output);
}

*/
