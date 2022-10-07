// Declare a Constant
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    // You cannot change a constant
    // READY_MOUNT = 3;

    // Declare variables
    let (missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    
    // missiles = missiles - ready;
    println!("{} missiles left", missiles - ready);
    
    // Declare an unused variable (need to add _ in front of the variable name)
    let _test_var: i32 = 9;
}
