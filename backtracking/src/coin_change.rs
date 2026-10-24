// Minimum Number of Coins to Make Up a Given Value
// You are given coins of different denominations and an amount. Write a function to compute the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return -1.

// Example 1:
// Input: coins = [1, 2, 5], amount = 11

// Output: 3

// Explanation:
// 11 = 5 + 5 + 1, 3 total coins

// Example 2:
// Input: coins = [3], amount = 1

// Output: -1

fn coin_change_dfs(coins: &Vec<i32>, remaining_amount: i32, memo: &mut Vec<i32>) -> i32 {
    if remaining_amount == 0 {
        return 0;
    } else if remaining_amount < 0 {
        return -1;
    }

    if memo[remaining_amount as usize] >= -1 {
        return memo[remaining_amount as usize];
    } 

    let mut curr_min = i32::MAX;
    let mut r = 0;
    for c in coins {
        r = coin_change_dfs(coins, remaining_amount - c, memo);
        if r == -1 {
            continue;
        }
        curr_min = std::cmp::min(r + 1, curr_min);
    }

    curr_min = if curr_min == i32::MAX { -1 } else { curr_min };
    memo[remaining_amount as usize] = curr_min;
    curr_min
}

fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    // ramaining_amount => coins needed
    // -1 not possible
    // i32::MIN - not visited
    let mut memo = vec![i32::MIN; (amount + 1) as usize];
    coin_change_dfs(&coins, amount, &mut memo)
}
