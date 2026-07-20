/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func maxLevelSum(root *TreeNode) int {
    return levelSum([]*TreeNode{root})
}

func levelSum(nextNodes []*TreeNode) int {
    maxLevel := 0
    maxSum := -int(^uint(0) >> 1) - 1 // init with minimum integer
    
    level := 1
    sum := 0

    nodes := make([]*TreeNode, 0)
    for len(nextNodes) > 0 {
        nodes, nextNodes = nextNodes, nodes

        for _, node := range nodes {
            sum += node.Val

            if node.Left != nil {
                nextNodes = append(nextNodes, node.Left)
            }
            if node.Right != nil {
                nextNodes = append(nextNodes, node.Right)
            }
        }
        nodes = make([]*TreeNode, 0)

        if sum > maxSum {
            maxLevel = level
            maxSum = sum
        }

        level++
        sum = 0
    }

    return maxLevel
}