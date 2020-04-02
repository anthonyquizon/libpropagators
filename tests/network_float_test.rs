use propagators::network::Network;
use propagators::content_float::Float;


#[test]
fn test_network_Float_read_write() {
    let mut network : Network<Float> = Network::new();

    let a = network.make_cell();
    network.write_cell(a, Float::new(1.));
    network.run();

    let expected = &Float::new(1.);
    let actual = network.read_cell(a);
    
    assert_eq!(expected, actual);
}


#[test]
fn test_network_Float_add() {
    let mut network : Network<Float> = Network::new();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();

    network.write_cell(a, Float::new(1.));
    network.write_cell(b, Float::new(2.));

    network.propagator_add(a, b, c);

    network.run();

    let expected = &Float::new(3.);
    let actual = network.read_cell(c);
    
    assert_eq!(expected, actual);
}

#[test]
fn test_network_Float_multiply() {
    let mut network : Network<Float> = Network::new();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();

    network.write_cell(a, Float::new(1.5));
    network.write_cell(b, Float::new(2.));

    network.propagator_multiply(a, b, c);

    network.run();

    let expected = &Float::new(3.);
    let actual = network.read_cell(c);
    
    assert_eq!(expected, actual);
}

#[test]
fn test_network_Float_constraint_add() {
    let mut network:Network<Float> = Network::new();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();

    network.label_cell(a, "a");
    network.label_cell(b, "b");
    network.label_cell(c, "c");

    network.write_cell(b, Float::new(2.));
    network.write_cell(c, Float::new(3.));

    network.constraint_add(a, b, c);

    network.run();

    let expected = &Float::new(1.);
    let actual = network.read_cell(a);

    assert_eq!(expected, actual);
}


#[test]
fn test_network_Float_constraint_product_a() {
    let mut network:Network<Float> = Network::new();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();

    network.write_cell(b, Float::new(2.));
    network.write_cell(c, Float::new(3.));

    network.constraint_product(a, b, c);

    network.run();

    let expected = &Float::new(1.5);
    let actual = network.read_cell(a);
    
    assert_eq!(expected, actual);
}


#[test]
fn test_network_Float_constraint_product_b() {
    let mut network:Network<Float> = Network::new();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();

    network.write_cell(a, Float::new(1.5));
    network.write_cell(c, Float::new(3.));

    network.constraint_product(a, b, c);

    network.run();

    let expected = &Float::new(2.);
    let actual = network.read_cell(b);
    
    assert_eq!(expected, actual);
}

#[test]
fn test_network_Float_constraint_product_c() {
    let mut network:Network<Float> = Network::new();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();

    network.write_cell(a, Float::new(1.5));
    network.write_cell(b, Float::new(2.));

    network.constraint_product(a, b, c);

    network.run();

    let expected = &Float::new(3.);
    let actual = network.read_cell(c);
    
    assert_eq!(expected, actual);
}

#[test]
fn test_network_Float_constraint_product_triangle_ratio() {
    let mut network:Network<Float> = Network::new();

    let ratio = network.make_cell();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();
    let d = network.make_cell();

    network.write_cell(a, Float::new(2.));
    network.write_cell(b, Float::new(4.));

    network.write_cell(c, Float::new(3.));
    network.write_cell(d, Float::new(6.));

    network.constraint_product(a, ratio, b);
    network.constraint_product(c, ratio, d);

    network.run();

    let expected = &Float::new(2.);
    let actual = network.read_cell(ratio);
    
    assert_eq!(expected, actual);
}

#[test]
fn test_network_Float_constraint_similar_triangles() {
    let mut network:Network<Float> = Network::new();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();
    let d = network.make_cell();

    network.label_cell(a, "a");
    network.label_cell(b, "b");
    network.label_cell(c, "c");
    network.label_cell(d, "d");

    network.write_cell(a, Float::new(2.));
    network.write_cell(b, Float::new(4.));
    network.write_cell(c, Float::new(3.));

    network.constraint_similar_triangles(a, b, c, d);

    network.run();

    let expected = &Float::new(6.);
    let actual = network.read_cell(d);
    
    assert_eq!(expected, actual);
}

//////TODO test contradictions
