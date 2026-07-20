public class Solution {
    public string[] SortPeople(string[] names, int[] heights) {
        var idxs = Enumerable.Range(0, names.Length).ToList();
        idxs.Sort((a, b) => heights[b].CompareTo(heights[a]));
        return idxs.Select(i => names[i]).ToArray();
    }
}