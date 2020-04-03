use propagators::network::Network;
use propagators::content_float::Float;
use propagators::content_supported::Supported;

#[test]
fn test_network_supported_float_add() {
    let mut network : Network<Supported<Float>> = Network::new();

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
    let mut network:Network<Supported<Float>> = Network::new();

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

//#[test]
//fn test_network_supported_float_constraint_product_triangle() {
    //let mut network:Network<Supported<Float>> = Network::new();

    //let ratio = network.make_cell();

    //let a = network.make_cell();
    //let b = network.make_cell();
    //let c = network.make_cell();
    //let d = network.make_cell();

    //network.write_cell(
        //a, 
        //Supported::new(2., &[String::from("foo")])
    //);

    //network.write_cell(
        //b, 
        //Supported::new(4., &[String::from("foo"), String::from("bar")])
    //);

    //network.write_cell(
        //c, 
        //Supported::new(3., &[String::from("baz")])
    //);

    //network.write_cell(
        //d, 
        //Supported::new(6., &[String::from("quux")])
    //);

    ////FIXME
    //network.constraint_product(a, ratio, b);
    //network.constraint_product(c, ratio, d);

    //network.run();

    //let expected = Supported::new(2., &[
        //String::from("baz"), 
        //String::from("quux")
    //]);

    //let actual = network.read_cell(ratio).unwrap();
    
    //assert_eq!(expected, actual);
//}

