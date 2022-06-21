use std::collections::HashMap;

fn number_of_loops_dict() -> HashMap<usize, i8> {
    return HashMap::from([
        (0, 1),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 1),
        (5, 0),
        (6, 1),
        (7, 0),
        (8, 2),
        (9, 1),
    ]);
}

pub fn number_of_loops_in_number(number: &str) -> i8 {
    let mut sum_of_loops: i8 = 0;
    number
        .split("")
        .into_iter()
        .collect::<Vec<&str>>()
        .iter()
        .filter(|digit_or_whitespace| digit_or_whitespace.len() > 0)
        .for_each(|digit| {
            sum_of_loops = sum_of_loops
                + number_of_loops_dict()
                    .get(&(*digit).to_string().parse::<usize>().unwrap())
                    .unwrap();
        });
    sum_of_loops
}

#[cfg(test)]
mod test;
