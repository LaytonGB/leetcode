class Solution:
    def removeElement(self, nums: List[int], val: int) -> int:
        if nums is None or val is None:
            return 0
        
        ptr = 0
        finding = False
        for i,n in enumerate(nums):
            if n != val:
                if finding:
                    nums[ptr] = n
                ptr += 1
            else:
                finding = True
        return ptr