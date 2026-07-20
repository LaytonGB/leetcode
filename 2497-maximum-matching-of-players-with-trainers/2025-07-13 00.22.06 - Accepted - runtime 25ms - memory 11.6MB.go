func matchPlayersAndTrainers(players []int, trainers []int) int {
    sort.Ints(players)
    sort.Ints(trainers)

    p, t := 0, 0
    for p < len(players) && t < len(trainers) {
        if players[p] <= trainers[t] {
            p += 1
        }
        t += 1
    }

    return p
}