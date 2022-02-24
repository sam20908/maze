use rand::{thread_rng, Rng};
use std::collections::*;

const WIDTH: usize = 5usize; // at least 1
const HEIGHT: usize = 5usize; // at least 1

#[derive(Clone, Copy)]
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

fn print_maze(maze: &Vec<Vec<usize>>, path: &VecDeque<usize>) {
    for r in 0..HEIGHT - 1 {
        let i = r * WIDTH;
        print!("{} ", if path.contains(&i) { '*' } else { '+' });

        let mut i2 = i;
        for _ in 1..WIDTH {
            print!(
                "{} {} ",
                if maze[i2 + 1].contains(&i2) { ' ' } else { '|' },
                if path.contains(&(i2 + 1)) { '*' } else { '+' }
            );
            i2 += 1;
        }
        println!();
        i2 = i;
        for _ in 0..WIDTH {
            print!(
                "{}   ",
                if maze[i2].contains(&(i2 + WIDTH)) {
                    ' '
                } else {
                    '-'
                }
            );
            i2 += 1;
        }
        println!();
    }

    let mut i = (HEIGHT - 1) * WIDTH;
    print!("{} ", if path.contains(&i) { '*' } else { '+' });
    for _ in 1..WIDTH {
        print!(
            "{} {} ",
            if maze[i + 1].contains(&i) { ' ' } else { '|' },
            if path.contains(&(i + 1)) { '*' } else { '+' }
        );
        i += 1;
    }
    println!();
}

// fn print_maze(maze: &Vec<bool>, path: &Vec<usize>) {
//     for i in 0..HEIGHT {
//         for j in 0..WIDTH {
//             let i = i * HEIGHT + j;
//             if path.contains(&i) {
//                 print!("{} ", "+".yellow());
//             } else {
//                 print!("{} ", if maze[i] { '.' } else { '#' });
//             }
//         }
//         println!("");
//     }
// }

// fn print_maze(maze: &Vec<bool>) {
//     for i in 0..HEIGHT {
//         for j in 0..WIDTH {
//             let i = i * HEIGHT + j;
//             print!("{} ", if maze[i] { '.' } else { '#' });
//         }
//         println!("");
//     }
// }

fn gen_maze() -> (Vec<Vec<usize>>, usize, usize) {
    let start = thread_rng().gen_range(0..HEIGHT) * WIDTH;
    let end = thread_rng().gen_range(0..HEIGHT) * WIDTH + WIDTH - 1;
    let mut maze = vec![vec![]; WIDTH * HEIGHT];
    let mut vis = vec![false; WIDTH * HEIGHT];
    let mut dir = vec![Direction::DOWN; WIDTH * HEIGHT];
    vis[end] = true;

    let mut remain = WIDTH * HEIGHT - 1;
    while remain > 0 {
        let mut from = thread_rng().gen_range(0..WIDTH * HEIGHT);
        while vis[from] {
            from = thread_rng().gen_range(0..WIDTH * HEIGHT);
        }

        let mut cur = from;
        while !vis[cur] {
            let d = DIRECTION_ARRAY[thread_rng().gen_range(0..4)];
            let r = cur / WIDTH;
            let c = cur % WIDTH;
            let to = match d {
                Direction::LEFT => {
                    if c == 0 {
                        cur
                    } else {
                        cur - 1
                    }
                }
                Direction::RIGHT => {
                    if c == WIDTH - 1 {
                        cur
                    } else {
                        cur + 1
                    }
                }
                Direction::UP => {
                    if r == 0 {
                        cur
                    } else {
                        cur - WIDTH
                    }
                }
                Direction::DOWN => {
                    if r == HEIGHT - 1 {
                        cur
                    } else {
                        cur + WIDTH
                    }
                }
            };
            dir[cur] = d;
            cur = to;
        }

        let mut path_count = 0;
        let to = cur;
        cur = from;
        while cur != to {
            vis[cur] = true;
            let next = match dir[cur] {
                Direction::LEFT => cur - 1,
                Direction::RIGHT => cur + 1,
                Direction::UP => cur - WIDTH,
                Direction::DOWN => cur + WIDTH,
            };
            maze[cur].push(next);
            maze[next].push(cur);
            path_count += 1;
            cur = next;
        }
        remain -= path_count;
    }

    (maze, start, end)
}

fn solve(maze: &Vec<Vec<usize>>, start: usize, end: usize) -> VecDeque<usize> {
    let mut vis = vec![false; WIDTH * HEIGHT];
    let mut pre = vec![usize::MAX; WIDTH * HEIGHT];
    let mut q = VecDeque::new();
    q.push_back(start);

    while !q.is_empty() {
        let i = q.pop_front().unwrap();
        if i == end {
            break;
        }
        vis[i] = true;

        for n in &maze[i] {
            if vis[*n] {
                continue;
            }

            pre[*n] = i;
            q.push_back(*n);
        }
    }

    let mut cur = end;
    let mut path = VecDeque::new();
    while pre[cur] != usize::MAX {
        path.push_front(cur);
        cur = pre[cur];
    }
    path.push_front(start);
    path
}

fn main() {
    let (maze, start, end) = gen_maze();
    let path = solve(&maze, start, end);
    print_maze(&maze, &path);
    // print_maze(&maze);
}
