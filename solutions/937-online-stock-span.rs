struct Span {
    price: i32,
    days: i32,
}

struct StockSpanner {
    stack: Vec<Span>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {

    fn new() -> Self {
        Self { stack: Vec::new() }
    }
    
    fn next(&mut self, price: i32) -> i32 {
        let mut days = 1;
        while let Some(span) = self.stack.last() {
            if span.price <= price {
                days += span.days;
                self.stack.pop();
            } else {
                break;
            }
        }
        self.stack.push(Span { price, days });
        days
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */