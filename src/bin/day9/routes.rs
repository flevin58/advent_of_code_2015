use std::{fmt::Display, ops::Index, slice::SliceIndex};

#[derive(Clone, Copy, Debug)]
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

pub struct LocationPairList(Vec<Box<LocationPair>>);

impl Display for LocationPairList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().for_each(|lp| {
            writeln!(f, "{lp:?}").unwrap();
        });
        Ok(())
    }
}

impl LocationPairList {
    pub fn from_input_data(input_data: &str) -> Self {
        let mut loc_pairs = Vec::<Box<LocationPair>>::new();
        for line in input_data.lines() {
            let lpline = line.trim();
            if !lpline.is_empty() {
                let locp = Box::new(LocationPair::from_string(lpline));
                loc_pairs.push(locp);
            }
        }
        Self(loc_pairs)
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

    pub fn from_location_pair_list(location_pairs: Vec<Box<LocationPair>>) -> Self {
        let mut route = Route::new();
        for lp in location_pairs {
            route.add_location_pair(lp);
        }
        Self(route.0.into())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn at(&self, index: usize) -> String {
        self.0[index].clone()
    }

    pub fn add_string(&mut self, location: &str) {
        let sloc = String::from(location);
        if !self.0.contains(&sloc) {
            self.0.push(sloc);
        }
    }

    pub fn add_location_pair(&mut self, location: LocationPair) {
        self.add_string(&location.a);
        self.add_string(&location.b);
    }
}
#[derive(Clone)]
pub struct Routes {
    loc_pairs: Vec<LocationPair>,
    start_route: Box<Route>,
}

impl Routes {
    pub fn new() -> Self {
        Self {
            loc_pairs: Vec::<LocationPair>::new(),
            start_route: Box::new(Route::new()),
        }
    }

    pub fn from_input_data(input_data: &str) -> Self {
        let mut routes = Routes::new();
        for line in input_data.lines() {
            let lpline = line.trim();
            if !lpline.is_empty() {
                let locp = LocationPair::from_string(lpline);
                routes.loc_pairs.push(locp);
            }
        }
        routes.start_route = Route::from_location_pair_list(routes.loc_pairs);
        routes.clone()
    }
}

// #region TEST

#[cfg(test)]
mod tests {

    use crate::routes::{LocationPair, Route};

    #[test]
    fn test_distance_from_string() {
        let location = LocationPair::from_string("London to Dublin = 464");
        assert_eq!(location.a, String::from("London"));
        assert_eq!(location.b, String::from("Dublin"));
        assert_eq!(location.d, 464);
    }
    // #endregion

    #[test]
    fn test_route_add_string() {
        let mut route = Route::new();
        route.add_string("London");
        route.add_string("Paris");
        assert_eq!(
            route.0.iter().collect::<Vec<&String>>(),
            vec!["London", "Paris"]
        );
    }

    #[test]
    fn test_route_add_location() {
        let mut route = Route::new();
        route.add_location_pair(LocationPair::from_string("London to Dublin = 464"));
        assert_eq!(
            route.0.iter().collect::<Vec<&String>>(),
            vec!["London", "Dublin"]
        );
    }
}
