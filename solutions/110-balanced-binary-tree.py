def getDepth(node: TreeNode, depth: int) -> int:
    l_depth = depth; r_depth = depth
    if node.left:
        l_depth = getDepth(node.left, depth + 1)
    if node.right:
        r_depth = getDepth(node.right, depth + 1)
    if node.left == -1 or node.right == -1 or abs(l_depth - r_depth) > 1:
        return -1
    return max(l_depth, r_depth)
    
    
class Solution:
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        if not root or (not root.left and not root.right):
            return True
        
        l_depth = getDepth(root.left, 2) if root.left else 1
        r_depth = getDepth(root.right, 2) if root.right else 1
        
        return l_depth != -1 and r_depth != -1 and not abs(l_depth - r_depth) > 1
        