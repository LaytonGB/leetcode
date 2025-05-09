# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def oddEvenList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None or head.next is None:
            return head
        
        evens = head
        odds = head.next
        ep = evens
        op = odds
        while ep.next is not None and op.next is not None:
            ep.next = op.next
            ep = ep.next
            op.next = ep.next
            op = op.next
        ep.next = odds
        return evens