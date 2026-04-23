pub struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums3 = nums1;
        nums3.extend(nums2);
        nums3.sort();
        let length = nums3.len();

        if length % 2 == 0 {
            let index = length / 2;
            let first_number = nums3[index];
            let second_number = nums3[index - 1];
            let median = (first_number + second_number) as f64 / 2 as f64;
            return median;
        } else {
            let index = (length - 1) / 2;
            return nums3[index] as f64;
        }
    }
}
