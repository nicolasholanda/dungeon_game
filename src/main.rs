use dungeon_game::Solution;

fn main() {
    let dungeon = vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]];
    let min_hp = Solution::calculate_minimum_hp(dungeon);
    println!("Minimum initial health required: {}", min_hp);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_minimum_hp() {
        let dungeon = vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]];
        assert_eq!(Solution::calculate_minimum_hp(dungeon), 7);
    }

    #[test]
    fn test_single_positive_room() {
        let dungeon = vec![vec![5]];
        assert_eq!(Solution::calculate_minimum_hp(dungeon), 1);
    }

    #[test]
    fn test_single_negative_room() {
        let dungeon = vec![vec![-5]];
        assert_eq!(Solution::calculate_minimum_hp(dungeon), 6);
    }

    #[test]
    fn test_all_negative() {
        let dungeon = vec![vec![-2, -3], vec![-5, -10]];
        assert_eq!(Solution::calculate_minimum_hp(dungeon), 16);
    }

    #[test]
    fn test_all_positive() {
        let dungeon = vec![vec![2, 3], vec![5, 10]];
        assert_eq!(Solution::calculate_minimum_hp(dungeon), 1);
    }

    #[test]
    fn test_mixed_small() {
        let dungeon = vec![vec![0, -3], vec![-10, 1]];
        assert_eq!(Solution::calculate_minimum_hp(dungeon), 4);
    }

    #[test]
    fn test_large_mixed_dungeon() {
        let dungeon = vec![
            vec![-2, -3, 3, -1],
            vec![-5, -10, 1, 30],
            vec![10, 30, -5, -10],
            vec![-1, -1, -1, -1],
        ];
        assert_eq!(Solution::calculate_minimum_hp(dungeon), 6);
    }

    #[test]
    fn test_multiple_healing_rooms() {
        let dungeon = vec![
            vec![-2, 10, -3],
            vec![-5, 20, -10],
            vec![10, -30, -5],
        ];
        assert_eq!(Solution::calculate_minimum_hp(dungeon), 3);
    }

    #[test]
    fn test_all_negative_except_end() {
        let dungeon = vec![
            vec![-2, -3, -3],
            vec![-5, -10, -1],
            vec![-10, -30, 100],
        ];
        assert_eq!(Solution::calculate_minimum_hp(dungeon), 10);
    }

    #[test]
    fn test_large_negative_at_end() {
        let dungeon = vec![
            vec![0, 0, 0],
            vec![1, 1, 1],
            vec![2, 2, -100],
        ];
        assert_eq!(Solution::calculate_minimum_hp(dungeon), 96);
    }

    #[test]
    fn test_single_row() {
        let dungeon = vec![vec![-2, -3, 3, -1, 2]];
        assert_eq!(Solution::calculate_minimum_hp(dungeon), 6);
    }

    #[test]
    fn test_single_column() {
        let dungeon = vec![vec![-2], vec![-3], vec![3], vec![-1], vec![2]];
        assert_eq!(Solution::calculate_minimum_hp(dungeon), 6);
    }
}
