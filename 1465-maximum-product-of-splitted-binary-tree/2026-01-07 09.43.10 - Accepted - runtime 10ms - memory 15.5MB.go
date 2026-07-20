const mod int64 = 1_000_000_007

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func maxProduct(root *TreeNode) int {
    treeSum := getSubtreeSums(root)

    var res int64 = 0
    q := []*TreeNode{root}

    for len(q) > 0 {
        node := q[0]
        q = q[1:]

        product := node.Val * (treeSum - node.Val)
        res = max(res, int64(product))

        if node.Left != nil {
            q = append(q, node.Left)
        }
        if node.Right != nil {
            q = append(q, node.Right)
        }
    }

    return int(res % mod)
}

func getSubtreeSums(root *TreeNode) int {
    sum := root.Val

    if root.Left != nil {
        sum += getSubtreeSums(root.Left)
    }
    if root.Right != nil {
        sum += getSubtreeSums(root.Right)
    }

    root.Val = sum
    return sum
}
