# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        # remove None
        i = 0
        while i < len(lists):
            if lists[i] is None:
                del lists[i]
            else:
                i += 1

        res = ListNode(-1)
        head = res
        while len(lists) > 0:
            # find lowest
            low = 10001
            low_i = None
            for i,n in enumerate(lists):
                if n.val < low:
                    low = n.val
                    low_i = i
            
            # for each, while val == lowest
            i = 0
            while i < len(lists):
                while lists[i].val == low:
                    head.next = lists[i]
                    head = head.next
                    lists[i] = lists[i].next
                    if not lists[i]:
                        del lists[i]
                        i -= 1
                        break
                i += 1
        head.next = None
        return res.next