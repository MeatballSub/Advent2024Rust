use voracious_radix_sort::RadixSort;

fn parse_file_content1(file_content: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left: Vec<u32> = Vec::with_capacity(1000);
    let mut right: Vec<u32> = Vec::with_capacity(1000);
    let mut lists = vec![&mut left, &mut right];
    let mut value: u32 = 0;
    let mut in_value = false;
    let mut was_in_value: bool;
    let mut insert_index: usize = 0;

    for c in file_content.bytes() {
        was_in_value = in_value;
        in_value = c >= b'0' && c <= b'9';
        if in_value {
            value = value * 10 + (c - b'0') as u32;
        } else if was_in_value {
            lists[insert_index].push(value);
            insert_index = 1 - insert_index;
            value = 0;
        }
    }

    if in_value {
        right.push(value);
    }

    (left, right)
}

fn parse_file_content2(file_content: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left: Vec<u32> = Vec::with_capacity(1000);
    let mut right: Vec<u32> = vec![0; 100000];
    let mut left_inserter = |v| left.push(v);
    let mut right_inserter = |v| right[v as usize] += 1;
    let mut inserters: Vec<&mut dyn FnMut(u32)> = vec![&mut left_inserter, &mut right_inserter];
    let mut value: u32 = 0;
    let mut in_value = false;
    let mut was_in_value: bool;
    let mut insert_index: usize = 0;

    for c in file_content.bytes() {
        was_in_value = in_value;
        in_value = c >= b'0' && c <= b'9';
        if in_value {
            value = value * 10 + (c - b'0') as u32;
        } else if was_in_value {
            inserters[insert_index](value);
            insert_index = 1 - insert_index;
            value = 0;
        }
    }

    if in_value {
        right[value as usize] += 1;
    }

    (left, right)
}

#[aoc(day1, part1)]
pub fn part1(file_content: &str) -> u32 {
    let (mut left, mut right) = parse_file_content1(file_content);
    left.voracious_sort();
    right.voracious_sort();
    left.iter().zip(right).map(|(a, b)| a.abs_diff(b)).sum()
}

#[aoc(day1, part2)]
pub fn part2(file_content: &str) -> u32 {
    let (left, right) = parse_file_content2(file_content);
    left.iter().map(|v| v * right[*v as usize]).sum()
}
