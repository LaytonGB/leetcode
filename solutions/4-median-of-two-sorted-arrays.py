class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        while len(nums1) > 1 and len(nums2) > 1:
            if nums1[len(nums1) - 1] > nums2[len(nums2) - 1]:
                del(nums1[len(nums1) - 1])
            else:
                del(nums2[len(nums2) - 1])
            
            if nums1[0] < nums2[0]:
                del(nums1[0])
            else:
                del(nums2[0])
                
        c = sorted(nums1 + nums2)
        while len(c) > 2:
            del(c[0])
            del(c[len(c) - 1])
        
        return float(sum(c) / len(c))