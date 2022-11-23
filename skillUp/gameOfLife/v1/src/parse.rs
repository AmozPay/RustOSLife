const CELL_ALIVE: char = 'o';
const CELL_DEAD: char = '.';

pub fn get_map_filepath(program_args: &Vec<String>) -> Result<&String, String> {
    if program_args.len() < 2 {
        return Err(String::from("You must pass a filepath to your map"));
    }
    Ok(&program_args[1])
}

fn string_to_map(src: String) -> Result<Vec<Vec<bool>>, String> {
    let mut map: Vec<Vec<bool>> = Vec::new();

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

fn map_fits(map: &Vec<Vec<bool>>, max_x: u32, max_y: u32) -> bool {
    if map.len() > max_y as usize {
        return false;
    }
    for line in map {
        if line.len() > max_x as usize {
            return false;
        }
    }
    return true;
}

pub fn get_map_from_filepath(
    path: &String,
    max_x: u32,
    max_y: u32,
) -> Result<Vec<Vec<bool>>, String> {
    if !std::path::Path::new(path).exists() {
        return Err(String::from("Not Found!"));
    }
    let content =
        std::fs::read_to_string(path).expect(format!("Could not read the file {}", path).as_str());
    let mut map = match string_to_map(content) {
        Ok(x) => x,
        Err(e) => return Err(e),
    };

    if !map_fits(&map, max_x, max_y) {
        return Err(format!("Map is too big. max size: {} by {}", max_x, max_y));
    }
    pad_map(&mut map, max_x, max_y);

    return Ok(map);
}

// UNIT TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_map_correct() {
        let string_map = String::from("...ooo\nooo...");
        let map = string_to_map(string_map).unwrap();
        assert_eq!(map[0].len(), 6);
        assert_eq!(map[1].len(), 6);
        assert_eq!(map[0][0], false);
        assert_eq!(map[0][1], false);
        assert_eq!(map[0][2], false);
        assert_eq!(map[0][3], true);
        assert_eq!(map[0][4], true);
        assert_eq!(map[0][5], true);
        assert_eq!(map[1][0], true);
        assert_eq!(map[1][1], true);
        assert_eq!(map[1][2], true);
        assert_eq!(map[1][3], false);
        assert_eq!(map[1][4], false);
        assert_eq!(map[1][5], false);
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_string_to_map_should_return_error() {
        let string_map = String::from("...ooA\nkljhg..");
        let should_return_error = match string_to_map(string_map) {
            Ok(_) => false,
            Err(_) => true,
        };

        assert_eq!(should_return_error, true);
    }

    #[test]
    fn test_string_to_map_error_should_match() {
        let string_map = String::from("...ooA\nkljhg..");
        let should_return_error = match string_to_map(string_map) {
            Ok(_) => panic!("test failed"),
            Err(e) => e,
        };

        assert_eq!(should_return_error, String::from("Invalid map character"));
    }

    #[test]
    fn test_get_map_filepath_should_return_path() {
        let args = vec![String::from("./game_of_life"), String::from("map.txt")];
        let path = get_map_filepath(&args).unwrap();
        assert_eq!(*path, String::from("map.txt"));
    }

    #[test]
    fn test_get_map_filepath_should_return_error() {
        let args = vec![String::from("./game_of_life")];
        let should_return_error = match get_map_filepath(&args) {
            Ok(_) => false,
            Err(_) => true,
        };

        assert_eq!(should_return_error, true)
    }

    #[test]
    fn test_get_map_filepath_error_should_match() {
        let args = vec![String::from("./game_of_life")];
        let error = match get_map_filepath(&args) {
            Ok(_) => panic!("Test failed"),
            Err(e) => e,
        };

        assert_eq!(error, String::from("You must pass a filepath to your map"))
    }

    #[test]
    fn test_pad_map_len_should_match() {
        let map_str = String::from("...ooo\n...ooo");
        let mut map = string_to_map(map_str).unwrap();
        pad_map(&mut map, 10, 10);
        assert_eq!(map.len(), 10);
        assert_eq!(map[0].len(), 10);
    }

    #[test]
    fn test_map_fits_should_fail() {
        let map_str = String::from("...ooo\n...ooo");
        let map = string_to_map(map_str).unwrap();
        let fits = map_fits(&map, 1, 1);
        assert_eq!(fits, false);
    }
}
