const CITY_DISTANCES: [[u8; 8]; 8] = [
    [0, 1, 3, 2, 2, 3, 5, 2],
    [1, 0, 1, 4, 4, 4, 4, 6],
    [3, 1, 0, 1, 3, 2, 1, 3],
    [2, 4, 1, 0, 1, 4, 3, 5],
    [2, 4, 3, 1, 0, 1, 4, 4],
    [3, 4, 2, 4, 1, 0, 1, 2],
    [5, 4, 1, 3, 4, 1, 0, 1],
    [2, 6, 3, 5, 4, 2, 1, 0],
];

fn main() {

    let mut route: [u8; 8] = [3, 6, 4, 1, 7, 8, 2, 5];
    print!("Initial route with value {}: ", calc_cost(&route));
    print_route(&route);
    let n = route.len();
    for i in 0..n*2 {
        let j = i % n;
        let mut comparison: [u8; 8];
        if j == 0 {
            comparison = route;
            switch_elems(&mut comparison, 0, 1);
        } else if j == n-1 {
            comparison = route;
            switch_elems(&mut comparison, j, n-1);
        } else {
            let mut first_comparison = route;
            switch_elems(&mut first_comparison, j, j -1);
            let mut second_comparison = route;
            switch_elems(&mut second_comparison, j, j +1);
            if calc_cost(&first_comparison) < calc_cost(&second_comparison) {
                comparison = first_comparison;
            } else {
                comparison = second_comparison;
            }
        }
        if calc_cost(&comparison) < calc_cost(&route) {
            route = comparison;
            print!("New route in iteration {}: ", i);
            print_route(&route);
        }
    }
    print!("Final route with value {}: ", calc_cost(&route));
    print_route(&route)
}

// expects the passed slice to have the passed indices
fn switch_elems(slice: &mut [u8], idx1: usize, idx2: usize) {
    let tmp = slice[idx1];
    slice[idx1] = slice[idx2];
    slice[idx2] = tmp;
}

// expects CITY_DISTANCES to have the length of the passed slice
// in all dimensions and the passed slice to have a length of at least 2
fn calc_cost(slice: &[u8]) -> u16 {
    let mut cost: u16 = 0;
    let n = slice.len();
    for i in 1..n {
        cost += CITY_DISTANCES[(slice[i-1]-1) as usize][(slice[i]-1) as usize] as u16;
    }
    cost + CITY_DISTANCES[(slice[0]-1) as usize][(slice[n-1]-1) as usize] as u16
}

fn print_route(route: &[u8]) {
    for route_item in route {
        print!("{}", route_item);
    }
    print!("\n");
}