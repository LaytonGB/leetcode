class Solution {

    /**
     * @param Integer $x
     * @return Integer
     */
    function reverse($x) {
        $n = (int) floor(log10(abs($x)));
        for ($i = 0; $i <= intdiv($n, 2); $i++) {
            $p = pow(10, $i);
            $q = pow(10, $n - $i);
            $y = ($x / $p) % 10;
            $z = ($x / $q) % 10;
            $x += $y * $q + $z * $p - $y * $p - $z * $q;
        }

        if ($x > 2147483647 || $x < -2147483648) {
            return 0;
        }

        return $x;
    }
}