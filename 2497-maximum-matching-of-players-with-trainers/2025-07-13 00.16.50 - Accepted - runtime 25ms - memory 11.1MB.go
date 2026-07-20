import (
    "slices"
)

func matchPlayersAndTrainers(players []int, trainers []int) int {
    slices.Sort(players)
    slices.Sort(trainers)

    res := 0
    t := 0
    for _, player := range players {
        for t < len(trainers) && trainers[t] < player {
            t += 1
        }

        if t == len(trainers) {
            break
        }

        res += 1
        t += 1
    }

    return res
}