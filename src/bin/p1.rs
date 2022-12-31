use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            num_map.insert(n, i);
        }

        for (i, n) in nums.iter().enumerate() {
            let target_2 = num_map.get(&(target - n));
            match target_2 {
                Some(val) => {
                    if *val != i {
                        return vec![i as i32, *val as i32];
                    }
                }
                _ => continue,
            };
        }

        Vec::new()
    }

    pub fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mp = HashMap::new();
        for i in 0..nums.len() {
            let it = mp.get(&(target - nums[i]));
            match it {
                Some(val) => {
                    return vec![i as i32, *val];
                }
                None => mp.insert(nums[i], i as i32),
            };
        }
        Vec::new()
    }

    pub fn two_sum_3(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mp = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            let it = mp.get(&(target - n));
            match it {
                Some(val) => {
                    return vec![i as i32, *val];
                }
                None => mp.insert(nums[i], i as i32),
            };
        }
        Vec::new()
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let r1 = Solution::two_sum(nums, target);
    println!("{:?}", r1);

    let nums = vec![2, 7, 11, 15];
    let r2 = Solution::two_sum_2(nums, target);
    println!("{:?}", r2);

    let nums = vec![2, 7, 11, 15];
    let r3 = Solution::two_sum_3(nums, target);
    println!("{:?}", r3);
}
