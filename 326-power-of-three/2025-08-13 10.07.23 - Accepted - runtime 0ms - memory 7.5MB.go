const maxValidPowerOfThree = 1162261467;

func isPowerOfThree(n int) bool {
    return n > 0 && maxValidPowerOfThree % n == 0
}