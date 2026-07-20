use std::sync::{Barrier};

struct Foo {
    barrier_one: Barrier,
    barrier_two: Barrier,
}

impl Foo {
    fn new() -> Self {
        Foo {
            barrier_one: Barrier::new(2),
            barrier_two: Barrier::new(2),
        }
    }

    fn first<F>(&self, print_first: F)
    where
        F: FnOnce(),
    {
        // Do not change this line
        print_first();

        self.barrier_one.wait();
    }

    fn second<F>(&self, print_second: F)
    where
        F: FnOnce(),
    {
        self.barrier_one.wait();
        
        // Do not change this line
        print_second();

        self.barrier_two.wait();
    }

    fn third<F>(&self, print_third: F)
    where
        F: FnOnce(),
    {
        self.barrier_two.wait();
        
        // Do not change this line
        print_third();
    }
}