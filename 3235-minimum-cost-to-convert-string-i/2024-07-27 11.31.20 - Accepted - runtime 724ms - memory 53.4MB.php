class Solution {

    /**
     * @param String $source
     * @param String $target
     * @param String[] $original
     * @param String[] $changed
     * @param Integer[] $cost
     * @return Integer
     */
    function minimumCost($source, $target, $original, $changed, $cost) {
        $map = array();
        foreach (range('a', 'z') as $char) {
            $map[$char . $char] = 0;
        }
        foreach (array_map(null, $original, $changed, $cost) as list($a, $b, $c)) {
            if (!array_key_exists($a . $b, $map) || $map[$a . $b] > $c) {
                $map[$a . $b] = $c;
            }
        }
        // print_r($map);
        
        foreach (range('a', 'z') as $a) {
            foreach (range('a', 'z') as $b) {
                if (array_key_exists($b . $a, $map)) {
                    foreach (range('a', 'z') as $c) {
                        if (array_key_exists($a . $c, $map)
                            && (!array_key_exists($b . $c, $map) || $map[$b . $c] > $map[$b . $a] + $map[$a . $c])
                        ) {
                            // var_dump("updating " . $b . "=>" . $c);
                            $map[$b . $c] = $map[$b . $a] + $map[$a . $c];
                        }
                    }
                }
            }
        }
        // print_r($map);

        $res = 0;
        foreach (array_map(null, str_split($source), str_split($target)) as list($a, $b)) {
            if ($a != $b) {
                if (array_key_exists($a . $b, $map)) {
                    $res += $map[$a . $b];
                } else {
                    return -1;
                }
            }
        }

        return $res;
    }
}