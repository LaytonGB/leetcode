import (
	"container/heap"
	"fmt"
)

const INT_MAX int = int(^uint(0) >> 1)

type IntHeap [][2]int

func (h IntHeap) Len() int           { return len(h) }
func (h IntHeap) Less(i, j int) bool { return h[i][1] < h[j][1] }
func (h IntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *IntHeap) Push(x any) {
	*h = append(*h, x.([2]int))
}

func (h *IntHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func minCost(n int, edges [][]int) int {
    m := initMap(n)
    for _, e := range edges {
        a, b, dist := e[0], e[1], e[2]
        m[a] = append(m[a], [2]int{b, dist})
        m[b] = append(m[b], [2]int{a, dist * 2})
    }

    fmt.Println(m)

    // Dijkstras
    vis := make([]bool, n)
    dist := initDist(n)
    h := &IntHeap{[2]int{0, 0}}
    heap.Init(h)
    for h.Len() > 0 {
        data := heap.Pop(h).([2]int)
        a, d1 := data[0], data[1]

        if a == n - 1 {
            return d1
        }

        if vis[a] {
            continue
        }
        vis[a] = true

        for _, data := range m[a] {
            b, d2 := data[0], data[1]
            d3 := d1 + d2
            if vis[b] || d3 >= dist[b] {
                continue
            }

            dist[b] = d3
            heap.Push(h, [2]int{b, d3})
        }
    }

    return -1
}

func initMap(n int) [][][2]int {
    m := make([][][2]int, n)
    for i := range n {
        m[i] = [][2]int{}
    }
    return m
}

func initDist(n int) []int {
    dist := make([]int, n)
    for i := range n {
        dist[i] = INT_MAX
    }
    dist[0] = 0
    return dist
}
