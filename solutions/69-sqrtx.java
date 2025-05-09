class Solution {
    public int mySqrt(int x) {
        if (x <= 1) {
            return x;
        }
        
        int sub = -1;
        while (x >= 0) {
            sub += 2;
            x -= sub;
        }
        return sub / 2;
    }
}