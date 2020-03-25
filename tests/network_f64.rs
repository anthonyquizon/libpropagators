use propagators::network::{ Network };
use propagators::cell::{ Cell };

#[test]
fn test_network_f64_add() {
    let mut network : Network<f64> = Network::new();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();

    network.write_cell(a, 1.);
    network.write_cell(b, 2.);

    network.propagator_add(a, b, c);

    network.run();

    let expected = 3.;
    let actual = network.read_cell(c).unwrap();
    
    assert!((expected - actual).abs() < 1e-10);
}

#[test]
fn test_network_f64_constraint_add() {
    let mut network:Network<f64> = Network::new();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();

    network.write_cell(b, 2.);
    network.write_cell(c, 3.);

    network.constraint_add(a, b, c);

    network.run();

    let expected = 1.;
    let actual = network.read_cell(a).unwrap();
    
    assert!((expected - actual).abs() < 1e-10);
}

#[test]
fn test_network_f64_constraint_product() {
    let mut network:Network<f64> = Network::new();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();

    network.write_cell(b, 2.);
    network.write_cell(c, 3.);

    network.constraint_product(a, b, c);

    network.run();

    let expected = 1.5;
    let actual = network.read_cell(a).unwrap();
    
    assert!((expected - actual).abs() < 1e-10);
}
