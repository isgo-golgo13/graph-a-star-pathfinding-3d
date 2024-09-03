use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Node {
    position: (usize, usize),
    cost: f64,
    priority: f64,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.partial_cmp(&self.priority).unwrap()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Implement Eq manually
impl Eq for Node {}

pub fn a_star(
    matrix: &Vec<Vec<u8>>,
    start: (usize, usize),
    goal: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from = vec![vec![None; matrix[0].len()]; matrix.len()];
    let mut g_score = vec![vec![f64::INFINITY; matrix[0].len()]; matrix.len()];

    open_set.push(Node {
        position: start,
        cost: 0.0,
        priority: heuristic(start, goal),
    });
    g_score[start.0][start.1] = 0.0;

    while let Some(current) = open_set.pop() {
        if current.position == goal {
            return Some(reconstruct_path(came_from, current.position));
        }

        for neighbor in neighbors(current.position, matrix) {
            let tentative_g_score = g_score[current.position.0][current.position.1] + 1.0;
            if tentative_g_score < g_score[neighbor.0][neighbor.1] {
                came_from[neighbor.0][neighbor.1] = Some(current.position);
                g_score[neighbor.0][neighbor.1] = tentative_g_score;
                open_set.push(Node {
                    position: neighbor,
                    cost: tentative_g_score,
                    priority: tentative_g_score + heuristic(neighbor, goal),
                });
            }
        }
    }

    None
}

fn heuristic(a: (usize, usize), b: (usize, usize)) -> f64 {
    (((a.0 as isize - b.0 as isize).pow(2) + (a.1 as isize - b.1 as isize).pow(2)) as f64).sqrt()
}

fn neighbors(position: (usize, usize), matrix: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let (x, y) = position;
    let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for (dx, dy) in directions.iter() {
        let new_x = x.wrapping_add(*dx as usize);
        let new_y = y.wrapping_add(*dy as usize);

        if new_x < matrix.len() && new_y < matrix[0].len() && matrix[new_x][new_y] == 0 {
            result.push((new_x, new_y));
        }
    }

    result
}

fn reconstruct_path(
    mut came_from: Vec<Vec<Option<(usize, usize)>>>,
    mut current: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut total_path = vec![current];
    while let Some(next) = came_from[current.0][current.1] {
        current = next;
        total_path.push(current);
    }
    total_path.reverse();
    total_path
}
