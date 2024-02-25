use crate::grid::{Grid, GridSquare};
use std::cmp;
use std::collections::{BinaryHeap, HashMap, HashSet};

// Horizontal and vertical steps cost 1 unit each.
// Diagonal steps cost 1 and 2 units alternately.
pub fn distance(a: GridSquare, b: GridSquare) -> i32 {
    let dy = (b.y - a.y).abs();
    let dx = (b.x - a.x).abs();

    let mx = cmp::max(dy, dx);
    let mi = cmp::min(dy, dx);

    mx + mi - (mi + 1) / 2
}

pub fn get_neighbors(square: GridSquare) -> Vec<GridSquare> {
    let mut result: Vec<GridSquare> = Vec::new();
    for dy in -1..2 {
        for dx in -1..2 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let neighbor = GridSquare {
                y: square.y + dy,
                x: square.x + dx,
            };

            result.push(neighbor);
        }
    }

    result
}

fn is_diagonal_neighbor(a: GridSquare, b: GridSquare) -> bool {
    let dy = (b.y - a.y).abs();
    let dx = (b.x - a.x).abs();

    assert!(dy <= 1 && dx <= 1, "the squares are not neighbors");

    (dy != 0) && (dx != 0)
}

pub fn emanation(center: GridSquare, radius: i32) -> Vec<GridSquare> {
    let mut result: Vec<GridSquare> = Vec::new();

    for cy in (center.y - radius)..(center.y + radius + 1) {
        for cx in (center.x - radius)..(center.x + radius + 1) {
            let sq = GridSquare { y: cy, x: cx };
            if distance(sq, center) <= radius {
                result.push(sq);
            }
        }
    }

    result.sort_by_key(|k| distance(*k, center));

    result
}

pub struct ShortestPaths {
    pub source: HashSet<GridSquare>,
    pub distance: Vec<HashMap<GridSquare, i32>>,
    pub predecessor: Vec<HashMap<GridSquare, Option<GridSquare>>>,
}

pub fn find_all_shortest_paths(source: Vec<GridSquare>, max_distance: i32, grid: &Grid) -> ShortestPaths {
    // [0]: even number of diagonals taken so far, next dist 1
    // [1]: odd number of diagonals taken so far, next dist 2
    let mut distance: Vec<HashMap<GridSquare, i32>> = vec![HashMap::new(), HashMap::new()];
    let mut predecessor: Vec<HashMap<GridSquare, Option<GridSquare>>> = vec![HashMap::new(), HashMap::new()];
    let mut visited: Vec<HashSet<GridSquare>> = vec![HashSet::new(), HashSet::new()];

    // (distance, (square, reached with even:0 / odd:1 number of diagonals))
    let mut pq: BinaryHeap<(i32, (GridSquare, usize))> = BinaryHeap::new();
    let inf: i32 = 1000000005;

    for sq in source.clone() {
        distance[0].insert(sq, 0);
        predecessor[0].insert(sq, None);
        pq.push((0, (sq, 0)));
    }

    while !pq.is_empty() {
        let (_, (cur_square, diagonals)) = pq.pop().unwrap();
        assert!(diagonals == 0 || diagonals == 1);

        if visited[diagonals].contains(&cur_square) {
            continue;
        }
        visited[diagonals].insert(cur_square);

        let neighbors = get_neighbors(cur_square);
        for nb in neighbors {
            if !grid.free_square(nb) {
                continue;
            }

            if is_diagonal_neighbor(nb, cur_square) {
                // If we have taken an even number of diagonals, the next one costs 1,
                // otherwise it costs 2.
                let dist_delta = 1 + diagonals as i32;
                let new_dist = distance[diagonals].get(&cur_square).unwrap() + dist_delta;

                if new_dist > max_distance {
                    continue;
                }

                let diagonals_nb = (diagonals + 1) % 2;

                if new_dist < *distance[diagonals_nb].get(&nb).unwrap_or(&inf) {
                    distance[diagonals_nb].insert(nb, new_dist);
                    predecessor[diagonals_nb].insert(nb, Some(cur_square));
                    pq.push((-new_dist, (nb, diagonals_nb)));
                }
            } else {
                let dist_delta = 1;
                let new_dist = distance[diagonals].get(&cur_square).unwrap() + dist_delta;

                if new_dist > max_distance {
                    continue;
                }

                if new_dist < *distance[diagonals].get(&nb).unwrap_or(&inf) {
                    distance[diagonals].insert(nb, new_dist);
                    predecessor[diagonals].insert(nb, Some(cur_square));
                    pq.push((-new_dist, (nb, diagonals)));
                }
            }
        }
    }

    let mut source_set: HashSet<GridSquare> = HashSet::new();
    for sq in source {
        source_set.insert(sq);
    }

    ShortestPaths {
        source: source_set,
        distance,
        predecessor,
    }
}

pub fn extract_shortest_path(paths: &ShortestPaths, target: GridSquare) -> Vec<GridSquare> {
    let mut result: Vec<GridSquare> = Vec::new();
    let inf: i32 = 1000000005;

    let mut cur_square = target;
    while !paths.source.contains(&cur_square) {
        result.push(cur_square);

        let dist_even = paths.distance[0].get(&cur_square).unwrap_or(&inf);
        let dist_odd = paths.distance[1].get(&cur_square).unwrap_or(&inf);

        if dist_even <= dist_odd {
            cur_square = paths.predecessor[0].get(&cur_square).unwrap().unwrap();
        } else {
            cur_square = paths.predecessor[1].get(&cur_square).unwrap().unwrap();
        }
    }

    result.push(cur_square);
    result.reverse();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_distance_2way(a: GridSquare, b: GridSquare, expected: i32) {
        assert_eq!(distance(a, b), expected);
        assert_eq!(distance(b, a), expected);
    }

    #[test]
    fn test_distance() {
        test_distance_2way(GridSquare { y: 0, x: 0 }, GridSquare { y: 0, x: 0 }, 0);

        test_distance_2way(GridSquare { y: 0, x: 0 }, GridSquare { y: 0, x: 1 }, 1);
        test_distance_2way(GridSquare { y: 0, x: 0 }, GridSquare { y: 1, x: 0 }, 1);

        test_distance_2way(GridSquare { y: 0, x: 0 }, GridSquare { y: 1, x: 1 }, 1);
        test_distance_2way(GridSquare { y: 0, x: 0 }, GridSquare { y: 2, x: 2 }, 3);
        test_distance_2way(GridSquare { y: 0, x: 0 }, GridSquare { y: 3, x: 3 }, 4);
        test_distance_2way(GridSquare { y: 0, x: 0 }, GridSquare { y: 4, x: 4 }, 6);
        test_distance_2way(GridSquare { y: 0, x: 0 }, GridSquare { y: 5, x: 5 }, 7);

        test_distance_2way(GridSquare { y: 2, x: 3 }, GridSquare { y: 3, x: 6 }, 3);
        test_distance_2way(GridSquare { y: 2, x: 3 }, GridSquare { y: 4, x: 6 }, 4);
    }

    #[test]
    fn test_get_neighbors() {
        let nb = get_neighbors(GridSquare { y: 3, x: 7 });
        assert_eq!(nb.len(), 8);
    }

    #[test]
    fn test_emanation() {
        assert_eq!(emanation(GridSquare { y: 2, x: 4 }, 0).len(), 1);
        assert_eq!(emanation(GridSquare { y: 5, x: 1 }, 1).len(), 9);
        assert_eq!(emanation(GridSquare { y: 4, x: 3 }, 2).len(), 21);
    }
}
