pub struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums3 = Vec::new();

        let nums1_len = nums1.len();
        let nums2_len = nums2.len();

        let mut nums1_index = 0;
        let mut nums2_index = 0;

        loop {
            if nums1_index < nums1_len && nums2_index < nums2_len {
                if nums1[nums1_index] < nums2[nums2_index] {
                    nums3.push(nums1[nums1_index]);
                    nums1_index += 1;
                } else if nums1[nums1_index] > nums2[nums2_index] {
                    nums3.push(nums2[nums2_index]);
                    nums2_index += 1;
                } else {
                    nums3.push(nums2[nums2_index]);
                    nums3.push(nums1[nums1_index]);
                    nums1_index += 1;
                    nums2_index += 1;
                }
            } else if nums1_index < nums1_len {
                nums3.push(nums1[nums1_index]);
                nums1_index += 1;
            } else if nums2_index < nums2_len {
                nums3.push(nums2[nums2_index]);
                nums2_index += 1;
            } else {
                break;
            }
        }

        let nums3_len = nums1_len + nums2_len;

        if nums3_len % 2 == 0 {
            let index = nums3_len / 2;

            (nums3[index] + nums3[index - 1]) as f64 / 2 as f64
        } else {
            nums3[(nums3_len - 1) / 2] as f64
        }
    }
}
