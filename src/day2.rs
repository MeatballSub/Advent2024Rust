fn parse(file_content: &str) -> Vec<Vec<u32>> {
    let mut reports = vec![vec![]; 1000];
    let mut index = 0;
    let mut in_value = false;
    let mut was_in_value: bool;
    let mut value = 0;

    for c in file_content.bytes() {
        was_in_value = in_value;
        in_value = (b'0'..=b'9').contains(&c);
        if in_value {
            value = value * 10 + (c - b'0') as u32;
        } else if was_in_value {
            reports[index].push(value);
            if c == b'\n' {
                index += 1;
            }
            value = 0;
        }
    }

    reports[index].push(value);

    reports
}

fn is_safe(report: &Vec<u32>) -> bool {
    report
        .iter()
        .zip(report.iter().skip(1))
        .all(|(a, b)| a < b && b - a < 4)
        || report
            .iter()
            .zip(report.iter().skip(1))
            .all(|(a, b)| a > b && a - b < 4)
}

fn exclude(report: &Vec<u32>, index: usize) -> Vec<u32> {
    let mut rval = report.clone();
    rval.remove(index);
    rval
}

#[aoc(day2, part1)]
pub fn part1(file_content: &str) -> usize {
    let mut in_value = false;
    let mut was_in_value: bool;
    let mut value: i32 = 0;
    let mut prev: Option<i32> = None;
    let mut sign: Option<i32> = None;
    let mut count: usize = 0;
    let mut count_it = false;
    let slice = file_content.bytes().collect::<Vec<u8>>();

    let mut i: usize = 0;
    let mut c;
    while i < slice.len() {
        c = slice[i];
        was_in_value = in_value;
        in_value = (b'0'..=b'9').contains(&c);
        if in_value {
            value = value * 10 + (c - b'0') as i32;
        } else if was_in_value {
            if let Some(prev_val) = prev {
                let diff = prev_val - value;
                count_it = (1..=3).contains(&diff.abs());
                if count_it {
                    let new_sign = diff.signum();
                    if let Some(old_sign) = sign {
                        count_it = old_sign == new_sign && (1..=3).contains(&diff.abs());
                        if !count_it {
                            while i + 1 < slice.len() && c != b'\n' {
                                i += 1;
                                c = slice[i];
                            }
                        }
                    } else {
                        sign = Some(new_sign);
                    }
                } else {
                    while i + 1 < slice.len() && c != b'\n' {
                        i += 1;
                        c = slice[i];
                    }
                }
            }
            if c == b'\n' {
                if count_it {
                    count += 1;
                }
                prev = None;
                sign = None;
                in_value = false;
            } else {
                prev = Some(value);
            }
            value = 0;
        }
        i += 1;
    }

    if count_it {
        count += 1
    }

    count
}

#[aoc(day2, part2)]
pub fn part2(file_content: &str) -> usize {
    let reports = parse(file_content);
    reports
        .iter()
        .filter(|r| is_safe(r) || (0..r.len()).any(|i| is_safe(&exclude(r, i))))
        .count()
}
