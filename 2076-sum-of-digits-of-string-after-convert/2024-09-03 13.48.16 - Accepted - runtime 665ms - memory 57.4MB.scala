object Solution {
    def getLucky(s: String, k: Int): Int = {
        var n = s.map((c) => (c - 'a' + 1).toString).mkString
        for (_ <- Range(0, k)) {
            n = n.map((x) => x - '0').sum.toString
        }
        n.toInt
    }
}