# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def lowestCommonAncestor(self, root: 'TreeNode', p: 'TreeNode', q: 'TreeNode') -> 'TreeNode':
        # dfs with a path-saving and comparing system
        paths = []
        path = []
        def search(n):
            if len(paths) > 1:
                return
            path.append(n)
            if n == p or n == q:
                paths.append(path[:])
            if n.left:
                search(n.left)
            if n.right:
                search(n.right)
            path.pop()
        search(root)
        l = min(map(len, paths))
        for a,b in zip(reversed(paths[0][:l]), reversed(paths[1][:l])):
            if a == b:
                return a