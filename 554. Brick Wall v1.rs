impl Solution {
    pub fn least_bricks(mut wall: Vec<Vec<i32>>) -> i32 {
        let mut max_removed = 0;
        loop {
            let min_width = wall.iter().map(|row| *row.last().unwrap()).min().unwrap();
            let count_removed = wall.iter_mut().filter_map(|row| {
                    *row.last_mut().unwrap() -= min_width;
                    if *row.last().unwrap() == 0 { row.pop(); Some(()) }
                    else { None }
                })
                .count() as i32;
            if wall.first().unwrap().is_empty() {
                break;
            }
            else {
                max_removed = std::cmp::max(max_removed, count_removed);
            }
        }
        wall.len() as i32 - max_removed
    }
}
