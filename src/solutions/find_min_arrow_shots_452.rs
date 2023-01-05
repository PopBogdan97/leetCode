//! There are some spherical balloons taped onto a flat wall that represents the XY-plane. The
//! balloons are represented as a 2D integer array points where points[i] = [xstart, xend] denotes
//! a balloon whose horizontal diameter stretches between xstart and xend. You do not know the
//! exact y-coordinates of the balloons.
//!
//! Arrows can be shot up directly vertically (in the positive y-direction) from different points
//! along the x-axis. A balloon with xstart and xend is burst by an arrow shot at x if xstart <= x
//! <= xend. There is no limit to the number of arrows that can be shot. A shot arrow keeps
//! traveling up infinitely, bursting any balloons in its path.
//!
//! Given the array points, return the minimum number of arrows that must be shot to burst all
//! balloons.
//!
//!
//!
//! Example 1:
//!
//! Input: points = [[10,16],[2,8],[1,6],[7,12]]
//! Output: 2
//! Explanation: The balloons can be burst by 2 arrows:
//! - Shoot an arrow at x = 6, bursting the balloons [2,8] and [1,6].
//! - Shoot an arrow at x = 11, bursting the balloons [10,16] and [7,12].
//!
//! Example 2:
//!
//! Input: points = [[1,2],[3,4],[5,6],[7,8]]
//! Output: 4
//! Explanation: One arrow needs to be shot for each balloon for a total of 4 arrows.
//!
//! Example 3:
//!
//! Input: points = [[1,2],[2,3],[3,4],[4,5]]
//! Output: 2
//! Explanation: The balloons can be burst by 2 arrows:
//! - Shoot an arrow at x = 2, bursting the balloons [1,2] and [2,3].
//! - Shoot an arrow at x = 4, bursting the balloons [3,4] and [4,5].
//!
//!
//!
//! Constraints:
//!
//!     1 <= points.length <= 105
//!         points[i].length == 2
//!             -231 <= xstart < xend <= 231 - 1
//!

mod find_min_arrow_shots_452 {
    use std::i32;

    /// In order to find the minimum arrow shots, we need to proceed with the following steps:
    /// 1. Order the points in increasing order by the starting points.
    /// 2. Scan the points from the start and save the smallest ending point of the balloons
    ///    scanned.    
    /// 3. As soon as you encounter a starting point that is higher or equal than the smallest
    ///    starting point, you shoot an arrow.
    /// 4. Start again with the starting point that is higher than the smallest starting point
    ///    where you just shot.
    /// 5. Repeat 2,3 and 4 util the end of the balloons.

    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut balloons = points;
        let mut arrows = 0;
        let mut min_end = std::i32::MAX;
        // let mut flag: bool = false;
        // 1. Order the balloons
        balloons.sort_by_key(|p| p[0]);
        // println!("Sorted balloons: {:#?}", balloons);

        for b in balloons {
            if b[0] > min_end {
                arrows += 1;
                min_end = b[1];
            } else if min_end > b[1] {
                min_end = b[1];
            }
        }

        arrows += 1;
        arrows
    }
}

#[cfg(test)]
mod tests {
    use super::find_min_arrow_shots_452::find_min_arrow_shots;

    #[test]
    fn test_example_1() {
        let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];

        assert_eq!(2, find_min_arrow_shots(points));
    }

    #[test]
    fn test_example_2() {
        let points = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];

        assert_eq!(4, find_min_arrow_shots(points));
    }

    #[test]
    fn test_example_3() {
        let points = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];

        assert_eq!(2, find_min_arrow_shots(points));
    }

    #[test]
    fn test_example_4() {
        let points = vec![vec![-1, 1], vec![0, 1], vec![2, 3], vec![1, 2]];

        assert_eq!(2, find_min_arrow_shots(points));
    }

    #[test]
    fn test_example_5() {
        let points = vec![
            vec![4, 12],
            vec![7, 8],
            vec![7, 9],
            vec![7, 9],
            vec![2, 8],
            vec![6, 7],
            vec![5, 14],
            vec![4, 13],
        ];

        assert_eq!(1, find_min_arrow_shots(points));
    }
}
