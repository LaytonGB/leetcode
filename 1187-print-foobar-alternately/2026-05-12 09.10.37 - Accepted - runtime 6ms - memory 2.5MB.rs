// https://leetcode.com/problems/print-foobar-alternately/solutions/7754077/mutex-condvar-alternating-threads-clean-6qozg

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NextPrint {
    Foo,
    Bar,
}

struct FooBar {
    n: usize,
    m: (Mutex<NextPrint>, Condvar),
}

impl FooBar {
    fn new(n: usize) -> Self {
        FooBar {
            n,
            m: (Mutex::new(NextPrint::Foo), Condvar::new()),
        }
    }

    fn foo<F: Fn()>(&self, print_foo: F) {
        let (lock, cvar) = &self.m;
        for _ in 0..self.n {
            let mut next_print = lock.lock().unwrap();
            while *next_print != NextPrint::Foo {
                next_print = cvar.wait(next_print).unwrap();
            }
            
            // printFoo() outputs "foo". Do not change or remove this line.
            print_foo();

            *next_print = NextPrint::Bar;
            cvar.notify_all();
        }
    }

    fn bar<F: Fn()>(&self, print_bar: F) {
        let (lock, cvar) = &self.m;
        for _ in 0..self.n {
            let mut next_print = lock.lock().unwrap();
            while *next_print != NextPrint::Bar {
                next_print = cvar.wait(next_print).unwrap();
            }
            
            // printBar() outputs "bar". Do not change or remove this line.
            print_bar();

            *next_print = NextPrint::Foo;
            cvar.notify_all();
        }
    }
}
