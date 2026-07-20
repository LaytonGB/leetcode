/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func sumRootToLeaf(root *TreeNode) int {
    return r(root, 0)
}

func r(node *TreeNode, sum int) int {
    if node == nil {
        return 0
    }
    
    sum = sum * 2 + node.Val

    if node.Left == node.Right {
        return sum
    }

    return r(node.Left, sum) + r(node.Right, sum)
}
