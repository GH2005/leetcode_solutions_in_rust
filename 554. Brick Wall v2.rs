impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let count_edges = {
            let mut count_edges = std::collections::HashMap::new();
            for row in wall.iter() {
                for position in row
                    .iter()
                    .take(row.len() - 1)
                    .scan(0, |dist_from_left, &brick_width| {
                        *dist_from_left += brick_width;
                        Some(*dist_from_left)
                    })
                {
                    *count_edges.entry(position).or_insert(0) += 1;
                }
            }
            count_edges
        };
        
        wall.len() as i32 - count_edges.into_values().max().unwrap_or(0)
    }
}
