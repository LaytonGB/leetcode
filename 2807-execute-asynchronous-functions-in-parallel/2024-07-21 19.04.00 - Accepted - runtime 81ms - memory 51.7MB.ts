type Fn<T> = () => Promise<T>

function promiseAll<T>(functions: Fn<T>[]): Promise<T[]> {
    return new Promise((resolve, reject) => {
        const results: T[] = Array.from({length: functions.length});
        let n = 0;
        functions.forEach((f, i) => {
            f()
            .then(res => {
                results[i] = res;
                n++;
                if (n == functions.length) {
                    resolve(results);
                }
            })
            .catch(err => reject(err));
        });
    });
};

/**
 * const promise = promiseAll([() => new Promise(res => res(42))])
 * promise.then(console.log); // [42]
 */