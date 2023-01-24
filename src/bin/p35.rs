struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = nums.len();
        let mut mid: usize;

        while left < right {
            mid = (left + right) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                left = mid + 1
            } else {
                right = mid
            }
        }

        left as i32
    }

    pub fn search_insert_2(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).unwrap_or_else(|x| x) as i32
    }
}

fn main() {
    let nums = vec![1, 3, 5, 5, 6];
    let target = 5;
    let r1 = Solution::search_insert(nums, target);
    println!("{:?}", r1);

    let nums = vec![1, 3];
    let target = 2;
    let r2 = Solution::search_insert(nums, target);
    println!("{:?}", r2);

    let nums = vec![1, 3, 5, 6];
    let target = 0;
    let r3 = Solution::search_insert(nums, target);
    println!("{:?}", r3);

    let nums = vec![1, 5, 5, 5, 6];
    let target = 5;
    let r4 = Solution::search_insert(nums, target);
    println!("{:?}", r4);

    let nums = vec![1, 5, 5, 5, 6];
    let target = 5;
    let r5 = Solution::search_insert_2(nums, target);
    println!("{:?}", r5);
}
