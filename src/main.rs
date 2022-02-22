use colored::*;
use rand::distributions::{Bernoulli, Distribution};
use rand::{thread_rng, Rng};

use std::collections::VecDeque;

const WIDTH: usize = 25usize; // at least 3
const HEIGHT: usize = 25usize; // at least 3

type MazeArray = [[char; WIDTH]; HEIGHT];
type Position = (usize, usize);
type Path = Vec<Position>;

fn print_maze(maze: &MazeArray) {
    for row in maze.iter() {
        for col in row.iter() {
            if *col == '+' {
                print!("{} ", "+".yellow());
            } else {
                print!("{} ", *col);
            }
        }
        println!("");
    }
}

fn gen_maze() -> (MazeArray, Position, Position) {
    let mut maze = [['#'; WIDTH]; HEIGHT]; // # indicates wall, . indicates clear path

    let start = thread_rng().gen_range(1, HEIGHT - 1);
    let end = thread_rng().gen_range(1, HEIGHT - 1);
    maze[start][0] = '.';
    maze[end][WIDTH - 1] = '.';

    let dist = Bernoulli::new(0.5).expect("Bernoulli distribution failed!");
    for i in 1..HEIGHT - 1 {
        for j in 1..WIDTH - 1 {
            if dist.sample(&mut rand::thread_rng()) {
                maze[i][j] = '.';
            }
        }
    }

    (maze, (start, 0), (end, WIDTH - 1))
}

fn solve(maze: &MazeArray, start: Position, end: Position) -> Option<Path> {
    let mut solvable = false;

    let mut pred = [[(usize::MAX, usize::MAX); WIDTH]; HEIGHT];
    let mut vis = [[false; WIDTH]; HEIGHT];

    let mut dist = 0;
    let mut q = VecDeque::new();
    q.push_back(start);

    while !q.is_empty() && !solvable {
        let mut s = q.len();
        while s > 0 {
            s -= 1;

            let pos = q.pop_front().unwrap();
            let (r, c) = pos;
            if pos == end {
                solvable = true;
                break;
            }
            vis[r][c] = true;

            if maze[r][c + 1] == '.' && !vis[r][c + 1] {
                pred[r][c + 1] = pos;
                q.push_back((r, c + 1));
            }
            if maze[r - 1][c] == '.' && !vis[r - 1][c] {
                pred[r - 1][c] = pos;
                q.push_back((r - 1, c));
            }
            if maze[r + 1][c] == '.' && !vis[r + 1][c] {
                pred[r + 1][c] = pos;
                q.push_back((r + 1, c));
            }
            if pos != start && maze[r][c - 1] == '.' && !vis[r][c - 1] {
                pred[r][c - 1] = pos;
                q.push_back((r, c - 1));
            }
        }

        dist += 1;
    }

    if solvable {
        let mut path = Vec::with_capacity(dist);
        let mut pos = end;
        while pred[pos.0][pos.1] != (usize::MAX, usize::MAX) {
            path.push(pos);
            pos = pred[pos.0][pos.1];
        }
        path.push(start);

        Some(path)
    } else {
        None
    }
}

fn main() {
    loop {
        let (mut maze, start, end) = gen_maze();
        if let Some(path) = solve(&maze, start, end) {
            for (x, y) in path {
                maze[x][y] = '+';
            }
            print_maze(&maze);

            break;
        }
    }
}
