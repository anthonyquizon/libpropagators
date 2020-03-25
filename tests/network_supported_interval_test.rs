use propagators::network::{ Network };
use propagators::cell_supported::{ Supported };
use propagators::cell_interval::{ Interval };
use propagators::cell::{ Cell };

#[test]
fn test_network_supported_interval_add() {
    let mut network : Network<Supported<Interval>> = Network::new();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();

    network.write_cell(
        a, 
        Supported::new(Interval::new(0., 1.), &[String::from("foo")])
    );

    network.write_cell(
        b, 
        Supported::new(Interval::new(1., 2.), &[String::from("bar")])
    );

    network.propagator_add(a, b, c);

    network.run();

    let actual = network.read_cell(c).unwrap();
    let expected = Supported::new(Interval::new(1., 3.), &[String::from("foo"), String::from("bar")]);

    assert_eq!(expected, actual);
}

#[test]
fn test_network_supported_interval_sub() {
    let mut network : Network<Supported<Interval>> = Network::new();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();

    network.write_cell(
        a, 
        Supported::new(Interval::new(0., 1.), &[String::from("foo")])
    );

    network.write_cell(
        b, 
        Supported::new(Interval::new(1., 2.), &[String::from("bar")])
    );

    network.propagator_subtract(a, b, c);

    network.run();

    let actual = network.read_cell(c).unwrap();
    let expected = Supported::new(Interval::new(-2., 0.), &[String::from("foo"), String::from("bar")]);

    assert_eq!(expected, actual);
}

#[test]
fn test_network_supported_interval_multiply() {
    let mut network : Network<Supported<Interval>> = Network::new();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();

    network.write_cell(
        a, 
        Supported::new(Interval::new(0., 1.), &[String::from("foo")])
    );

    network.write_cell(
        b, 
        Supported::new(Interval::new(1., 2.), &[String::from("bar")])
    );

    network.propagator_multiply(a, b, c);

    network.run();

    let actual = network.read_cell(c).unwrap();
    let expected = Supported::new(Interval::new(0., 2.), &[String::from("foo"), String::from("bar")]);

    assert_eq!(expected, actual);
}

/*
#[test]
fn test_network_supported_interval_constraint_add() {
    let mut network : Network<Supported<Interval>> = Network::new();

    let a = network.make_cell();
    let b = network.make_cell();
    let c = network.make_cell();

    network.write_cell(
        b, 
        Supported::new(Interval::new(1., 2.), &[String::from("foo")])
    );

    network.write_cell(
        c, 
        Supported::new(Interval::new(0., 2.), &[String::from("bar")])
    );

    network.constraint_add(a, b, c);

    network.run();

    let actual = network.read_cell(c).unwrap();
    let expected = Supported::new(Interval::new(0., 1.), &[String::from("foo"), String::from("bar")]);

    assert_eq!(expected, actual);
}
*/
//#[test]
//fn test_network_supported_interval_similar_triangles() {
    //let mut network : Network<Supported<Interval>> = Network::new();

    //let a = network.make_cell();
    //let b = network.make_cell();
    //let c = network.make_cell();
    //let d = network.make_cell();

    //network.write_cell(
        //a, 
        //Supported::new(Interval::new(44, 1), &[String::from("foo")])
    //);

    //network.write_cell(
        //b, 
        //Supported::new(Interval::new(1, 2), &[String::from("bar")])
    //);

    //network.write_cell(
        //c, 
        //Supported::new(Interval::new(0, 2), &[String::from("baz")])
    //);

    //network.constraint_similar_triangles(a, b, c, d);

    //network.run();

    //let actual = network.read_cell(d).unwrap();
    //let expected = Supported::new(Interval::new(0, 2), &[String::from("foo"), String::from("bar")]);

    //assert_eq!(expected, actual);
//}

#[test]
fn test_network_supported_interval_barometer() {
    let mut network : Network<Supported<Interval>> = Network::new();

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
        building_shadow, 
        Supported::new(Interval::new(54.9, 55.1), &[String::from("shadows")])
    );

    network.write_cell(
        barometer_height,
        Supported::new(Interval::new(0.3, 0.32), &[String::from("shadows")])
    );

    network.write_cell(
        barometer_shadow, 
        Supported::new(Interval::new(0.36, 0.37), &[String::from("shadows")])
    );

    //network.run();

    //let expected = Some();
    //let actual = network.read_cell(building_height);
    //println!("=== {:?} === ", actual.unwrap());
    
    //assert_eq!(expected, actual);
}


