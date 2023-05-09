//! Given an m x n matrix, return all elements of the matrix in spiral order.
//!
//!  
//!
//!  Example 1:
//!
//!  Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
//!  Output: [1,2,3,6,9,8,7,4,5]
//!
//!  Example 2:
//!
//!  Input: matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
//!  Output: [1,2,3,4,8,12,11,10,9,5,6,7]
//!
//!   
//!
//!   Constraints:
//!
//!       m == matrix.length
//!           n == matrix[i].length
//!               1 <= m, n <= 10
//!                   -100 <= matrix[i][j] <= 100

mod spiral_matrix_54 {
    /// The idea is to use a while loop to iterate over the matrix until all the elements have been
    /// visited and keep track of the current position in the matrix.
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let n_items = matrix.len() * matrix[0].len();

        let mut row_start = 1;
        let mut row_end = matrix.len() - 1;
        let mut col_start = 0;
        let mut col_end = matrix[0].len() - 1;
        let mut i = 0;
        let mut j = 0;

        while result.len() < n_items {
            while j <= col_end && result.len() < n_items {
                result.push(matrix[i][j]);
                if j == col_end {
                    i += 1;
                    break;
                } else {
                    j += 1;
                }
            }
            col_end -= 1;

            while i <= row_end && result.len() < n_items {
                result.push(matrix[i][j]);
                if i == row_end {
                    j -= 1;
                    break;
                } else {
                    i += 1;
                }
            }
            row_end -= 1;

            while j >= col_start && result.len() < n_items {
                result.push(matrix[i][j]);
                if j == col_start {
                    i -= 1;
                    break;
                } else {
                    j -= 1;
                }
            }
            col_start += 1;

            while i >= row_start && result.len() < n_items {
                result.push(matrix[i][j]);
                if i == row_start {
                    j += 1;
                    break;
                } else {
                    i -= 1;
                }
            }
            row_start += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::spiral_matrix_54::spiral_order;

    #[test]
    fn test_example_1() {
        let ratings = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        assert_eq!(vec![1, 2, 3, 6, 9, 8, 7, 4, 5], spiral_order(ratings));
    }

    #[test]
    fn test_example_2() {
        let ratings = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];

        assert_eq!(
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
            spiral_order(ratings)
        );
    }

    // #[test]
    // fn test_example_3() {
    //     let ratings = vec![1, 2, 87, 87, 87, 2, 1];
    //
    //     assert_eq!(13, candy(ratings));
    // }
    //
    // #[test]
    // fn test_example_4() {
    //     let ratings = vec![1, 3, 4, 5, 2];
    //
    //     assert_eq!(11, candy(ratings));
    // }
}
