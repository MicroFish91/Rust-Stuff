const STARTING_ORDERS: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (mut orders, ready) = (STARTING_ORDERS, READY_AMOUNT);
    println!("{} of my {} orders are ready.", ready, orders);
    // println!("{0} of my {1} orders are ready.", ready, orders);

    orders = orders - ready;
    println!("{} orders remaining.", orders);
}
