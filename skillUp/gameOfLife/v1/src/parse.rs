const CELL_ALIVE: char = 'o';
const CELL_DEAD: char = '.';

pub fn get_map_filepath() -> Result<String, String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(String::from("You must pass a filepath to your map"));
    }
    Ok(args[1].to_owned())
}

fn vec_to_2d_vec(src: String) -> Result<Vec<Vec<bool>>, String> {
    let mut map: Vec<Vec<bool>> = Vec::new();
    // let mut line_start: usize = 0;

    for line in src.split("\n") {
        let mut map_line: Vec<bool> = Vec::new();
        for c in line.chars() {
            let status: bool = match c {
                CELL_ALIVE => true,
                CELL_DEAD => false,
                _ => return Err(String::from("Invalid map character")),
            };
            map_line.push(status);
        }
        map.push(map_line);
    }
    Ok(map)
}

fn pad_map(map: &mut Vec<Vec<bool>>, max_x: u32, max_y: u32) {
    for line in map.into_iter() {
        if line.len() < max_x as usize {
            let padding = max_x as usize - line.len();
            line.append(&mut vec![false; padding])
        }
    }
    if map.len() < max_y as usize {
        let padding = max_y as usize - map.len();
        map.append(&mut vec![vec![false; max_x as usize]; padding])
    }
}

pub fn get_map_from_filepath(
    path: String,
    max_x: u32,
    max_y: u32,
) -> Result<Vec<Vec<bool>>, String> {
    if !std::path::Path::new(&path).exists() {
        return Err(String::from("Not Found!"));
    }
    let content = std::fs::read_to_string(&path)
        .expect(format!("Could not read the file {}", &path).as_str());
    let mut vec_2d = match vec_to_2d_vec(content) {
        Ok(x) => x,
        Err(e) => return Err(e),
    };

    pad_map(&mut vec_2d, max_x, max_y);

    return Ok(vec_2d);
}
