use std::fs;

fn main() {
    let map = fs::read("../input.txt").expect("Failed to read file");
    let width = find_width(&map);
    let sum = find_adjacent_numbers(&map, width);
    println!("{}", sum);
}

fn find_width(map: &[u8]) -> isize {
    map.iter().position(|&b| b == b'\n').unwrap() as isize
}

fn find_adjacent_numbers(map: &[u8], width: isize) -> usize {
    let filter = (0..map.len() - 2)
        .filter(|&i| {
            map[i].is_ascii_digit()
                && !map.get(i.wrapping_sub(1)).map_or(false, |&b| b.is_ascii_digit())
        });
    filter
        .map(|i| {
            let d = (i + 1..i + 4).position(|j| !map[j].is_ascii_digit()).unwrap() + 1;
            let num_str = std::str::from_utf8(&map[i..i + d]).unwrap();
            let n = num_str.parse::<usize>().unwrap();
            (i, d as _, n)
        })
        .filter(|&(i, d, _)| {
            (-width - 2..-width + d)
                .chain([-1, d])
                .chain(width..width + d + 2)
                .any(|j| {
                    map.get((i as isize + j) as usize)
                        .map_or(false, |&b| b != b'.' && b.is_ascii_punctuation())
                })
        })
        .map(|(_i, _d, n)| n)
        .sum::<usize>()
}