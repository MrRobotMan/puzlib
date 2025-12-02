use std::{
    fmt::{Debug, Display},
    fs::read_to_string,
    path::Path,
    str::FromStr,
};

/// Gather a string of text or file name to a string
pub fn contents<T: AsRef<Path> + Display>(path: T) -> String {
    match path.as_ref().exists() {
        false => path.to_string(),
        true => read_to_string(path).expect("Failed to open file {path}"),
    }
}

/// Read the text of a file to a vec of strings
pub fn read_lines<T: AsRef<Path> + Display>(path: T) -> Vec<String> {
    contents(path)
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// Reads records that are line delineated.
/// For example:
/// 1234
/// 4567
///
/// 3423
/// 2543
pub fn read_number_records<T: AsRef<Path> + Display, U: FromStr>(path: T) -> Vec<Vec<U>>
where
    <U as FromStr>::Err: Debug,
{
    contents(path)
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.lines()
                .filter(|s| !s.is_empty())
                .map(|num| num.parse::<U>().expect("Unable to parse number"))
                .collect::<Vec<U>>()
        })
        .collect()
}

/// Return records split by \n\n.
pub fn read_string_records<T: AsRef<Path> + Display>(path: T) -> Vec<String> {
    contents(path)
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// Reads the text of a file to a vector of numbers.
pub fn read_numbers<T: AsRef<Path> + Display, U: FromStr>(path: T) -> Vec<U>
where
    <U as FromStr>::Err: Debug,
{
    read_lines(path)
        .iter()
        .map(|l| l.parse::<U>().expect("Could not parse number {l:?}"))
        .collect()
}

/// Reads the text of a file to a vector of vector of numbers.
pub fn read_number_lists<T: AsRef<Path> + Display, U: FromStr>(path: T, sep: &str) -> Vec<Vec<U>>
where
    <U as FromStr>::Err: Debug,
{
    read_lines(path)
        .iter()
        .map(|l| {
            l.split(sep)
                .map(|l| l.parse::<U>().expect("Could not parse number {l:?}"))
                .collect()
        })
        .collect()
}

/// Reads the file to a list of chars.
pub fn read_line<T: AsRef<Path> + Display>(path: T) -> Vec<char> {
    contents(path).chars().filter(|&chr| chr != '\n').collect()
}

/// Reads a single line file to a list breaking on a separator.
pub fn read_line_sep<T: AsRef<Path> + Display>(path: T, sep: &str) -> Vec<String> {
    contents(path).trim().split(sep).map(|s| s.into()).collect()
}

/// Reads the file to a list of chars.
pub fn read_line_record<T: AsRef<Path> + Display, U: FromStr>(path: T) -> Vec<U>
where
    <U as FromStr>::Err: Debug,
{
    contents(path)
        .trim()
        .split(",")
        .map(|v| v.parse().expect("Could not parse number {v:?}"))
        .collect()
}

/// Reads the file to a grid (vec of vec) of chars
pub fn read_grid<T: AsRef<Path> + Display>(path: T) -> Vec<Vec<char>> {
    contents(path)
        .trim()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

/// Reads the file to a grid (vec of vec) of u8
pub fn read_grid_numbers<T: AsRef<Path> + Display>(path: T) -> Vec<Vec<u8>> {
    contents(path)
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - b'0').collect())
        .collect()
}

/// Reads the contents to an iterator of coordinates / char pairs
pub fn read_grid_to_map<T: AsRef<Path> + Display>(path: T) -> Vec<((usize, usize), char)> {
    contents(path)
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, ch)| ((row, col), ch))
                .collect::<Vec<_>>()
        })
        .collect()
}

/// Reads the file to grids (vec of vec) of char records line delineated
/// ```
/// let input = "..##.
/// .#...
///
/// ..#..
/// ....#";
/// let expected = vec![
///     vec![
///         vec!['.', '.', '#', '#', '.'],
///         vec!['.', '#', '.', '.', '.']
///         ],
///     vec![
///         vec!['.', '.', '#', '.', '.'],
///         vec!['.', '.', '.', '.', '#']
///         ]
///     ];
/// let actual = puzlib::read_grid_records(input);
/// assert_eq!(expected, actual);
/// ```
pub fn read_grid_records<T: AsRef<Path> + Display>(path: T) -> Vec<Vec<Vec<char>>> {
    contents(path)
        .split("\n\n")
        .map(|l| l.lines().map(|r| r.chars().collect()).collect())
        .collect()
}
