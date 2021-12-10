fn map_to_score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut stack = Vec::<char>::new();
            stack.reserve(line.len()/2);
            for c in line.chars() {
                match c {
                    '{' | '<' | '(' | '[' => stack.push(c),
                    '}' | '>' | ')' | ']' => match (stack.pop().unwrap(), c) {
                        ('{', '}') | ('<', '>') | ('(', ')') | ('[', ']') => (),
                        (_, close) => return map_to_score(close),
                    },
                    _ => unreachable!(),
                }
            }
            0
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut scores: Vec<usize> = input
        .lines()
        .filter_map(|line| {
            let mut stack = Vec::<char>::new();
            stack.reserve(line.len()/2);
            for c in line.chars() {
                match c {
                    '{' | '<' | '(' | '[' => stack.push(c),
                    '}' | '>' | ')' | ']' => match (stack.pop().unwrap(), c) {
                        ('{', '}') | ('<', '>') | ('(', ')') | ('[', ']') => (),
                        (_, _) => return None,
                    },
                    _ => unreachable!(),
                }
            }
            let mut score = 0;
            for c in stack.iter().rev() {
                score *= 5;
                score += match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                }
            }
            Some(score)
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

pub fn run() {
    println!("day 10:");
    const INPUT: &'static str = include_str!("input.txt");

    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST: &'static str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
    #[test]
    fn part1() {
        assert_eq!(26397, super::part1(TEST))
    }
    #[test]
    fn part2() {
        assert_eq!(288957, super::part2(TEST))
    }
}
