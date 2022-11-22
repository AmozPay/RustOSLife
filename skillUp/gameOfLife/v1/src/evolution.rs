use std::collections::VecDeque;

pub fn print_map(map: &Vec<Vec<bool>>) {
    for line in map {
        for cell in line {
            print!("{}", if *cell { 'o' } else { '.' });
        }
        print!("\n");
    }
}

fn get_nb_surrounding_alive(cell_x: usize, cell_y: usize, map: &Vec<Vec<bool>>) -> usize {
    let mut nb: usize = 0;
    let mut x_directions: VecDeque<i32> = VecDeque::from([-1, 0, 1]);
    let mut y_directions: VecDeque<i32> = VecDeque::from([-1, 0, 1]);
    if cell_x == 0 {
        x_directions.pop_front();
    }
    if cell_y == 0 {
        y_directions.pop_front();
    }
    if cell_y == map.len() - 1 {
        y_directions.pop_back();
    }
    if cell_x == map[cell_y].len() - 1 {
        x_directions.pop_back();
    }
    for y_dir in &y_directions {
        for x_dir in &x_directions {
            let center_cell = map[cell_y][cell_x];
            if *y_dir == 0 && *x_dir == 0 {
                continue;
            }
            let new_x = (cell_x as i32 + *x_dir) as usize;
            let new_y = (cell_y as i32 + *y_dir) as usize;
            let cell = map[new_y][new_x];
            if cell {
                nb += 1;
            }
        }
    }
    nb
}

fn compute_state(x: usize, y: usize, map: &Vec<Vec<bool>>) -> bool {
    let surrounding_cells_alive = get_nb_surrounding_alive(x, y, &map);
    match surrounding_cells_alive {
        0 | 1 => return false,
        2 => {
            if map[y][x] {
                return true;
            } else {
                return false;
            }
        }
        3 => return true,
        _ => return false,
    }
}

pub fn evolve(map: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_map: Vec<Vec<bool>> = map.clone();
    for (y, line) in map.iter().enumerate() {
        for (x, _cell) in line.iter().enumerate() {
            new_map[y][x] = compute_state(x, y, &map)
        }
    }
    return new_map;
}
