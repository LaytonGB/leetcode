func makeFancyString(s string) string {
	res := strings.Builder{}
	res.WriteByte(s[0])

	repeatCount := 1

	for i := 1; i < len(s); i++ {
		if s[i] == s[i-1] && repeatCount >= 2 {
			repeatCount++
		} else if s[i] == s[i-1] {
			res.WriteByte(s[i])
			repeatCount++
		} else {
			res.WriteByte(s[i])
			repeatCount = 1
		}
	}

	return res.String()
}