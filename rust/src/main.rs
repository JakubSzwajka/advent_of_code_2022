pub mod common;
fn is_visible(
    tree_xy: (usize, usize),
    tree_height: &u32,
    tree_map: &Vec<Vec<u32>>,
    map_size_xy: (usize, usize),
) -> bool {
    let mut is_visible = true;

    // println!("---------------------");
    // println!(
    //     "Checking x:{}, y:{} - {}",
    //     tree_xy.0, tree_xy.1, tree_height
    // );
    // println!("from left to middle");
    // traverse left
    for i in 0..tree_xy.0 {
        if tree_map[tree_xy.1][i] >= *tree_height {
            is_visible = false;
            break; // this can go higher
        }
    }

    if is_visible {
        return true;
    } else {
        is_visible = true;
    }

    // traverse right
    for i in tree_xy.0 + 1..map_size_xy.1 {
        if tree_map[tree_xy.1][i] >= *tree_height {
            is_visible = false;
            break; // this can go higher
        }
    }

    if is_visible {
        return true;
    } else {
        is_visible = true;
    }

    // traverse up
    for i in 0..tree_xy.1 {
        if tree_map[i][tree_xy.0] >= *tree_height {
            is_visible = false;
            break; // this can go higher
        }
    }

    if is_visible {
        return true;
    } else {
        is_visible = true;
    }

    // traverse down
    for i in tree_xy.1 + 1..map_size_xy.0 {
        if tree_map[i][tree_xy.0] >= *tree_height {
            is_visible = false;
            break; // this can go higher
        }
    }

    return is_visible;
}

fn main() {
    let args = common::read_args();
    let tree_map = common::read_file(&args[1]).unwrap();

    let tree_map = tree_map
        .lines()
        .map(|x| {
            x.chars()
                .map(|n| n.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    // border trees
    let mut visible_trees: usize = 0;
    let map_width = tree_map.len();
    let map_height = tree_map[0].len();

    visible_trees = visible_trees + (map_width * 2);
    visible_trees = visible_trees + (map_height * 2);
    visible_trees = visible_trees - 4; // corner duplicates

    for (y, tree_row) in tree_map[1..tree_map.len() - 1].iter().enumerate() {
        for (x, tree) in tree_row[1..tree_row.len() - 1].iter().enumerate() {
            if is_visible((x + 1, y + 1), tree, &tree_map, (map_width, map_height)) {
                visible_trees = visible_trees + 1;
            }
        }
    }

    dbg!(&visible_trees);
}
