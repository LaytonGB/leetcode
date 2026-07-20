"""
# Definition for a Node.
class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next
"""

class Solution:
    def connect(self, root: 'Node') -> 'Node':
        if root is None:
            return root
        
        prev, curr = [], [root]
        
        while len(curr) > 0:
            for i in range(len(curr) - 1):
                curr[i].next = curr[i + 1]
            prev = curr
            curr = []
            for p in prev:
                if p.left:
                    curr.append(p.left)
                if p.right:
                    curr.append(p.right)
        
        return root
        