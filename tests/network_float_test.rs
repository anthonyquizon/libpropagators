//use propagators::network::{ Network };

//#[test]
//fn test_network_f64_add() {
    //let mut network : Network<f64> = Network::new();

    //let a = network.make_cell();
    //let b = network.make_cell();
    //let c = network.make_cell();

    //network.write_cell(a, 1.);
    //network.write_cell(b, 2.);

    //network.propagator_add(a, b, c);

    //network.run();

    //let expected = 3.;
    //let actual = network.read_cell(c).unwrap();
    
    //assert_eq!(expected, actual);
//}

//#[test]
//fn test_network_f64_multiply() {
    //let mut network : Network<f64> = Network::new();

    //let a = network.make_cell();
    //let b = network.make_cell();
    //let c = network.make_cell();

    //network.write_cell(a, 1.5);
    //network.write_cell(b, 2.);

    //network.propagator_multiply(a, b, c);

    //network.run();

    //let expected = 3.;
    //let actual = network.read_cell(c).unwrap();
    
    //assert_eq!(expected, actual);
//}


//#[test]
//fn test_network_f64_constraint_add() {
    //let mut network:Network<f64> = Network::new();

    //let a = network.make_cell();
    //let b = network.make_cell();
    //let c = network.make_cell();

    //network.write_cell(b, 2.);
    //network.write_cell(c, 3.);

    //network.constraint_add(a, b, c);

    //network.run();

    //let expected = 1.;
    //let actual = network.read_cell(a).unwrap();
    
    //assert_eq!(expected, actual);
//}

//#[test]
//fn test_network_f64_constraint_product_a() {
    //let mut network:Network<f64> = Network::new();

    //let a = network.make_cell();
    //let b = network.make_cell();
    //let c = network.make_cell();

    //network.write_cell(a, 1.5);
    //network.write_cell(b, 2.);

    //network.constraint_product(a, b, c);

    //network.run();

    //let expected = 3.;
    //let actual = network.read_cell(c).unwrap();
    
    //assert_eq!(expected, actual);
//}

//#[test]
//fn test_network_f64_constraint_product_b() {
    //let mut network:Network<f64> = Network::new();

    //let a = network.make_cell();
    //let b = network.make_cell();
    //let c = network.make_cell();

    //network.write_cell(a, 1.5);
    //network.write_cell(c, 3.);

    //network.constraint_product(a, b, c);

    //network.run();

    //let expected = 2.;
    //let actual = network.read_cell(b).unwrap();
    
    //assert_eq!(expected, actual);
//}

//#[test]
//fn test_network_f64_constraint_product_c() {
    //let mut network:Network<f64> = Network::new();

    //let a = network.make_cell();
    //let b = network.make_cell();
    //let c = network.make_cell();

    //network.write_cell(b, 2.);
    //network.write_cell(c, 3.);

    //network.constraint_product(a, b, c);

    //network.run();

    //let expected = 1.5;
    //let actual = network.read_cell(a).unwrap();
    
    //assert_eq!(expected, actual);
//}


//#[test]
//fn test_network_f64_constraint_product_triangle() {
    //let mut network:Network<f64> = Network::new();

    //let ratio = network.make_cell();

    //let a = network.make_cell();
    //let b = network.make_cell();
    //let c = network.make_cell();
    //let d = network.make_cell();

    //network.write_cell(a, 2.);
    //network.write_cell(b, 4.);

    //network.write_cell(c, 3.);
    //network.write_cell(d, 6.);

    //network.constraint_product(a, ratio, b);
    //network.constraint_product(c, ratio, d);

    //network.run();

    //let expected = 2.;
    //let actual = network.read_cell(ratio).unwrap();
    
    //assert_eq!(expected, actual);
//}

//#[test]
//fn test_network_f64_constraint_product_triangle_2() {
    //let mut network:Network<f64> = Network::new();

    //let ratio = network.make_cell();

    //let a = network.make_cell();
    //let b = network.make_cell();
    //let c = network.make_cell();
    //let d = network.make_cell();

    //network.write_cell(ratio, 2.);

    //network.write_cell(a, 2.);
    //network.write_cell(b, 4.);
    //network.write_cell(c, 3.);

    //network.constraint_product(a, ratio, b);
    //network.constraint_product(c, ratio, d);

    //network.run();

    //let expected = 6.;
    //let actual = network.read_cell(d).unwrap();
    
    //assert_eq!(expected, actual);
//}


//#[test]
//fn test_network_f64_constraint_similar_triangles() {
    //let mut network:Network<f64> = Network::new();

    //let a = network.make_cell();
    //let b = network.make_cell();
    //let c = network.make_cell();
    //let d = network.make_cell();

    //network.write_cell(a, 2.);
    //network.write_cell(b, 4.);
    //network.write_cell(c, 3.);

    //network.constraint_similar_triangles(a, b, c, d);

    //network.run();

    //let expected = 6.;
    //let actual = network.read_cell(d).unwrap();
    
    //assert_eq!(expected, actual);
//}

//////TODO test contradictions
