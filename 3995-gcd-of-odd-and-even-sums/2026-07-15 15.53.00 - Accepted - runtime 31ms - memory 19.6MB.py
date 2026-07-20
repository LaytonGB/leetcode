import math

class Solution:
    def gcdOfOddEvenSums(self, n: int) -> int:
        sumOdd = 0
        sumEven = 0
        for i in range(1, n * 2 + 1):
            if i % 2 == 0:
                sumEven += i
            else:
                sumOdd += i
        return math.gcd(sumOdd, sumEven)