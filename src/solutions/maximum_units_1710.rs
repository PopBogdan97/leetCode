//! You are assigned to put some amount of boxes onto one truck. You are given a 2D array boxTypes,
//! where boxTypes[i] = [numberOfBoxesi, numberOfUnitsPerBoxi]:
//!
//!     numberOfBoxesi is the number of boxes of type i.
//!         numberOfUnitsPerBoxi is the number of units in each box of the type i.
//!
//!         You are also given an integer truckSize, which is the maximum number of boxes that can
//!         be put on the truck. You can choose any boxes to put on the truck as long as the
//!         number of boxes does not exceed truckSize.
//!
//!         Return the maximum total number of units that can be put on the truck.
//!
//!
//!
//!         Example 1:
//!
//!         Input: boxTypes = [[1,3],[2,2],[3,1]], truckSize = 4
//!         Output: 8
//!         Explanation: There are:
//!         - 1 box of the first type that contains 3 units.
//!         - 2 boxes of the second type that contain 2 units each.
//!         - 3 boxes of the third type that contain 1 unit each.
//!         You can take all the boxes of the first and second types, and one box of the third
//!         type.
//!         The total number of units will be = (1 * 3) + (2 * 2) + (1 * 1) = 8.
//!
//!         Example 2:
//!
//!         Input: boxTypes = [[5,10],[2,5],[4,7],[3,9]], truckSize = 10
//!         Output: 91
//!
//!
//!
//!         Constraints:
//!
//!             1 <= boxTypes.length <= 1000
//!                 1 <= numberOfBoxesi, numberOfUnitsPerBoxi <= 1000
//!                     1 <= truckSize <= 106
//!

mod maximum_units_1710 {
    use std::cmp::Ordering;

    /// In order to return the maximum total number of units that can be put on the truck, we need
    /// to:
    /// 1. Order the 2D box_types array in decreasing order for the units that can be contained in
    ///    one box type.
    /// 2. Start from the beginning of the array and put the boxes into the truck until its
    ///    capacity is full while counting the units that the boxes put into the truck correspond
    ///    to.
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        // Sort the array by the number of units in each type of box
        let mut box_array = box_types.clone();
        box_array.sort_by(|a, b| b[1].cmp(&a[1]));
        println!("The sorted vecotr is: {:#?}", box_array);

        // Count the number of units until the truck is fulfilled.

        let mut units = 0;
        let mut capacity = truck_size;

        for b in box_array {
            match b[0].cmp(&capacity) {
                Ordering::Greater => {
                    units = units + capacity * b[1];
                    capacity = 0;
                }
                _ => {
                    units = units + b[0] * b[1];
                    capacity = capacity - b[0];
                }
            }

            if capacity <= 0 {
                break;
            }
        }

        units
    }
}

#[cfg(test)]
mod tests {
    use super::maximum_units_1710::maximum_units;

    #[test]
    fn test_example_1() {
        let box_types = vec![vec![1, 3], vec![2, 2], vec![3, 1]];
        let truck_size = 4;

        let result = maximum_units(box_types, truck_size);

        assert_eq!(result, 8);
    }
}
