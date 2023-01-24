struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut cur:usize = 0;
        loop {
            if cur == nums.len() {
                break;
            }
            if nums[cur] == val {
                nums.remove(cur);
            } else {
                cur += 1;
            }
        }
        nums.len() as i32
    }
}

fn main() {
    let mut nums:Vec<i32> = vec![3, 2, 2, 3];
    let val = 3;
    let r1 = Solution::remove_element(&mut nums, val);
    println!("{:?}", r1);
    println!("{:?}", nums);
}
