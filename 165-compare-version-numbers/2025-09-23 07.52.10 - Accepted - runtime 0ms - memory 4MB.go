func compareVersion(version1 string, version2 string) int {
    parts1 := strings.Split(version1, ".")
    parts2 := strings.Split(version2, ".")

    nums1 := make([]int, len(parts1))
    for i := range parts1 {
        nums1[i], _ = strconv.Atoi(parts1[i])
    }

    nums2 := make([]int, len(parts2))
    for i := range parts2 {
        nums2[i], _ = strconv.Atoi(parts2[i])
    }

    for i := range min(len(nums1), len(nums2)) {
        if nums1[i] < nums2[i] {
            return -1
        } else if nums1[i] > nums2[i] {
            return 1
        }
    }

    if len(nums1) < len(nums2) {
        for _, n2 := range nums2[len(nums1):] {
            if n2 > 0 {
                return -1
            }
        }
    } else if len(nums1) > len(nums2) {
        for _, n1 := range nums1[len(nums2):] {
            if n1 > 0 {
                return 1
            }
        }
    }

    return 0
}