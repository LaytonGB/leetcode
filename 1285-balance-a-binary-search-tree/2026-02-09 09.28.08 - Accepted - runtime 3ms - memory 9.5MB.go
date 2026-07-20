/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func balanceBST(root *TreeNode) *TreeNode {
    list := &[]int{}
    addToNodeList(list, root)
    return getBalancedTree(*list)
}

func getBalancedTree(list []int) *TreeNode {    
    if len(list) == 0 {
        return nil
    }
    
    m := len(list) / 2
    root := &TreeNode{Val: list[m]}
    root.Left = getBalancedTree(list[:m])
    root.Right = getBalancedTree(list[m+1:])

    return root
}

func addToNodeList(list *[]int, root *TreeNode) {
    if root.Left != nil {
        addToNodeList(list, root.Left)
    }

    *list = append(*list, root.Val)

    if root.Right != nil {
        addToNodeList(list, root.Right)
    }
}
