struct BrowserHistory {
    pages: Vec<String>,
    current: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BrowserHistory {

    fn new(homepage: String) -> Self {
        BrowserHistory {
            pages: vec![homepage],
            current: 0,
        }
    }
    
    fn visit(&mut self, url: String) {
        self.pages.truncate(self.current + 1);
        self.pages.push(url);
        self.current += 1;
    }
    
    fn back(&mut self, steps: i32) -> String {
        if steps as usize > self.current {
            self.current = 0;
        } else {
            self.current -= steps as usize;
        }
        self.pages[self.current].clone()
    }
    
    fn forward(&mut self, steps: i32) -> String {
        if self.current + steps as usize >= self.pages.len() {
            self.current = self.pages.len() - 1;
        } else {
            self.current += steps as usize;
        }
        self.pages[self.current].clone()
    }
}

/**
 * Your BrowserHistory object will be instantiated and called as such:
 * let obj = BrowserHistory::new(homepage);
 * obj.visit(url);
 * let ret_2: String = obj.back(steps);
 * let ret_3: String = obj.forward(steps);
 */