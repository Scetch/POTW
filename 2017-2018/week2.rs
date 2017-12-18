
use std::io::{ self, Read };

fn main() {
    let input = {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        input
    };

    let mut lines = input.lines();

    // Number of measurements
    let n = lines.next().and_then(|v| v.parse().ok()).unwrap();

    // Collect the total sum and a vector of (sum_of_previous_n, n)
    let (total, nums) = lines.next()
        .unwrap()
        .split(' ')
        .take(n)
        .flat_map(str::parse::<u32>)
        .fold((0, Vec::with_capacity(n)), |(sum, mut nums), v| {
            nums.push((sum, v));
            (sum + v, nums)
        });

    let res = nums.iter()
        .enumerate()
        .position(|(idx, &(s_l, v))| {
            // Helper closure to check if v is as least 15% lower than the average from s and n
            let lower = |s, n| n == 0 || ((s as f32 /  n as f32) - v as f32) >= v as f32 * 0.15;
            // For this to be the eye the current value must be 15% lower than both the average
            // values to the left and to the right of the current value
            lower(s_l, idx) && lower(total - (s_l + v), nums.len() - idx - 1)
        })
        .map(|v| v as i32)
        .unwrap_or(-1);

    println!("{}", res);
}