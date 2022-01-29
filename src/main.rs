mod route;

use crate::route::{Route, RouteActions};

fn main() {
    // to use the greedy approach just use the greedy generator here
    let mut route: Route = Route::generate_route();
    print!("Initial route with cost {}: ", route.calc_cost());
    route.print();
    {
        let n = route.len();
        let mut i: usize = 0;
        let mut last_improvement: usize = 0;
        // find improvements until there are no findings in 25 consecutive searches
        while i - last_improvement < 25 {
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
                last_improvement = i;
            }
            i += 1;
        }
    }
    print!("Final route with cost {}: ", route.calc_cost());
    route.print();
}