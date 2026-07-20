/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func maxLevelSum(root *TreeNode) int {
    maxLevel := 0
    maxSum := -int(^uint(0) >> 1) - 1 // init with minimum integer
    
    level := 1

    q := []*TreeNode{root}
    for len(q) > 0 {
        levelSize := len(q)
        sum := 0

        for i := 0; i < levelSize; i++ {
            node := q[0]
            q = q[1:]

            sum += node.Val
            if node.Left != nil {
                q = append(q, node.Left)
            }
            if node.Right != nil {
                q = append(q, node.Right)
            }
        }

        if sum > maxSum {
            maxLevel = level
            maxSum = sum
        }

        level++
    }

    return maxLevel
}
