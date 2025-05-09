/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func sortedListToBST(head *ListNode) *TreeNode {
    var s = listNodeToSlice(head)
    return sliceToBinaryTree(s)
}

func listNodeToSlice(head *ListNode) []int {
    var res []int
    curr := head
    for curr != nil {
        res = append(res, curr.Val)
        curr = curr.Next
    }
    return res
}

func sliceToBinaryTree(s []int) *TreeNode {
    switch len(s) {
        case 1:
            return &TreeNode{Val: s[0]}
        case 0:
            return nil
        default:
            var n = len(s)
            var left = sliceToBinaryTree(s[:n/2])
            var right = sliceToBinaryTree(s[n/2 + 1:])
            return &TreeNode{
                Val: s[n/2],
                Left: left,
                Right: right,
            }
    }
}