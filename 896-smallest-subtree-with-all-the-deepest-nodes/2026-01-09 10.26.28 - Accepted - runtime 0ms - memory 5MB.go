/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
var parent = make(map[int]*TreeNode)

func subtreeWithAllDeepest(root *TreeNode) *TreeNode {
    deepest := getDeepest(root)
    return climbToCommonNode(deepest)
}

func getDeepest(root *TreeNode) []*TreeNode {
    q := []*TreeNode{root}
    for len(q) > 0 {
        n := len(q)
        firstChildFoundIdx := -1
        for i := 0; i < n; i++ {
            var node *TreeNode
            if firstChildFoundIdx == -1 {
                node = q[i]
            } else {
                node = q[0]
            }

            hasChild := node.Left != nil || node.Right != nil
            if hasChild {
                if node.Left != nil {
                    q = append(q, node.Left)
                    parent[node.Left.Val] = node
                }
                if node.Right != nil {
                    q = append(q, node.Right)
                    parent[node.Right.Val] = node
                }

                // This is the first node with children found
                if firstChildFoundIdx == -1 {
                    firstChildFoundIdx = i
                    q = q[firstChildFoundIdx:]
                }
            }

            // Children have been found and this node is no longer needed
            if firstChildFoundIdx != -1 {
                q = q[1:]
            }
        }

        if firstChildFoundIdx == -1 {
            break
        }
    }
    return q
}

func climbToCommonNode(nodes []*TreeNode) *TreeNode {
    for len(nodes) > 1 {
        n := len(nodes)
        for i := 0; i < n; i++ {
            node := nodes[0]
            nodes = nodes[1:]

            p := parent[node.Val]

            if nodes[len(nodes)-1] != p {
                nodes = append(nodes, p)
            }
        }
    }
    return nodes[0]
}
