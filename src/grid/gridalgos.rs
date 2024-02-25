use crate::grid::GridSquare;
use std::cmp;

// Horizontal and vertical steps cost 1 unit each.
// Diagonal steps cost 1 and 2 units alternately.
pub fn distance(a: GridSquare, b: GridSquare) -> i32 {
    let dy = (b.y - a.y).abs();
    let dx = (b.x - a.x).abs();

    let mx = cmp::max(dy, dx);
    let mi = cmp::min(dy, dx);

    mx + mi - (mi + 1) / 2
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
    fn test_emanation() {
        assert_eq!(emanation(GridSquare { y: 2, x: 4 }, 0).len(), 1);
        assert_eq!(emanation(GridSquare { y: 5, x: 1 }, 1).len(), 9);
        assert_eq!(emanation(GridSquare { y: 4, x: 3 }, 2).len(), 21);
    }
}
