const ZERO byte = byte('0')
const ONE byte = byte('1')

func numSteps(s string) int {
    n := len(s)
    firstOneIndex := 0
    for i := range len(s) {
        if s[n-i-1] == ONE {
            firstOneIndex = n - i - 1
            break
        }
    }

    if firstOneIndex == 0 {
        return n - 1
    }

    remainingZeroes := 0
    for i := range firstOneIndex {
        if s[i] == ZERO {
            remainingZeroes++
        }
    }

    return 1 + n + remainingZeroes
}