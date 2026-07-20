use std::sync::Condvar;

#[derive(Debug, PartialEq, Eq)]
enum State {
    ZeroToEven,
    Even,
    ZeroToOdd,
    Odd,
    Done,
}

impl State {
    pub fn is_zero(&self) -> bool {
        match self {
            Self::ZeroToEven | Self::ZeroToOdd => true,
            _ => false,
        }
    }

    pub fn get_next(&self) -> Self {
        match self {
            Self::ZeroToEven => Self::Even,
            Self::Even => Self::ZeroToOdd,
            Self::ZeroToOdd => Self::Odd,
            Self::Odd => Self::ZeroToEven,
            _ => panic!(),
        }
    }
}

struct ZeroEvenOdd {
    n: i32,
    m: (Mutex<(State, i32)>, Condvar),
}

impl ZeroEvenOdd {
    fn new(n: i32) -> Self {
        ZeroEvenOdd {
            n,
            m: (Mutex::new((State::ZeroToOdd, 1)), Condvar::new()),
        }
    }

    fn zero<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        let (lock, cvar) = &self.m;
        let mut current = lock.lock().unwrap();

        loop {
            while !current.0.is_zero() && current.0 != State::Done {
                current = cvar.wait(current).unwrap();
            }
            if current.0 == State::Done {
                return;
            }

            print_number(0);

            *current = (current.0.get_next(), current.1);
            cvar.notify_all();
        }
    }

    fn even<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        let (lock, cvar) = &self.m;
        let mut current = lock.lock().unwrap();
        loop {
            while !matches!(current.0, State::Even | State::Done) {
                current = cvar.wait(current).unwrap();
            }
            if current.0 == State::Done {
                return;
            }
            
            print_number(current.1);

            if current.1 == self.n {
                *current = (State::Done, current.1);
            } else {
                *current = (current.0.get_next(), current.1 + 1);
            }

            cvar.notify_all();
        }
    }

    fn odd<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        let (lock, cvar) = &self.m;
        let mut current = lock.lock().unwrap();
        loop {
            while !matches!(current.0, State::Odd | State::Done) {
                current = cvar.wait(current).unwrap();
            }
            if current.0 == State::Done {
                return;
            }
            
            print_number(current.1);

            if current.1 == self.n {
                *current = (State::Done, current.1);
            } else {
                *current = (current.0.get_next(), current.1 + 1);
            }

            cvar.notify_all();
        }
    }
}