class Solution {
public:
    vector<int> searchRange(vector<int>& nums, int target) {
        int l = 0, h = nums.size(), m = l + ((h - l) / 2);
        int lm, hm;
        
        while (l < h && nums[m] != target) {
            if (nums[m] < target)
                l = m + 1;
            else
                h = m;
            m = l + ((h - l) / 2);
        }
        
        if (l >= h)
            return {-1, -1};
        
        l = 0; h = m + 1; lm = l + ((h - l) / 2);
        while (l < h && (nums[lm] != target || lm > 0 && nums[lm - 1] == target)) {
            if (nums[lm] == target)
                h = lm;
            else
                l = lm + 1;
            lm = l + ((h - l) / 2);
        }
        
        l = m; h = nums.size(); hm = l + ((h - l) / 2);
        while(l < h && (nums[hm] != target || hm + 1 < nums.size() && nums[hm + 1] == target)) {
            cout << l << ", " << hm << ", " << h << "\n";
            if (nums[hm] == target)
                l = hm + 1;
            else
                h = hm;
            hm = l + ((h - l) / 2);
        }
        
        return {lm, hm};
    }
};