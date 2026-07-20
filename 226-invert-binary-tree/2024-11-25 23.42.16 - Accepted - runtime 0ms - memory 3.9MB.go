/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func invertTree(root *TreeNode) *TreeNode {
    if root == nil {
        return nil
    }
    
    l := root.Left
    r := root.Right
    if l != nil {
        invertTree(root.Left)
    }
    if r != nil {
        invertTree(root.Right)
    }
    root.Right = l
    root.Left = r
    
    return root
}