use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    // let a: [[i32; 8]; 8] = [
    //     [0, 0, 0, 1, 0, 0, 0, 0],
    //     [1, 1, 0, 1, 0, 1, 1, 0],
    //     [0, 0, 0, 1, 0, 0, 0, 0],
    //     [0, 1, 1, 1, 0, 1, 1, 0],
    //     [0, 0, 0, 0, 0, 1, 0, 0],
    //     [0, 1, 1, 1, 1, 1, 0, 1],
    //     [0, 1, 0, 0, 0, 1, 0, 1],
    //     [0, 0, 0, 1, 0, 1, 0, 0],
    //
    // ];
    // print!("{}[2J", 27 as char); // Clear terminal window

    let a = vec![vec![0; 21]; 31];
    let width = a.len();
    let height = a[0].len();
    let a = maze_maker(a, rand::random(), [0, width], [0, height]);
    draw_maze(&a);
    let a = maze_solver(&a, [0, 0], [width - 1, height - 1]);
    if a.is_some() {
        draw_maze(&a.unwrap());
    } else {
        println!("No path through the maze!");
    }
}

fn draw_maze(maze: &Vec<Vec<i32>>) {
    for y in 0..maze[0].len() {
        for x in 0..maze.len() {
            if maze[x][y] == 1 {
                print!("\u{2588}\u{2588}"); // Print two blocks. Unicodes at w3schools.com/charsets/ref_utf_box.asp
            } else if maze[x][y] == 2 {
                print!("\u{2592}\u{2592}");
            } else {
                print!("\u{2591}\u{2591}"); // Print two lightly shaded blocks
            }
        }
        println!("");
    }
    println!("");
}

fn maze_maker(mut process_maze: Vec<Vec<i32>>, axis_vert: bool,
              x_range: [usize; 2], y_range: [usize; 2]) -> Vec<Vec<i32>> {
    if x_range[1] - x_range[0] <= 1 || y_range[1] - y_range[0] <= 1 {
        return process_maze;
    }

    if axis_vert {
        let rand_x = random_odd_or_even(x_range, 1);
        let rand_y = random_odd_or_even(y_range, 0);

        for y_num in y_range[0]..rand_y {
            process_maze[rand_x as usize][y_num as usize] = 1;
        }
        for y_num in rand_y + 1..y_range[1] {
            process_maze[rand_x as usize][y_num as usize] = 1;
        }

        process_maze = maze_maker(process_maze, false, [x_range[0], rand_x], y_range);
        process_maze = maze_maker(process_maze, false, [rand_x + 1, x_range[1]], y_range);
    } else {
        let rand_x = random_odd_or_even(x_range, 0);
        let rand_y = random_odd_or_even(y_range, 1);

        for x_num in x_range[0]..rand_x {
            process_maze[x_num as usize][rand_y as usize] = 1;
        }
        for x_num in rand_x + 1..x_range[1] {
            process_maze[x_num as usize][rand_y as usize] = 1;
        }

        process_maze = maze_maker(process_maze, true, x_range, [y_range[0], rand_y]);
        process_maze = maze_maker(process_maze, true, x_range, [rand_y + 1, y_range[1]]);
    }
    process_maze
}

fn random_odd_or_even(range: [usize; 2], odd_or_even: usize) -> usize {
    let mut right_numbers: Vec<usize> = Vec::new();
    for num in range[0]..range[1] {
        if num % 2 == odd_or_even {
            right_numbers.push(num);
        }
    }
    let mut rng = thread_rng();
    let result = *right_numbers.choose(&mut rng).unwrap();
    result
}

fn maze_solver(maze: &Vec<Vec<i32>>, focus_space:[usize; 2], goal:[usize; 2]) -> Option<Vec<Vec<i32>>> {

    if maze[focus_space[0] as usize][focus_space[1] as usize] > 0 {
        return None;
    }

    let mut path = maze.clone();
    path[focus_space[0] as usize][focus_space[1] as usize] = 2;

    if focus_space[0] == goal[0] && focus_space[1] == goal[1] {
        return Some(path);
    }

    if focus_space[0] < maze.len() - 1 {
        let temp_result = maze_solver(&path, [focus_space[0] + 1, focus_space[1]], goal);
        if temp_result.is_some() {
            return temp_result;
        }
    }

    if focus_space[1] < maze[0].len() - 1 {
        let temp_result = maze_solver(&path, [focus_space[0], focus_space[1] + 1], goal);
        if temp_result.is_some() {
            return temp_result;
        }
    }

    if focus_space[0] > 0 {
        let temp_result = maze_solver(&path, [focus_space[0] - 1, focus_space[1]], goal);
        if temp_result.is_some() {
            return temp_result;
        }
    }

    if focus_space[1] > 0 {
        let temp_result = maze_solver(&path, [focus_space[0], focus_space[1] - 1], goal);
        if temp_result.is_some() {
            return temp_result;
        }
    }

    return None;
}
