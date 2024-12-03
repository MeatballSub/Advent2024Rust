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

fn ordered_asc(report: &Vec<u32>) -> bool {
    report.iter().zip(report.iter().skip(1)).all(|(a, b)| a < b)
}

fn ordered_desc(report: &Vec<u32>) -> bool {
    report.iter().zip(report.iter().skip(1)).all(|(a, b)| a > b)
}

fn ordered(report: &Vec<u32>) -> bool {
    ordered_asc(report) || ordered_desc(report)
}

fn max_diff(report: &Vec<u32>) -> bool {
    report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(a, b)| a.abs_diff(*b))
        .max()
        .unwrap()
        < 4
}

fn is_safe(report: &Vec<u32>) -> bool {
    ordered(report) && max_diff(report)
}

fn exclude(report: &Vec<u32>, index: usize) -> Vec<u32> {
    let mut rval = report.clone();
    rval.remove(index);
    rval
}

#[aoc(day2, part1)]
pub fn part1(file_content: &str) -> usize {
    let reports = parse(file_content);
    reports.iter().filter(|r| is_safe(*r)).count()
}

#[aoc(day2, part2)]
pub fn part2(file_content: &str) -> usize {
    let reports = parse(file_content);
    reports
        .iter()
        .filter(|r| is_safe(r) || (0..r.len()).any(|i| is_safe(&exclude(r, i))))
        .count()
}
