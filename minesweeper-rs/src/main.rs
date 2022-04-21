fn is_mine(c: char) -> bool {
    c == '*'
}

fn mine_to_int(c: char) -> i32 {
    match c {
        '*' => 1,
        _ => 0,
    }
}

fn mine_to_str(m: i32) -> String {
    match m {
        0 => String::from("."),
        -1 => String::from("*"),
        _ => m.to_string(),
    }
}

#[derive(Debug)]
struct Adjacent {
    i: usize,
    j: usize,
}

fn check_mines_adjacents(rows: usize, cols: usize, i: usize, j: usize) -> Vec<Adjacent> {
    [
        (i.checked_sub(1), j.checked_sub(1)),
        (Some(i), j.checked_sub(1)),
        (i.checked_add(1), j.checked_sub(1)),
        (i.checked_sub(1), Some(j)),
        (i.checked_add(1), Some(j)),
        (i.checked_sub(1), j.checked_add(1)),
        (Some(i), j.checked_add(1)),
        (i.checked_add(1), j.checked_add(1)),
    ]
    .into_iter()
    .map(|o| -> Option<Adjacent> {
        match o {
            (Some(some_i), Some(some_j)) if (some_i < cols && some_j < rows) => Some(Adjacent {
                i: some_i,
                j: some_j,
            }),
            _ => None,
        }
    })
    .flatten()
    .collect()
}

fn gen_mine_sweeper(mine2d: Vec<String>) -> Vec<Vec<i32>> {
    let rows = mine2d.len();
    let cols = mine2d[0].chars().count();
    mine2d
        .iter()
        .enumerate()
        .map(|(j, row_str)| -> Vec<i32> {
            row_str
                .chars()
                .enumerate()
                .map(|(i, c)| -> i32 {
                    match is_mine(c) {
                        true => -1,
                        false => check_mines_adjacents(rows, cols, i, j)
                            .iter()
                            .map(|a| -> i32 {
                                let local_row: Vec<char> = mine2d[a.j].chars().collect();
                                mine_to_int(local_row[a.i])
                            })
                            .sum::<i32>(),
                    }
                })
                .collect()
        })
        .collect()
}

fn main() {
    let sample_mine_sweeper: Vec<String> = vec![
        String::from("·*·*·"),
        String::from("··*··"),
        String::from("··*··"),
        String::from("·····"),
    ];

    let lines: Vec<String> = gen_mine_sweeper(sample_mine_sweeper)
        .iter()
        .map(|ms| ms.iter().map(|v| mine_to_str(*v)).collect::<String>())
        .collect();
    for l in lines {
        println!("{}", l);
    }
}
