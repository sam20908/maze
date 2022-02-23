use colored::*;
use rand::distributions::{Bernoulli, Distribution};
use rand::{thread_rng, Rng};

use std::collections::VecDeque;

const WIDTH: usize = 50usize; // at least 3
const HEIGHT: usize = 50usize; // at least 3

fn print_maze(maze: &Vec<bool>, path: &Vec<usize>) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let i = i * HEIGHT + j;
            if path.contains(&i) {
                print!("{} ", "+".yellow());
            } else {
                print!("{} ", if maze[i] { '.' } else { '#' });
            }
        }
        println!("");
    }
}

fn gen_maze(maze: &mut Vec<bool>, dist: &Bernoulli) -> (usize, usize) {
    let start = thread_rng().gen_range(1, HEIGHT - 1) * WIDTH;
    let end = thread_rng().gen_range(1, HEIGHT - 1) * WIDTH + WIDTH - 1;
    maze[start] = true;
    maze[end] = true;

    for i in 1..HEIGHT - 1 {
        for j in 1..WIDTH - 1 {
            maze[i * WIDTH + j] = dist.sample(&mut rand::thread_rng());
        }
    }

    (start, end)
}

fn solve(
    maze: &Vec<bool>,
    start: usize,
    end: usize,
    pred: &mut Vec<usize>,
    vis: &mut Vec<bool>,
) -> Option<Vec<usize>> {
    let mut solvable = false;
    let mut dist = 0;

    let mut q = VecDeque::new();
    q.push_back(start);

    while !q.is_empty() && !solvable {
        let mut s = q.len();
        while s > 0 {
            s -= 1;

            let pos = q.pop_front().unwrap();
            if pos == end {
                solvable = true;
                break;
            }
            vis[pos] = true;

            if maze[pos + 1] && !vis[pos + 1] {
                pred[pos + 1] = pos;
                q.push_back(pos + 1);
            }
            if maze[pos - WIDTH] && !vis[pos - WIDTH] {
                pred[pos - WIDTH] = pos;
                q.push_back(pos - WIDTH);
            }
            if maze[pos + WIDTH] && !vis[pos + WIDTH] {
                pred[pos + WIDTH] = pos;
                q.push_back(pos + WIDTH);
            }
            if pos != start && maze[pos - 1] && !vis[pos - 1] {
                pred[pos - 1] = pos;
                q.push_back(pos - 1);
            }
        }

        dist += 1;
    }

    if solvable {
        let mut path = Vec::with_capacity(dist);
        let mut pos = end;
        while pred[pos] != usize::MAX {
            path.push(pos);
            pos = pred[pos];
        }
        path.push(start);

        Some(path)
    } else {
        None
    }
}

fn main() {
    let mut maze = vec![false; WIDTH * HEIGHT]; // false indicates wall, true indicates clear path
    let mut pred = vec![usize::MAX; WIDTH * HEIGHT];
    let mut vis = vec![false; WIDTH * HEIGHT];

    let dist = Bernoulli::new(0.7).expect("Bernoulli distribution failed!");

    loop {
        maze[0..WIDTH].fill(false);
        let i = (HEIGHT - 1) * WIDTH;
        maze[i..i + WIDTH].fill(false);
        for i in 1..HEIGHT - 1 {
            let j = i * WIDTH;
            maze[j] = false;
            maze[j + WIDTH - 1] = false;
        }

        vis.fill(false);

        let (start, end) = gen_maze(&mut maze, &dist);
        if let Some(path) = solve(&maze, start, end, &mut pred, &mut vis) {
            print_maze(&maze, &path);
            break;
        }
    }
}
