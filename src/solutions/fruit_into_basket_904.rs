//! You are visiting a farm that has a single row of fruit trees arranged from left to right. The
//! trees are represented by an integer array fruits where fruits[i] is the type of fruit the ith
//! tree produces.
//!
//! You want to collect as much fruit as possible. However, the owner has some strict rules that
//! you must follow:
//!
//!     You only have two baskets, and each basket can only hold a single type of fruit. There is
//!     no limit on the amount of fruit each basket can hold.
//!         Starting from any tree of your choice, you must pick exactly one fruit from every tree
//!         (including the start tree) while moving to the right. The picked fruits must fit in one
//!         of your baskets.
//!             Once you reach a tree with fruit that cannot fit in your baskets, you must stop.
//!
//!             Given the integer array fruits, return the maximum number of fruits you can pick.
//!
//!              
//!
//!              Example 1:
//!
//!              Input: fruits = [1,2,1]
//!              Output: 3
//!              Explanation: We can pick from all 3 trees.
//!
//!              Example 2:
//!
//!              Input: fruits = [0,1,2,2]
//!              Output: 3
//!              Explanation: We can pick from trees [1,2,2].
//!              If we had started at the first tree, we would only pick from trees [0,1].
//!
//!              Example 3:
//!
//!              Input: fruits = [1,2,3,2,2]
//!              Output: 4
//!              Explanation: We can pick from trees [2,3,2,2].
//!              If we had started at the first tree, we would only pick from trees [1,2].
//!
//!               
//!
//!               Constraints:
//!
//!                   1 <= fruits.length <= 105
//!                       0 <= fruits[i] < fruits.length
//!
//!

mod fruit_into_basket_904 {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut max_total_fruits = 0;
     
        let mut basket1 = -1;
        let mut basket2 = -1;
        let mut len_b1 = 0;
        let mut len_b2 = 0;
        let mut prev_type = -1;
        let mut max_unique = 0;

        for fruit in fruits {
            if basket1 < 0 {
                basket1 = fruit;
                len_b1 = len_b1 + 1;
                if prev_type == fruit {
                    max_unique = max_unique + 1;
                } else {
                    max_unique = 1;
                    prev_type = fruit;
                }
            } else if basket2 < 0 && fruit != basket1 {
                basket2 = fruit;
                len_b2 = len_b2 + 1;
                if prev_type == fruit {
                    max_unique = max_unique + 1;
                } else {
                    max_unique = 1;
                    prev_type = fruit;
                }
            } else if fruit == basket1 {
                len_b1 = len_b1 + 1;
                if prev_type == fruit {
                    max_unique = max_unique + 1;
                } else {
                    max_unique = 1;
                    prev_type = fruit;
                }
            } else if fruit == basket2 {
                len_b2 = len_b2 + 1;
                if prev_type == fruit {
                    max_unique = max_unique + 1;
                } else {
                    max_unique = 1;
                    prev_type = fruit;
                }
            } else {
                let total_fruits = len_b2 + len_b1;
                if total_fruits > max_total_fruits {
                    max_total_fruits = total_fruits;
                }

                if prev_type == basket1 {
                    len_b2 = 1;
                    len_b1 = max_unique;
                    basket2 = fruit;
                    prev_type = fruit;
                    max_unique = 1;
                } else {
                    len_b1 = 1;
                    len_b2 = max_unique;

                    basket1 = fruit;
                    prev_type = fruit;
                    max_unique = 1;
                }
            }
        }
        let total_fruits = len_b2 + len_b1;
        if total_fruits > max_total_fruits {
            max_total_fruits = total_fruits;
        }
        max_total_fruits
    }
}

#[cfg(test)]
mod tests {
    use super::fruit_into_basket_904::total_fruit;

    #[test]
    fn test_example_1() {
        let fruit = vec![1, 2, 1];

        assert_eq!(3, total_fruit(fruit));
    }

    #[test]
    fn test_example_2() {
        let fruit = vec![0, 1, 2, 2];

        assert_eq!(3, total_fruit(fruit));
    }

    #[test]
    fn test_example_3() {
        let fruit = vec![1, 2, 3, 2, 2];

        assert_eq!(4, total_fruit(fruit));
    }

    #[test]
    fn test_example_4() {
        let fruit = vec![1, 3, 4, 5, 2];

        assert_eq!(2, total_fruit(fruit));
    }

    #[test]
    fn test_example_5() {
        let fruit = vec![1, 0, 3, 4, 3];

        assert_eq!(3, total_fruit(fruit));
    }

    #[test]
    fn test_example_6() {
        let fruit = vec![1, 0, 1, 4, 1, 4, 1, 2, 3];

        assert_eq!(5, total_fruit(fruit));
    }
}
