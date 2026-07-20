function canChange(start: string, target: string): boolean {
    let i = 0,
        j = 0
    
    while (i < start.length && j < target.length) {
        if (start[i] === '_') {
            i += 1
            continue
        }
        if (target[j] === '_') {
            j += 1
            continue
        }

        if (
            start[i] !== target[j]
            || start[i] === 'R' && i > j
            || start[i] === 'L' && i < j
        ) {
            return false
        }

        i += 1
        j += 1
    }

    return i === start.length && ![...target.slice(j)].some(c => c !== '_')
        || j === target.length && ![...start.slice(i)].some(c => c !== '_')
};