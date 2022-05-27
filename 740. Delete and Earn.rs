use std::collections::HashMap;
impl Solution {
    fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let num_to_points = {
            let mut num_to_points = HashMap::new();
            for &num in &nums {
                *num_to_points.entry(num).or_insert(0) += num;
            }
            num_to_points
        };
        
        let sorted_unique_nums = {
            let mut v: Vec<_> = num_to_points.keys().map(|&k| k).collect();
            v.push(0);
            if !num_to_points.contains_key(&1) {
                v.push(1);
            }
            v.sort_unstable();
            v
        };
        
        let mut max_points: HashMap<_, _> = sorted_unique_nums.iter().map(|&n| (n, 0)).collect();
        if let Some(&points_of_one) = num_to_points.get(&1) {
            *max_points.get_mut(&1).unwrap() = points_of_one;
        }
        
        for i in 2..sorted_unique_nums.len() {
            let num = sorted_unique_nums[i];
            let num_prev = sorted_unique_nums[i-1];
            let num_prev_prev = sorted_unique_nums[i-2];
            let (num_minus_one, num_minus_two) = 
                if num_prev + 1 == num { (num_prev, num_prev_prev) }
                else { (num_prev, num_prev) }
                ;
            
            *max_points.get_mut(&num).unwrap() = std::cmp::max(
                max_points[&num_minus_one],
                max_points[&num_minus_two] + num_to_points[&num],
            );
        }
        
        *max_points.get(sorted_unique_nums.last().unwrap()).unwrap()
    }
}
