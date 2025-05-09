class Solution:
    def addToArrayForm(self, num: List[int], k: int) -> List[int]:
        n = len(num)
        n1 = 0
        for i,x in enumerate(num):
            n1 += x * 10 ** (n - i - 1)
        res = n1 + k
        return [int(x) for x in str(res)]