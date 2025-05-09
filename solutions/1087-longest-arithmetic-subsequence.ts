function longestArithSeqLength(nums: number[]): number {
    const n: number = nums.length;
    const dp: number[][] = Array.from({length: n}, () => new Array(1001).fill(0)); // dp[index][diff]
    let res: number = 1;
    for (let i = 0; i < nums.length - 1; i++) {
        for (let j = i + 1; j < nums.length; j++) {
            // add here because some combinations of nums[j] - nums[i] result in negative values (not indexable)
            const k = nums[j] - nums[i] + 500; 
            dp[j][k] = Math.max(dp[j][k], dp[i][k] + 1);
            res = Math.max(res, dp[j][k]);
        }
    }
    return res + 1;
};