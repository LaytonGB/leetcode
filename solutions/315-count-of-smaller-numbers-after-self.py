class Solution:
    def countSmaller(self, nums: List[int]) -> List[int]:
        def sort(lis):
            m = len(lis) // 2
            if m:
                l, r = sort(lis[:m]), sort(lis[m:])
                # for each index, where i is the index of the index, reversed \U0001f62d
                for i in range(len(lis))[::-1]:
                    # if there is no right, and left[-1] greater than right[-1]
                    if not r or l and nums[l[-1]] > nums[r[-1]]:
                        # the last left value is the index of the number
                        # that is greater than all numbers in right
                        s[l[-1]] += len(r)
                        lis[i] = l.pop() # sort left half
                    else:
                        lis[i] = r.pop() # sort right half
            return lis
        s = [0] * len(nums)
        sort(list(range(len(nums)))) # get indexes and feed them into sort algorithm
        return s