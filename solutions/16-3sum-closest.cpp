class Solution {
public:
    int threeSumClosest(vector<int>& nums, int target) {
        std::sort(nums.begin(), nums.end());
        int o = nums[0] + nums[1] + nums[2];
        int l, r, s;
        for (int i = 0; i < nums.size() - 2; i++) {
            l = i + 1, r = nums.size() - 1;
            while (l < r) {
                s = nums[i] + nums[l] + nums[r];
                if (s == target)
                    return s;
                
                if (abs(target - s) < abs(target - o)) {
                    o = s;
                }
                
                if (s < target)
                    l++;
                else
                    r--;
            }
        }
        return o;
    }
};