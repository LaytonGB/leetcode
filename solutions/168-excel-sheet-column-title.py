class Solution:
    def convertToTitle(self, columnNumber: int) -> str:
        left = ""
        if columnNumber > 26:
            left = self.convertToTitle((columnNumber - 1) // 26)
            columnNumber = ((columnNumber - 1) % 26) + 1
        return left + chr(columnNumber + 64)
