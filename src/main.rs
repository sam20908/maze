use rand::distributions::{Bernoulli, Distribution};
use rand::{thread_rng, Rng};
use std::collections::VecDeque;

const WIDTH: usize = 50usize; // at least 3
const HEIGHT: usize = 45usize; // at least 3

type MazeArray = [[bool; WIDTH]; HEIGHT];
type Position = (usize, usize);
type Path = VecDeque<Position>;

fn print_maze(maze: &MazeArray) {
    for row in maze.iter() {
        print!("[ ");

        for col in row {
            print!("{} ", *col as i32);
        }

        println!("]");
    }
}

fn gen_maze() -> (MazeArray, Position, Position) {
    let mut maze = [[false; WIDTH]; HEIGHT]; // false indicates wall, true indicates clear path

    let start = thread_rng().gen_range(1, HEIGHT - 1);
    let end = thread_rng().gen_range(1, HEIGHT - 1);
    maze[start][0] = true;
    maze[end][WIDTH - 1] = true;

    let dist = Bernoulli::new(0.5).expect("Bernoulli distribution failed!");
    for i in 1..HEIGHT - 1 {
        for j in 1..WIDTH - 1 {
            maze[i][j] = dist.sample(&mut rand::thread_rng());
        }
    }

    (maze, (start, 0), (end, WIDTH - 1))
}

fn dfs(
    maze: &MazeArray,
    vis: &mut MazeArray,
    pos: Position,
    end: Position,
    path: &mut Path,
) -> bool {
    if pos == end {
        path.push_front(pos);
        return true;
    }
    if !maze[pos.0][pos.1] {
        return false;
    }
    vis[pos.0][pos.1] = true;

    if pos.1 > 0 {
        if !vis[pos.0][pos.1 - 1] && dfs(maze, vis, (pos.0, pos.1 - 1), end, path) {
            path.push_front(pos);
            return true;
        }
    }
    if !vis[pos.0][pos.1 + 1] && dfs(maze, vis, (pos.0, pos.1 + 1), end, path) {
        path.push_front(pos);
        return true;
    }
    if !vis[pos.0 - 1][pos.1] && dfs(maze, vis, (pos.0 - 1, pos.1), end, path) {
        path.push_front(pos);
        return true;
    }
    if !vis[pos.0 + 1][pos.1] && dfs(maze, vis, (pos.0 + 1, pos.1), end, path) {
        path.push_front(pos);
        return true;
    }

    false
}

fn solve(maze: &MazeArray, start: Position, end: Position) -> Option<Path> {
    let mut path = Path::default();
    let mut vis = [[false; WIDTH]; HEIGHT];

    if !dfs(maze, &mut vis, start, end, &mut path) {
        None
    } else {
        Some(path)
    }
}

fn main() {
    let mut maze = [[false; WIDTH]; HEIGHT];
    let mut path: Path = Path::default();
    let mut found = false;
    while !found {
        let (maze2, start, end) = gen_maze();
        let solution = solve(&maze2, start, end);
        if let Some(path2) = solution {
            found = true;
            maze = maze2;
            path = path2;
        }
    }

    print_maze(&maze);
    println!("Path is {:?}", path);
}
