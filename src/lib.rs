pub struct Solution;

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
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

        dp[0][0]
    }
} 