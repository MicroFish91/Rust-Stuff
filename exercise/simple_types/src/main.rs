use simple_types::{ding, print_array, print_distance, print_difference, on_off};

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);
    let coords_arr: [f32; 2] = [coords.0, coords.1];
    let series = [1, 1, 2, 3, 5, 8, 13];
    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");

    print_difference(coords.0, coords.1);
    print_array(coords_arr);
    ding(series[6]);
    on_off(mess.2[1].0);
    print_distance(coords);
}

