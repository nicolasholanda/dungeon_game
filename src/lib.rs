pub struct Solution;

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> (i32, Vec<(usize, usize)>) {
        let rows = dungeon.len();
        let cols = dungeon[0].len();

        let mut dp = vec![vec![i32::MAX; cols + 1]; rows + 1];

        dp[rows][cols - 1] = 1;
        dp[rows - 1][cols] = 1;

        for i in (0..rows).rev() {
            for j in (0..cols).rev() {
                let min_next = dp[i + 1][j].min(dp[i][j + 1]);
                let need = min_next - dungeon[i][j];
                dp[i][j] = if need <= 0 { 1 } else { need };
            }
        }

        let mut path = Vec::new();
        let mut i = 0;
        let mut j = 0;
        while i < rows && j < cols {
            path.push((i, j));
            if i == rows - 1 && j == cols - 1 {
                break;
            }
            if i + 1 < rows && j + 1 < cols {
                if dp[i + 1][j] < dp[i][j + 1] {
                    i += 1;
                } else {
                    j += 1;
                }
            } else if i + 1 < rows {
                i += 1;
            } else {
                j += 1;
            }
        }

        (dp[0][0], path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_minimum_hp() {
        let dungeon = vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]];
        assert_eq!(Solution::calculate_minimum_hp(dungeon).0, 7);
    }

    #[test]
    fn test_single_positive_room() {
        let dungeon = vec![vec![5]];
        assert_eq!(Solution::calculate_minimum_hp(dungeon).0, 1);
    }

    #[test]
    fn test_single_negative_room() {
        let dungeon = vec![vec![-5]];
        assert_eq!(Solution::calculate_minimum_hp(dungeon).0, 6);
    }

    #[test]
    fn test_all_negative() {
        let dungeon = vec![vec![-2, -3], vec![-5, -10]];
        assert_eq!(Solution::calculate_minimum_hp(dungeon).0, 16);
    }

    #[test]
    fn test_all_positive() {
        let dungeon = vec![vec![2, 3], vec![5, 10]];
        assert_eq!(Solution::calculate_minimum_hp(dungeon).0, 1);
    }

    #[test]
    fn test_mixed_small() {
        let dungeon = vec![vec![0, -3], vec![-10, 1]];
        assert_eq!(Solution::calculate_minimum_hp(dungeon).0, 4);
    }

    #[test]
    fn test_large_mixed_dungeon() {
        let dungeon = vec![
            vec![-2, -3, 3, -1],
            vec![-5, -10, 1, 30],
            vec![10, 30, -5, -10],
            vec![-1, -1, -1, -1],
        ];
        assert_eq!(Solution::calculate_minimum_hp(dungeon).0, 6);
    }

    #[test]
    fn test_multiple_healing_rooms() {
        let dungeon = vec![
            vec![-2, 10, -3],
            vec![-5, 20, -10],
            vec![10, -30, -5],
        ];
        assert_eq!(Solution::calculate_minimum_hp(dungeon).0, 3);
    }

    #[test]
    fn test_all_negative_except_end() {
        let dungeon = vec![
            vec![-2, -3, -3],
            vec![-5, -10, -1],
            vec![-10, -30, 100],
        ];
        assert_eq!(Solution::calculate_minimum_hp(dungeon).0, 10);
    }

    #[test]
    fn test_large_negative_at_end() {
        let dungeon = vec![
            vec![0, 0, 0],
            vec![1, 1, 1],
            vec![2, 2, -100],
        ];
        assert_eq!(Solution::calculate_minimum_hp(dungeon).0, 96);
    }

    #[test]
    fn test_single_row() {
        let dungeon = vec![vec![-2, -3, 3, -1, 2]];
        assert_eq!(Solution::calculate_minimum_hp(dungeon).0, 6);
    }

    #[test]
    fn test_single_column() {
        let dungeon = vec![vec![-2], vec![-3], vec![3], vec![-1], vec![2]];
        assert_eq!(Solution::calculate_minimum_hp(dungeon).0, 6);
    }
} 