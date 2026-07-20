func maximumGain(s string, x int, y int) int {
    var (
        top, bot string
        topScore, botScore, res int
    )

    if x >= y {
        top = "ab"
        topScore = x
        bot = "ba"
        botScore = y
    } else {
        top = "ba"
        topScore = y
        bot = "ab"
        botScore = x
    }

    stack1 := []byte{}
    for i := 0; i < len(s); i++ {
        if s[i] == top[1] && len(stack1) > 0 && stack1[len(stack1) - 1] == top[0] {
            res += topScore
            stack1 = stack1[:len(stack1) - 1]
        } else {
            stack1 = append(stack1, s[i])
        }
    }

    stack2 := []byte{}
    for _, b := range stack1 {
        if b == bot[1] && len(stack2) > 0 && stack2[len(stack2) - 1] == bot[0] {
            res += botScore
            stack2 = stack2[:len(stack2) - 1]
        } else {
            stack2 = append(stack2, b)
        }
    }

    return res
}