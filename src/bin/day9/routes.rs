use itertools::Itertools;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct LocationPair {
    a: String,
    b: String,
    d: usize,
}

impl LocationPair {
    pub fn from_string(s: &str) -> Self {
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();
        Self {
            a: String::from(parts[0]),
            b: String::from(parts[2]),
            d: String::from(parts[4])
                .parse()
                .unwrap_or_else(|e| panic!("{}", e)),
        }
    }
}

#[derive(Clone)]
pub struct Route(Vec<String>);

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let last = self.0.last().unwrap();
        self.0.iter().for_each(|loc| {
            write!(f, "{}", loc).unwrap();
            if loc != last {
                write!(f, " -> ").unwrap();
            }
        });
        Ok(())
    }
}

impl Route {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_string_vector(v: Vec<&String>) -> Self {
        let mut route = Route::new();
        for loc in v {
            route.add_string(loc);
        }
        route
    }

    pub fn add_location_pair(&mut self, location: &LocationPair) {
        self.add_string(&location.a);
        self.add_string(&location.b);
    }

    // Helper function
    fn add_string(&mut self, location: &str) {
        let sloc = String::from(location);
        if !self.0.contains(&sloc) {
            self.0.push(sloc);
        }
    }
}

#[derive(Clone)]
pub struct Routes {
    loc_pairs: Vec<LocationPair>,
    routes: Vec<Route>,
}

impl Display for Routes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Routes {{")?;

        writeln!(f, "  loc_pairs: {{")?;
        for lp in &self.loc_pairs {
            writeln!(f, "    {:?}", *lp)?;
        }
        writeln!(f, "  }}")?;

        writeln!(f, "  routes: {{")?;
        for (i, route) in self.routes.iter().enumerate() {
            writeln!(f, "    {}: {}", *route, self.calc_route_distance(i))?;
        }
        writeln!(f, "  }}")?;

        writeln!(f, "}}")?;
        Ok(())
    }
}

impl Routes {
    pub fn new() -> Self {
        Self {
            loc_pairs: Vec::<LocationPair>::new(),
            routes: Vec::<Route>::new(),
        }
    }

    pub fn from_input_data(input_data: &str) -> Self {
        let mut routes = Routes::new();
        let mut start_route = Route::new();
        for line in input_data.lines() {
            let lpline = line.trim();
            if !lpline.is_empty() {
                let locp = LocationPair::from_string(lpline);
                start_route.add_location_pair(&locp);
                routes.loc_pairs.push(locp);
            }
        }

        let perms = start_route.0.iter().permutations(start_route.0.len());
        for item in perms {
            routes.routes.push(Route::from_string_vector(item));
        }
        routes
    }

    fn distance_between_locations(&self, a: &String, b: &String) -> usize {
        for lp in &self.loc_pairs {
            if (lp.a == *a && lp.b == *b) || (lp.a == *b && lp.b == *a) {
                return lp.d;
            }
        }
        0
    }

    pub fn calc_route_distance(&self, i: usize) -> usize {
        let route = self.routes.get(i).unwrap();
        let route_slice = route.0.as_slice();
        let mut tot_dist = 0_usize;
        for i in 0..route_slice.len() - 1 {
            tot_dist += self.distance_between_locations(&route_slice[i], &route_slice[i + 1]);
        }
        tot_dist
    }

    pub fn get_shortest_route_distance(&self) -> usize {
        let mut shortest_distance = 1_000_000_usize;
        for i in 0..self.routes.len() {
            let dist = self.calc_route_distance(i);
            shortest_distance = std::cmp::min(shortest_distance, dist);
        }
        shortest_distance
    }

    pub fn get_longest_route_distance(&self) -> usize {
        let mut longest_distance = 0_usize;
        for i in 0..self.routes.len() {
            let dist = self.calc_route_distance(i);
            longest_distance = std::cmp::max(longest_distance, dist);
        }
        longest_distance
    }
}

// #region TEST

#[cfg(test)]
mod tests {

    use crate::routes::{LocationPair, Route, Routes};

    #[test]
    fn distance_from_string() {
        let location = LocationPair::from_string("London to Dublin = 464");
        assert_eq!(location.a, String::from("London"));
        assert_eq!(location.b, String::from("Dublin"));
        assert_eq!(location.d, 464);
    }
    // #endregion

    #[test]
    fn route_add_string() {
        let mut route = Route::new();
        route.add_string("London");
        route.add_string("Paris");
        assert_eq!(
            route.0.iter().collect::<Vec<&String>>(),
            vec!["London", "Paris"]
        );
    }

    #[test]
    fn route_add_location() {
        let mut route = Route::new();
        route.add_location_pair(&LocationPair::from_string("London to Dublin = 464"));
        assert_eq!(
            route.0.iter().collect::<Vec<&String>>(),
            vec!["London", "Dublin"]
        );
    }

    #[test]
    fn get_shortest_route_distance() {
        let input_data = r#"
            London to Dublin = 464
            London to Belfast = 518
            Dublin to Belfast = 141
        "#;

        let routes = Routes::from_input_data(input_data);
        let shortest = routes.get_shortest_route_distance();
        assert_eq!(shortest, 605);
    }

    #[test]
    fn get_longest_route_distance() {
        let input_data = r#"
            London to Dublin = 464
            London to Belfast = 518
            Dublin to Belfast = 141
        "#;

        let routes = Routes::from_input_data(input_data);
        let longest = routes.get_longest_route_distance();
        assert_eq!(longest, 982);
    }
}
