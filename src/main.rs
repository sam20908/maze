use colored::*;
use rand::distributions::{Bernoulli, Distribution};
use rand::{thread_rng, Rng};

use std::collections::{HashSet, VecDeque};

const WIDTH: usize = 50usize; // at least 3
const HEIGHT: usize = 50usize; // at least 3

#[derive(Clone)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

const DIRECTION_ARRAY: [Direction; 4] = [
    Direction::LEFT,
    Direction::RIGHT,
    Direction::UP,
    Direction::DOWN,
];

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

fn gen_maze() -> (Vec<bool>, usize, usize) {
    let start = thread_rng().gen_range(1, HEIGHT - 1) * WIDTH;
    let end = thread_rng().gen_range(1, WIDTH - 1) * WIDTH + WIDTH - 1;
    let mut maze = vec![false; WIDTH * HEIGHT];
    let mut vis = vec![false; WIDTH * HEIGHT];
    let mut dir = vec![Direction::DOWN; WIDTH * HEIGHT];

    vis[start] = true;
    vis[start + 1] = true;
    vis[end] = true;
    vis[end - 1] = true;

    let remain = WIDTH * HEIGHT - 2 * WIDTH - (2 * HEIGHT - 4);
    while remain > 0 {
        let mut start = WIDTH + 1;
        let r = start / WIDTH;
        let c = start % WIDTH;
        while vis[start] || r == 0 || r == HEIGHT - 1 || c == 0 || c == WIDTH - 1 {
            start = thread_rng().gen_range(0, WIDTH * HEIGHT);
            r = start / WIDTH;
            c = start % WIDTH;
        }
        let mut cur = start;

        while !vis[cur] {
            let mut d = dir[thread_rng().gen_range(0, 4)];
            let mut to = match d {
                Direction::LEFT => cur - 1,
                Direction::RIGHT => cur + 1,
                Direction::UP => cur - WIDTH,
                Direction::DOWN => cur + WIDTH,
            };
            let r = to / WIDTH;
            let c = to % WIDTH;
            while r == 0 || r == HEIGHT - 1 || c == 0 || c == WIDTH - 1 {
                d = dir[thread_rng().gen_range(0, 4)];
                to = match d {
                    Direction::LEFT => cur - 1,
                    Direction::RIGHT => cur + 1,
                    Direction::UP => cur - WIDTH,
                    Direction::DOWN => cur + WIDTH,
                };
                r = to / WIDTH;
                c = to % WIDTH;
            }
            dir[start] = d;
            cur = to;
        }
    }

    maze
}

fn solve(maze: &Vec<bool>, start: usize, end: usize) -> Vec<usize> {}

fn main() {
    loop {}
}
