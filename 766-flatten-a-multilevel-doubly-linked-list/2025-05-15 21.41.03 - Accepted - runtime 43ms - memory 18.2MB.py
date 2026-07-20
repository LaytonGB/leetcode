"""
# Definition for a Node.
class Node:
    def __init__(self, val, prev, next, child):
        self.val = val
        self.prev = prev
        self.next = next
        self.child = child
"""

class Solution:
    def flatten(self, head: 'Optional[Node]') -> 'Optional[Node]':
        node = head
        while node is not None:
            if node.child is not None:
                n = node.next

                node.next = self.flatten(node.child)
                node.next.prev = node
                node.child = None

                while node.next is not None:
                    node = node.next

                node.next = n
                if n is not None:
                    n.prev = node
            else:
                node = node.next
        return head
            