class Solution {

    /**
     * @param Integer[] $rating
     * @return Integer
     */
    function numTeams($rating) {
        $n = sizeof($rating);
        $count = 0;
        for ($i = 1; $i < $n - 1; $i++) {
            $l = array_fill(0, 2, 0);
            $r = array_fill(0, 2, 0);

            for ($j = 0; $j < $i; $j++) {
                $l[(int)($rating[$j] < $rating[$i])]++;
            }

            for ($k = $i + 1; $k < $n; $k++) {
                $r[(int)($rating[$k] < $rating[$i])]++;
            }

            $count += $l[0] * $r[1] + $l[1] * $r[0];
        }

        return $count;
    }
}