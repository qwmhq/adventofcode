use std::{fs::File, io::Read};

fn main() {
    let input_path = "inputs/day9";
    let diskmap = parse_diskmap(&input_path);

    println!("part1: filesystem checksum: {:?}", solve_part1(&diskmap));
    println!("part2: filesystem checksum: {:?}", solve_part2(&diskmap));
    println!(
        "part2 using alt: filesystem checksum: {:?}",
        solve_part2_alt(&diskmap)
    );
}

fn parse_diskmap(path: &str) -> Vec<u8> {
    let mut file = File::open(path).expect(&format!("couldn't open file '{path}'"));
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("problem reading file");

    buf.chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect()
}

fn solve_part1(diskmap: &Vec<u8>) -> u64 {
    let mut blocks: Vec<Option<u64>> = diskmap
        .iter()
        .enumerate()
        .map(|(idx, val)| {
            if idx % 2 == 0 {
                vec![Some(idx as u64 / 2); *val as usize]
            } else {
                vec![None; *val as usize]
            }
        })
        .flatten()
        .collect();
    let mut i = 0;
    while i < blocks.len() {
        if blocks[i].is_none() {
            loop {
                if let Some(v) = blocks.pop().unwrap() {
                    blocks[i] = Some(v);
                    break;
                } else if i >= blocks.len() {
                    break;
                }
            }
        }
        i += 1;
    }
    blocks
        .iter()
        .enumerate()
        .map(|(idx, val)| idx as u64 * val.unwrap())
        .sum()
}

fn solve_part2(diskmap: &Vec<u8>) -> u64 {
    let mut blocks: Vec<Vec<Option<u64>>> = diskmap
        .iter()
        .enumerate()
        .filter_map(|(idx, val)| {
            let block_size = *val as usize;
            if block_size < 1 {
                return None;
            }
            let block = if idx % 2 == 0 {
                Some((idx / 2) as u64)
            } else {
                None
            };
            Some(vec![block; block_size])
        })
        .collect();

    let mut i = blocks.len() - 1;
    loop {
        if blocks[i][0].is_some() {
            for j in 0..i {
                if blocks[j][0].is_none() {
                    let free_space = blocks[j].len();
                    let file_size = blocks[i].len();
                    if free_space >= file_size {
                        blocks.swap(j, i);
                        if free_space > file_size {
                            // resize the free block to match the file block
                            blocks[i].resize(file_size, None);
                            // insert free block with the remaining free space after the file block
                            blocks.insert(j + 1, vec![None; free_space - file_size]);
                            // update iteration index to reflect the added block
                            i += 1;
                        }
                        break;
                    }
                }
            }
        }
        i -= 1;
        if i == 0 {
            break;
        }
    }
    blocks
        .iter()
        .flatten()
        .enumerate()
        .filter_map(|(idx, val)| val.map(|v| v * idx as u64))
        .sum()
}

fn solve_part2_alt(diskmap: &Vec<u8>) -> u64 {
    let mut blocks: Vec<Option<u64>> = diskmap
        .iter()
        .enumerate()
        .filter_map(|(idx, val)| {
            let block_size = *val as usize;
            if block_size < 1 {
                return None;
            }
            let block = if idx % 2 == 0 {
                Some((idx / 2) as u64)
            } else {
                None
            };
            Some(vec![block; block_size])
        })
        .flatten()
        .collect();

    let mut i = blocks.len() - 1;
    let mut last_handled_fileid = None;
    loop {
        if let Some(id) = blocks[i] {
            let mut start_idx = i;
            while blocks[start_idx - 1].is_some_and(|x| x == id) {
                start_idx -= 1;
                if start_idx == 0 {
                    break;
                }
            }
            if last_handled_fileid.is_some_and(|fileid| id >= fileid) {
                i = start_idx - 1;
                continue;
            }
            let file_size = i - start_idx + 1;
            for j in 0..i {
                if blocks[j].is_none() {
                    let mut end_idx = j;
                    while blocks[end_idx + 1].is_none() {
                        end_idx += 1;
                        if end_idx == blocks.len() - 1 {
                            break;
                        }
                    }
                    let free_space = end_idx - j + 1;
                    if free_space >= file_size {
                        let (p1, p2) = (i, j);
                        for k in 0..file_size {
                            blocks.swap(p2 + k, p1 - k);
                        }
                        break;
                    }
                }
            }
            i = start_idx;
            last_handled_fileid = Some(id);
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    blocks
        .iter()
        .enumerate()
        .filter_map(|(idx, val)| val.map(|v| v * idx as u64))
        .sum()
}

fn blocks_as_string(blocks: &Vec<Option<u64>>) -> String {
    blocks
        .iter()
        .map(|x| x.map(|z| z.to_string()).unwrap_or(".".to_string()))
        .collect::<String>()
}
