/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     public int val;
 *     public TreeNode left;
 *     public TreeNode right;
 *     public TreeNode(int val=0, TreeNode left=null, TreeNode right=null) {
 *         this.val = val;
 *         this.left = left;
 *         this.right = right;
 *     }
 * }
 */
public class Solution {
    public IList<TreeNode> DelNodes(TreeNode root, int[] to_delete) {
        var roots = new List<TreeNode>();
        var sentinel = new TreeNode(-1, root);
        DFS(roots, to_delete, sentinel);
        if (!to_delete.Contains(root.val)) {
            roots.Add(root);
        }
        return roots;
    }

    private void DFS(IList<TreeNode> roots, int[] to_del, TreeNode node) {
        if (node == null) {
            return;
        }

        if (node.left != null) {
            DFS(roots, to_del, node.left);
            if (to_del.Contains(node.left.val)) {
                node.left = null;
            }
        }
        if (node.right != null) {
            DFS(roots, to_del, node.right);
            if (to_del.Contains(node.right.val)) {
                node.right = null;
            }
        }

        if (to_del.Contains(node.val)) {
            if (node.left != null) {
                roots.Add(node.left);
            }
            if (node.right != null) {
                roots.Add(node.right);
            }
        }
    }
}