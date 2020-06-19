#![allow(unused_variables)]

use std::{f32, thread};

use crossbeam::crossbeam_channel;
use crossbeam_channel::{Receiver, Sender};
use crossbeam_channel::select;
use crossbeam_channel::unbounded;

use assert_approx_eq::assert_approx_eq;

use crate::tsp::{
    calculate_distance, calculate_distance_optimized, City, find_shortest_route,
    generate_routes, get_cities,
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

enum Work {
    Calculate(Vec<City>),
}

enum Result {
    Distance(f32),
}

enum Subscription {
    Subscribe(Sender<ShortestRoute>),
    PublishShortestRoute(f32),
}

enum ShortestRoute {
    NewShortestRoute(f32)
}

const NUM_CITIES: usize = 9;


// Example of a sequential solution of TSP using iterators
fn tsp_sequential() -> f32 {
    let routes = generate_routes(get_cities(NUM_CITIES));

    let distances: Vec<f32> = routes
        .into_iter()
        .map(|route| calculate_distance(&route))
        .collect();

    find_shortest_route(distances)
}


// TASK 1: Parallelize TSP with threads and channels
// crossbeam channel documentation: https://docs.rs/crossbeam/0.7.3/crossbeam/channel/index.html
fn tsp_parallel() -> f32 {
    let routes = generate_routes(get_cities(NUM_CITIES));
    let number_of_routes = routes.len();
    let num_workers = 6;

    let (work_send, work_recv) = unbounded();
    let (result_send, result_recv) = unbounded();

    for i in 0..num_workers {
        let work_recv = work_recv.clone();
        let result_send = result_send.clone();

        thread::spawn(move || loop {
            match work_recv.recv() {
                Ok(Work::Calculate(cities)) => {
                    result_send.send(calculate_distance(&cities)).unwrap();
                }
                Err(_) => break
            }
        });
    }

    for route in routes {
        work_send.send(Work::Calculate(route)).unwrap();
    }

    let mut distances = Vec::with_capacity(number_of_routes);
    for i in 0..number_of_routes {
        let route_distance = result_recv.recv().unwrap();
        distances.push(route_distance);
    }

    drop(work_recv);

    return find_shortest_route(distances);
}

// TASK 2: Optimize TSP with branch-and-bound.
// Hint! use calculate_distance_optimized and keep track of the shortest route
pub fn tsp_parallel_optimized() -> f32 {
    let routes = generate_routes(get_cities(NUM_CITIES));
    let number_of_routes = routes.len();
    let num_workers = 4;

    let (work_send, work_recv) = unbounded();
    let (result_send, result_recv) = unbounded();
    let (subscription_send, subscription_recv) = unbounded();

    start_broadcast_router(subscription_recv);

    for i in 0..num_workers {
        tsp_worker_optimized(work_recv.clone(), result_send.clone(), subscription_send.clone());
    }

    for route in routes {
        work_send.send(Work::Calculate(route)).unwrap();
    }

    let mut shortest_route = f32::MAX;
    for i in 0..number_of_routes {
        let subscribe_send = subscription_send.clone();

        match result_recv.recv() {
            Ok(Result::Distance(distance)) => {
                if distance < shortest_route {
                    shortest_route = distance;
                    subscribe_send.send(Subscription::PublishShortestRoute(shortest_route)).unwrap();
                }
            }
            Err(_) => {}
        }
    }

    shortest_route
}

fn tsp_worker_optimized(work_recv: Receiver<Work>, result_send: Sender<Result>, subscription_send: Sender<Subscription>) {
    let (shortest_route_send, shortest_route_recv) = unbounded();

    // Each worker subscribes to newly discovered globally shortest routes
    subscription_send.clone().send(Subscription::Subscribe(shortest_route_send)).unwrap();

    let mut current_shortest_route = f32::MAX;
    thread::spawn(move || loop {
        select! {
                recv(work_recv) -> msg => match msg {
                    Ok(Work::Calculate(vec)) => {
                        let distance = calculate_distance_optimized(&vec, current_shortest_route);
                        if distance < current_shortest_route { // branch-and-bound optimization
                            current_shortest_route = distance;
                        }

                        result_send.send(Result::Distance(distance)).unwrap();
                    },
                    Err(_) => break
                },
                recv(shortest_route_recv) -> msg => match msg {
                    Ok(ShortestRoute::NewShortestRoute(distance)) => {
                        current_shortest_route = distance;
                    },
                    Err(_) => break
                }
            }
    });
}

// Allows for broadcasting a message from one sender to arbitrary many receivers.
// Receivers must actively subscribe by providing a separate channel for receiving messages that are broadcasted
fn start_broadcast_router(subscribe_recv: Receiver<Subscription>) {
    let mut subscriptions: Vec<Sender<ShortestRoute>> = vec![];
    thread::spawn(move || loop {
        match subscribe_recv.recv() {
            Ok(Subscription::Subscribe(sender)) => {
                subscriptions.push(sender);
            }
            Ok(Subscription::PublishShortestRoute(distance)) => {
                for subscription in &subscriptions {
                    subscription.send(ShortestRoute::NewShortestRoute(distance)).unwrap();
                }
            }
            Err(_) => break
        }
    });
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tsp_sequential() {
        assert_approx_eq!(tsp_sequential(), 10.262417, 1e-3f32);
    }

    #[test]
    fn test_tsp_parallel() {
        assert_approx_eq!(tsp_parallel(), 10.262417, 1e-3f32);
    }

    #[test]
    fn test_tsp_parallel_optimized() {
        assert_approx_eq!(tsp_parallel_optimized(), 10.262417, 1e-3f32);
    }
}
