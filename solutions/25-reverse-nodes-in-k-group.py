# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reverseKGroup(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        rev = []
        n = head
        for i in range(k):
            if n:
                rev.append(n)
                n = n.next
            else:
                break
        nex = None
        if n:
            nex = self.reverseKGroup(n, k);
        if len(rev) == k:
            for a,b in zip(rev[:-1], rev[1:]):
                b.next = a
            rev[0].next = nex
            rev = rev[::-1]
        return rev[0]