class Solution:
    def intToRoman(self, num: int) -> str:
        out = []
        
        def calc(num, lim, change, char1, char2):
            while num >= lim:
                num -= change
                out.append(char1)
            if num < 0:
                out.insert(-1, char2)
                num += change - lim
            return num
                
        num = calc(num, 900, 1000, "M", "C")
        num = calc(num, 400, 500, "D", "C")
        num = calc(num, 90, 100, "C", "X")
        num = calc(num, 40, 50, "L", "X")
        num = calc(num, 9, 10, "X", "I")
        num = calc(num, 4, 5, "V", "I")
        num = calc(num, 1, 1, "I", "err")
            
        return "".join(out)