def sumFrom(node: TreeNode, tgt: int, s: int) -> bool:
    s += node.val
    if not node.left and not node.right and s == tgt:
        return True
    
    l = False; r = False
    if node.left:
        l = sumFrom(node.left, tgt, s)
    if node.right:
        r = sumFrom(node.right, tgt, s)
    return l or r
    
class Solution:
    def hasPathSum(self, root: Optional[TreeNode], targetSum: int) -> bool:
        if not root:
            return False
        if not root.left and not root.right and root.val == targetSum:
            return True
        
        l = False; r = False
        if root.left:
            l = sumFrom(root.left, targetSum, root.val)
        if root.right:
            r = sumFrom(root.right, targetSum, root.val)
        return l or r
        