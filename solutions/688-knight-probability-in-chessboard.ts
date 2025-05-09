function knightProbability(n: number, k: number, r: number, c: number): number {
    function isInBounds(r: number, c: number): boolean {
        return r >= 0 && c >= 0 && r < n && c < n;
    }
    
    const dirs = [[-1, 2], [-2, 1], [-2, -1], [-1, -2], [1, -2], [2, -1], [2, 1], [1, 2]];

    // pre-generate array in most optimized way https://stackoverflow.com/a/37951043/12425097
    let dp: number[][] = Array.from({length: n}, () => new Array(n).fill(0));
    dp[r][c] = 1;
    for (let i = 0; i < k; i++) {
        let next_dp = Array.from({length: n}, () => new Array(n).fill(0));
        for (let y = 0; y < n; y++) {
            for (let x = 0; x < n; x++) {
                for (let [a,b] of dirs) {
                    const row = y + a, col = x + b;
                    if (!isInBounds(row, col)) {
                        continue;
                    }
                    if (dp[row][col] > 0) {
                        next_dp[y][x] += 0.125 * dp[row][col];
                    }
                }
            }
        }
        dp = next_dp;
    }

    return dp.reduce((a,b) => a + b.reduce((c,d) => c + d, 0), 0);
};