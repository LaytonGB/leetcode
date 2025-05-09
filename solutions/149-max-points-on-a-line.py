from collections import defaultdict

class Solution:
    def maxPoints(self, points: List[List[int]]) -> int:
        n = len(points)
        res = 1
        for i in range(n - 1):
            p1 = points[i]
            coord_counts = defaultdict(int)
            identical = 1
            for j in range(i + 1, n):
                p2 = points[j]

                if p1 == p2:
                    identical += 1
                    continue
                
                dx, dy = p1[0] - p2[0], p1[1] - p2[1]
                if dx < 0 or (dx == 0 and dy < 0):
                    dx, dy = dx * -1, dy * -1

                gcd = math.gcd(dx, dy)
                key = dx / gcd, dy / gcd

                coord_counts[key] += 1

            res = max(res, max(coord_counts.values()) + identical)

        return res
        