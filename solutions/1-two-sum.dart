class Solution {
  List<int> twoSum(List<int> nums, int target) {
    var map = {};
    for (int i = 0; i < nums.length; i++) {
        if (map[nums[i]] != null) {
            return [map[nums[i]], i];
        }
        map[target - nums[i]] = i;
    }
    return [-1, -1];
  }
}