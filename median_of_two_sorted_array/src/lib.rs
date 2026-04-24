pub struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums3 = Vec::new();
        let nums1_length = nums1.len();
        let nums2_length = nums2.len();
        let nums3_length = nums1_length + nums2_length;

        let mut nums1_index = 0;
        let mut nums2_index = 0;

        while nums1_index < nums1.len() {
            if nums2_index < nums2_length {
                if nums1[nums1_index] < nums2[nums2_index] {
                    nums3.push(nums1[nums1_index]);
                    nums1_index += 1;
                } else if nums1[nums1_index] == nums2[nums2_index] {
                    nums3.push(nums1[nums1_index]);
                    nums3.push(nums2[nums2_index]);
                    nums1_index += 1;
                    nums2_index += 1;
                } else {
                    nums3.push(nums2[nums2_index]);
                    nums2_index += 1;
                }
            } else {
                nums3.push(nums1[nums1_index]);
                nums1_index += 1;
            }
        }
        while nums2_index < nums2_length {
            nums3.push(nums2[nums2_index]);
            nums2_index += 1;
        }

        if nums3_length % 2 == 0 {
            let index = nums3_length / 2;
            let first_number = nums3[index];
            let second_number = nums3[index - 1];
            let median = (first_number + second_number) as f64 / 2 as f64;
            return median;
        } else {
            let index = (nums3_length - 1) / 2;
            return nums3[index] as f64;
        }
    }
}
