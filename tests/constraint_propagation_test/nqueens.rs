/*
use libpropagators::kernel::scheduler::{Scheduler};
use libpropagators::propagators::{};

const LIMIT = 10;

#[test]
fn test_nqueens() {
    let scheduler = Scheduler::new();

    let x = Variable::new("x");
    let y = Variable::new("y");
    let z = Variable::new("z");

    scheduler.add_many([
        Equal::new(x, y),
        EqualToValue::new(x, z),
    ]);

    let result = scheduler.run(LIMIT);

    match result {
        Success(values) => {
            println!("Success!");

            for (i, value) in values.iter().enumerate() {
                println!("{}: {}", i, value);
            }
        },
        Failure => {
            println!("Failure!");
        }
    }
}
*/
