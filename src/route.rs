const ROUTE_LENGTH: usize = 8;

const CITY_DISTANCES: [[u8; ROUTE_LENGTH]; ROUTE_LENGTH] = [
    [0, 1, 3, 2, 2, 3, 5, 2],
    [1, 0, 1, 4, 4, 4, 4, 6],
    [3, 1, 0, 1, 3, 2, 1, 3],
    [2, 4, 1, 0, 1, 4, 3, 5],
    [2, 4, 3, 1, 0, 1, 4, 4],
    [3, 4, 2, 4, 1, 0, 1, 2],
    [5, 4, 1, 3, 4, 1, 0, 1],
    [2, 6, 3, 5, 4, 2, 1, 0],
];

pub type Route = [u8; ROUTE_LENGTH];

pub trait RouteActions {
    fn print(&self);
    fn calc_cost(&self) -> u16;
    fn is_shorter_than(&self, other: &Route) -> bool;
}

impl RouteActions for Route {
    fn print(&self) {
        for route_item in self {
            print!("{}", route_item);
        }
        println!();
    }
    fn calc_cost(&self) -> u16 {
        let mut cost: u16 = 0;
        let n = self.len();
        for i in 1..n {
            cost += CITY_DISTANCES[(self[i-1]-1) as usize][(self[i]-1) as usize] as u16;
        }
        cost + CITY_DISTANCES[(self[0]-1) as usize][(self[n-1]-1) as usize] as u16
    }
    fn is_shorter_than(&self, other: &Route) -> bool {
        self.calc_cost() < other.calc_cost()
    }
}