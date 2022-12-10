fn part_1(data: &str) {
    let mut rows:Vec<Vec<u32>> = Vec::new();
    for line in data.split("\n") {
        if line.is_empty() {
            continue;
        }
        rows.push(line.chars().map(|v| v as u32 - '0' as u32).collect());
    }

    let x = rows.last().unwrap().len();
    let y = rows.len();

    println!("{}x{}", x, y);

    let mut visible:Vec<Vec<u32>> = vec![
        vec![
            0;
            y
        ];
        x
    ];

    let mut row_count = 0;
    let mut max_column:Vec<u32> = Vec::with_capacity(x);

    for row in &rows {
        if row_count == 0 {
            max_column = vec![0; x];
            max_column.copy_from_slice(&row[..]);
            visible[row_count].fill(1);
        } else if row_count == y - 1 {
            // last row
            visible[row_count].fill(1);
        } else {
            let mut row_max = row[0];
            visible[row_count][0] = 1;
            visible[row_count][x - 1] = 1;
            for col in 1..x - 1 {
                if row[col] > row_max {
                    visible[row_count][col] = 1;
                    row_max = row[col];
                }
                if row[col] > max_column[col] {
                    visible[row_count][col] = 1;
                    max_column[col] = row[col];
                }
            }
        }

        row_count += 1;
    }

    row_count = 0;
    for row in rows.iter().rev() {
        if row_count == 0 {
            max_column = vec![0; x];
            max_column.copy_from_slice(&row[..]);
        } else if row_count == y - 1 {
            // last row
        } else {
            let mut row_max = row[x - 1];
            for col in (1..x - 1).rev() {
                //println!("@{},{} v:{} m:{}", col, row_count, row[col], row_max);
                if row[col] > row_max {
                    visible[y - row_count - 1][col] = 1;
                    row_max = row[col];
                }
                if row[col] > max_column[col] {
                    visible[y - row_count - 1][col] = 1;
                    max_column[col] = row[col];
                }
            }
        }

        row_count += 1;
    }

    let foo:u32 = visible.iter().map(|x| x.iter().sum::<u32>()).sum();
    println!("Part1: {}", foo);
}

struct Los {
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
    val: u32,
}

impl Los {
    fn new(v:u32) -> Self {
        Los {
            top: 0,
            bottom: 0,
            left: 0,
            right: 0,
            val: v,
        }
    }
}

fn part_2(data:&str) {
    let mut rows:Vec<Vec<Los>> = Vec::new();
    for line in data.split("\n") {
        if line.is_empty() {
            continue;
        }
        rows.push(line.chars().map(|v| Los::new(v as u32 - '0' as u32)).collect());
    }

    let x = rows.last().unwrap().len();
    let y = rows.len();

    println!("{}x{}", x, y);

    let mut row_count = 0;
    for mut row in &rows {
        for col in 0..x-1 {
            if col == 0 {
                row[col].left = 0;
            } else {
                if row[col - 1].val <= row[col] {
                    row[col].left += row.co
                }
            }
        }
        row_count += 1;
    }

}

fn main() {
    // Consumes the iterator, returns an (Optional) String
    let foo = include_str!("../example.txt");
    part_1(&foo);
    part_2(&foo);

}
