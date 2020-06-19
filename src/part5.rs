#![allow(unused_variables, unused_imports, dead_code)]

use std::{f32, thread};

use crossbeam::crossbeam_channel;
use crossbeam_channel::unbounded;

use crate::tsp::{
    calculate_distance, calculate_distance_optimized, find_shortest_route, generate_routes,
    get_cities, City,
};

/*
Helper functions:
                        City: A struct that holds the position of a city
                  get_cities: Returns a vector of cities with the specified length (9 is used for the tests)
             generate_routes: Takes in a vector of cities and generates all possible permutations of this vector (every route)
          calculate_distance: Calculates the length of a given route using euclidean distance
calculate_distance_optimized: Same as calculate distance, but will return f32::MAX if the calculation is already longer than the shortest known route
         find_shortest_route: Returns the shortest distance out of a vector of distances
*/

const NUM_CITIES: usize = 9;

// Example of a sequential solution of TSP using iterators
fn tsp_sequential() -> f32 {
    let routes = generate_routes(get_cities(NUM_CITIES));

    let distances: Vec<f32> = routes
        .into_iter()
        .map(|route| calculate_distance(&route))
        .collect();

    let best = find_shortest_route(distances);

    best
}

// TASK 1: Parallelize TSP with threads and channels
// crossbeam channel documentation: https://docs.rs/crossbeam/0.7.3/crossbeam/channel/index.html
fn tsp_parallel() -> f32 {
    unimplemented!();
}

// TASK 2: Optimize TSP with branch-and-bound.
// Hint! use calculate_distance_optimized and keep track of the shortest route
fn tsp_parallel_optimized() -> f32 {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_tsp_sequential() {
        assert_eq!(tsp_sequential(), 10.262417);
    }

    #[test]
    fn test_tsp_parallel() {
        assert_eq!(tsp_parallel(), 10.262417);
    }

    #[test]
    fn test_tsp_parallel_optimized() {
        assert_eq!(tsp_parallel_optimized(), 10.262417);
    }
}
