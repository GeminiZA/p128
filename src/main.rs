struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut best: i32 = 0;
        let mut set: HashSet<i32> = HashSet::new();
        for num in nums {
            set.insert(num);
        }
        while set.len() > 0 {
            let mut a: i32 = 0;
            if let Some(first) = set.iter().next() {
                a = first.clone();
                set.remove(&a);
            }
            let mut cur: i32 = 1;
            let mut b: i32 = a + 1;
            let mut c: i32 = a - 1;
            while set.len() > 0 && set.contains(&b) {
                set.remove(&b);
                b += 1;
                cur += 1;
            }
            while set.len() > 0 && set.contains(&c) {
                set.remove(&c);
                c -= 1;
                cur += 1;
            }
            if cur > best {
                best = cur;
            }
        }
        return best;
    }
}
fn main() {
    let input: Vec<i32> = vec![0,3,7,2,5,8,4,6,0,1];
    let result: i32 = Solution::longest_consecutive(input);
    println!("{}", result);
}