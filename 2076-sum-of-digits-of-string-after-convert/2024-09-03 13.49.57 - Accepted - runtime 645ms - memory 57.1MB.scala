object Solution {
    def getLucky(s: String, k: Int): Int = {
        var n = s.map(_ - 'a' + 1).map(_.toString).mkString
        for (_ <- Range(0, k)) {
            n = n.map((x) => x - '0').sum.toString
        }
        n.toInt
    }
}