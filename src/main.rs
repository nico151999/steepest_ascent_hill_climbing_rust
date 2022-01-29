mod route;

use crate::route::{Route, RouteActions};

fn main() {
    let mut route: Route = [3, 6, 4, 1, 7, 8, 2, 5];
    print!("Initial route with cost {}: ", route.calc_cost());
    route.print();
    let n = route.len();
    for i in 0..n {
        let j = i % n;
        let mut comparison: Route;
        if j == 0 {
            comparison = route;
            comparison.swap(0, 1);
        } else if j == n-1 {
            comparison = route;
            comparison.swap(j, n-1);
        } else {
            let mut first_comparison = route;
            first_comparison.swap(j, j -1);
            let mut second_comparison = route;
            second_comparison.swap(j, j +1);
            if first_comparison.is_shorter_than(&second_comparison) {
                comparison = first_comparison;
            } else {
                comparison = second_comparison;
            }
        }
        if comparison.is_shorter_than(&route) {
            route = comparison;
            print!("New route in iteration {} with cost {}: ", i, route.calc_cost());
            route.print();
        }
    }
    print!("Final route with cost {}: ", route.calc_cost());
    route.print();
}