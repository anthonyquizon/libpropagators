use propagators::network::Network;
use propagators::content_float::Float;
use propagators::content_supported::Supported;

#[test]
fn test_network_supported_float_add() {
    let mut network : Network<Supported<Float, String>> = Network::new();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();

    network.write_cell(
        a, 
        Supported::new(Float::new(1.), &[String::from("foo")])
    );

    network.write_cell(
        b, 
        Supported::new(Float::new(2.), &[String::from("bar")])
    );

    network.propagator_add(a, b, c);

    network.run();

    let expected = &Supported::new(Float::new(3.), &[
        String::from("foo"), String::from("bar")
    ]);

    let actual = network.read_cell(c);
    
    assert_eq!(expected, actual);
}

#[test]
fn test_network_supported_float_constraint_add() {
    let mut network:Network<Supported<Float, String>> = Network::new();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();

    network.write_cell(
        b, 
        Supported::new(Float::new(2.), &[String::from("foo")])
    );

    network.write_cell(
        c, 
        Supported::new(Float::new(3.), &[String::from("bar")])
    );

    network.constraint_add(a, b, c);

    network.run();

    let expected = &Supported::new(Float::new(1.), &[
        String::from("foo"), String::from("bar")
    ]);

    let actual = network.read_cell(a);
    
    assert_eq!(expected, actual);
}

#[test]
fn test_network_supported_float_constraint_triangle() {
    let mut network:Network<Supported<Float, String>> = Network::new();

    let ratio = network.make_cell();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();
    let d = network.make_cell();

    network.write_cell(
        a, 
        Supported::new(Float::new(2.), &[String::from("foo")])
    );

    network.write_cell(
        b, 
        Supported::new(Float::new(4.), &[String::from("foo"), String::from("bar")])
    );

    network.write_cell(
        c, 
        Supported::new(Float::new(3.), &[String::from("baz")])
    );
    
    network.constraint_similar_triangles(a, b, c, d);

    network.run();

    let expected = &Supported::new(
        Float::new(6.), &[
            String::from("foo"), String::from("bar"), String::from("baz")
        ]
    );

    let actual = network.read_cell(d);
    
    assert_eq!(expected, actual);
}

