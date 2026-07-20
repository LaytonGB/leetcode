/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
var maxDepths = make(map[int]int)
var maxDepth int

func subtreeWithAllDeepest(root *TreeNode) *TreeNode {
    maxDepth = paintDepths(root, 0)
    return findDeepestCommonAncestor(root)
}

func paintDepths(root *TreeNode, depth int) int {
    if root == nil {
        return depth - 1
    }

    d := max(paintDepths(root.Left, depth + 1), paintDepths(root.Right, depth + 1))
    maxDepths[root.Val] = d
    return d
}

func findDeepestCommonAncestor(node *TreeNode) *TreeNode {
    var l, r int
    if node.Left != nil {
        l = maxDepths[node.Left.Val]
    }
    if node.Right != nil {
        r = maxDepths[node.Right.Val]
    }

    if l > r {
        return findDeepestCommonAncestor(node.Left)
    } else if l < r {
        return findDeepestCommonAncestor(node.Right)
    }
    return node
}
