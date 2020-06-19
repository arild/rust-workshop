#![allow(unused_variables)]

use std::f32;

#[derive(Debug, Clone)]
pub struct City {
    x: f32,
    y: f32,
}

pub fn calculate_distance(cities: &Vec<City>) -> f32 {
    let mut sum = 0.0;
    for i in 0..cities.len() {
        let c1 = &cities[i];
        let c2 = &cities[(i + 1) % cities.len()];
        let distance = euclidean_distance(c1, c2);
        sum += distance
    }
    sum
}

pub fn calculate_distance_optimized(cities: &Vec<City>, current_best: f32) -> f32 {
    let mut sum = 0.0;
    for i in 0..cities.len() {
        let c1 = &cities[i];
        let c2 = &cities[(i + 1) % cities.len()];
        let distance = euclidean_distance(c1, c2);
        sum += distance;
        if sum >= current_best {
            sum = std::f32::MAX;
            break;
        }
    }
    sum
}

fn euclidean_distance(c1: &City, c2: &City) -> f32 {
    ((c2.x - c1.x).abs() + (c2.y - c1.y).abs()).sqrt()
}

pub fn get_cities(num_cities: usize) -> Vec<City> {
    vec![
        City { x: 0.0, y: 0.0 },
        City { x: 0.5, y: 1.0 },
        City { x: 0.3, y: 1.5 },
        City { x: 1.0, y: 2.5 },
        City { x: 1.3, y: 1.2 },
        City { x: 1.5, y: 1.4 },
        City { x: 3.0, y: 1.5 },
        City { x: 2.3, y: 1.2 },
        City { x: 3.1, y: 0.2 },
        City { x: 1.0, y: 2.0 },
        City { x: 1.5, y: 1.9 },
        City { x: 4.3, y: 3.1 },
        City { x: 4.0, y: 1.1 },
        City { x: 3.3, y: 2.2 },
        City { x: 1.4, y: 5.4 },
        City { x: 5.0, y: 3.5 },
        City { x: 3.3, y: 1.2 },
        City { x: 0.1, y: 0.9 },
    ][..num_cities]
        .to_owned()
}

pub fn generate_routes(mut cities: Vec<City>) -> Vec<Vec<City>> {
    let mut permutation_vec: Vec<Vec<City>> = vec![cities.clone()];
    let n = cities.len();
    let mut i: usize = 0;
    let mut c = vec![0; n];
    while i < n {
        if c[i] < i {
            cities.swap(if i % 2 == 1 { c[i] } else { 0 }, i);
            c[i] += 1;
            i = 0;
            permutation_vec.push(cities.clone());
        } else {
            c[i] = 0;
            i += 1;
        }
    }
    permutation_vec
}

pub fn find_shortest_route(distances: Vec<f32>) -> f32 {
    distances.iter().fold(f32::MAX, |a, &b| a.min(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_generate_permutations() {
        assert_eq!(generate_routes(get_cities(0)).len(), 1);
        assert_eq!(generate_routes(get_cities(1)).len(), 1);
        assert_eq!(generate_routes(get_cities(2)).len(), 2);
        assert_eq!(generate_routes(get_cities(3)).len(), 6);
        assert_eq!(generate_routes(get_cities(4)).len(), 24);
        assert_eq!(generate_routes(get_cities(5)).len(), 120);
        assert_eq!(generate_routes(get_cities(6)).len(), 720);
        assert_eq!(generate_routes(get_cities(7)).len(), 5040);
        assert_eq!(generate_routes(get_cities(8)).len(), 40320);
        //        assert_eq!(generate_permutations(get_cities(9)).len(), 362880);
        //        assert_eq!(generate_permutations(get_cities(10)).len(), 3628800);
    }

    #[test]
    #[ignore]
    fn test_calculate_distance() {
        assert_eq!(
            calculate_distance(&vec![City { x: 0.0, y: 0.0 }, City { x: 2.0, y: 2.0 }]),
            4.0
        );
        assert_eq!(
            calculate_distance(&vec![
                City { x: 0.0, y: 0.0 },
                City { x: 0.0, y: 1.0 },
                City { x: 1.0, y: 1.0 },
                City { x: 1.0, y: 0.0 }
            ]),
            4.0
        );
    }
}
