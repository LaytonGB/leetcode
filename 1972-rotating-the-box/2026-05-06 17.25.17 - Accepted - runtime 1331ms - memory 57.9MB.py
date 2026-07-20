def rotateStatic(boxGrid):
    h, w = len(boxGrid), len(boxGrid[0])
    rotated = [['' for _ in range(h)] for _ in range(w)]
    
    for r in range(h):
        for c in range(w):
            rotated[c][h-r-1] = boxGrid[r][c]

    # printMat(rotated, "rotated")
    
    return rotated


def applyGravity(boxGrid):
    h, w = len(boxGrid), len(boxGrid[0])
    res = [['' for _ in range(w)] for _ in range(h)]

    for c in range(w):
        last = '*'
        for r in range(h-1, -1, -1):
            res[r][c] = boxGrid[r][c]
            while r >= 0 and r < h-1 and res[r][c] == '#' and res[r+1][c] == '.':
                res[r+1][c] = '#'
                res[r][c] = '.'
                r += 1
    
    return res


def printMat(mat, name=""):
    print("===", name, "===")
    for l in mat: print(l)
    print()


class Solution:
    def rotateTheBox(self, boxGrid: List[List[str]]) -> List[List[str]]:
        # printMat(boxGrid, "boxGrid")
        rotated = rotateStatic(boxGrid)
        res = applyGravity(rotated)
        # printMat(res, "res")
        return res