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
    score: u64,
}

impl Los {
    fn new(v:u32) -> Self {
        Los {
            top: 0,
            bottom: 0,
            left: 0,
            right: 0,
            val: v,
            score: 0,
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

    // Do the simple check for all positions. There must be a cleverer way using some sort
    // dynamic programming

    let x = rows.last().unwrap().len();
    let y = rows.len();

    let mut max_score = 0;

    for i in 0..x {
        for j in 0..y {
            // Check to the left. if i = 0, left is 0, don't bother checking
            if i == 0 {
                rows[j][i].left = 0;
            } else {
                // same height or heigher - just one tree
                if rows[j][i - 1].val >= rows[j][i].val {
                    rows[j][i].left = 1;
                } else {
                    // Viewing distance is our neighbor and its viewing distance
                    // After that, we have to check if the view was blocked by
                    // Something larger than us or not
                    rows[j][i].left = rows[j][i-1].left + 1;
                    let mut k = i as u32 - rows[j][i-1].left - 1;
                    while k > 0 {
                        if rows[j][k as usize].val < rows[j][i].val {
                            rows[j][i].left += rows[j][k as usize].left;
                            k -= rows[j][k as usize].left;
                        }
                        else {
                            break;
                        }
                    }
                }
            }

            // Go up. if at top, just set to 0
            if j == 0 {
                rows[j][i].top = 0;
            } else {
                // same height or heigher - just one tree
                if rows[j - 1][i].val >= rows[j][i].val {
                    rows[j][i].top = 1;
                } else {
                    // Viewing distance is our neighbor and its viewing distance
                    // After that, we have to check if the view was blocked by
                    // Something larger as us or not
                    rows[j][i].top = rows[j - 1][i].top + 1;
                    let mut k = j as u32 - rows[j - 1][i].top - 1;
                    //println!("{} {} -> Top Jumping to {} for checking", i, j, k);
                    while k > 0 {
                        if rows[k as usize][i].val < rows[j][i].val {
                            rows[j][i].top += rows[k as usize][i].top;
                            k -= rows[k as usize][i].top;
                            //println!("{} {} -> Top Jumping to {} for checking", i, j, k);
                        }
                        else {
                            break;
                        }
                    }
                }
            }
        }
    }

    for i in (0..x).rev() {
        for j in (0..y).rev() {
            // Check to the right. if i = x - 1 , right is 0, don't bother checking
            if i == x - 1 {
                rows[j][i].right = 0;
            } else {
                // same height or heigher - just one tree
                if rows[j][i + 1].val >= rows[j][i].val {
                    rows[j][i].right = 1;
                } else {
                    // Viewing distance is our neighbor and its viewing distance
                    // After that, we have to check if the view was blocked by
                    // Something larger as us or not
                    rows[j][i].right = rows[j][i+1].right + 1;
                    let mut k = i as u32 + rows[j][i + 1].right + 1; 
                    //println!("{} {} -> right Jumping to {} for checking", i, j, k);
                    while k < (x - 1) as u32 {
                        if rows[j][k as usize].val < rows[j][i].val {
                            rows[j][i].right += rows[j][k as usize].right;
                            k += rows[j][k as usize].right;
                            //println!("{} {} -> right Jumping to {} for checking", i, j, k);
                        }
                        else {
                            break;
                        }
                    }
                }
            }

            if j == y - 1 {
                rows[j][i].bottom = 0;
            } else {
                // same height or heigher - just one tree
                if rows[j + 1][i].val >= rows[j][i].val {
                    rows[j][i].bottom = 1;
                } else {
                    // Viewing distance is our neighbor and its viewing distance
                    // After that, we have to check if the view was blocked by
                    // Something larger as us or not
                    rows[j][i].bottom = rows[j + 1][i].bottom + 1;
                    let mut k = j as u32 + rows[j + 1][i].bottom + 1;
                    //println!("{} {} -> bottom Jumping to {} for checking", i, j, k);
                    while k < (y - 1) as u32 {
                        if rows[k as usize][i].val < rows[j][i].val {
                            rows[j][i].bottom += rows[k as usize][i].bottom;
                            k += rows[k as usize][i].bottom;
                            //println!("{} {} -> bottom Jumping to {} for checking", i, j, k);
                        }
                        else {
                            break;
                        }
                    }
                }
            }
        }
    }

    for i in 0..x {
        for j in 0..y {
            let score = rows[j][i].left as u64 * rows[j][i].right as u64 * rows[j][i].top as u64  * rows[j][i].bottom as u64;
            //println!("{}x{}: l{} t{} r{} b{} s{}", i, j, rows[j][i].left, rows[j][i].top, rows[j][i].right, rows[j][i].bottom, score);
            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("{}x{} -> {}", x, y, max_score);

}

fn main() {
    // Consumes the iterator, returns an (Optional) String
    let foo = include_str!("../input");
    part_1(&foo);
    part_2(&foo);
}
