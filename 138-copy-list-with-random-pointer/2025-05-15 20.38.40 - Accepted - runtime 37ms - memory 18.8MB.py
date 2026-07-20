"""
# Definition for a Node.
class Node:
    def __init__(self, x: int, next: 'Node' = None, random: 'Node' = None):
        self.val = int(x)
        self.next = next
        self.random = random
"""

class Solution:
    def copyRandomList(self, head: 'Optional[Node]') -> 'Optional[Node]':
        if head is None:
            return None
        
        inp = []
        out = []
        i = 0
        rev = dict()
        while head is not None:
            inp.append(head)
            out.append(Node(head.val))
            rev[head] = i
            head = head.next
            i += 1
        
        for i in range(len(out)):
            if inp[i].random:
                out[i].random = out[rev[inp[i].random]]
            if i < len(out) - 1:
                out[i].next = out[i + 1]

        return out[0]
        