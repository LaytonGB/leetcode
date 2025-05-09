use std::collections::{ BTreeSet, HashMap };

struct NumberContainers {
    index_to_number: HashMap<i32, i32>,
    number_to_indices: HashMap<i32, BTreeSet<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumberContainers {
    fn new() -> Self {
        Self {
            index_to_number: HashMap::new(),
            number_to_indices: HashMap::new(),
        }
    }
    
    fn change(&mut self, index: i32, number: i32) {
        self.index_to_number
            .entry(index)
            .and_modify(|old_num| *old_num = number)
            .or_insert(number);
        
        self.number_to_indices
            .entry(number)
            .and_modify(|indices| { indices.insert(index); })
            .or_insert(BTreeSet::from([index]));
    }
    
    fn find(&mut self, number: i32) -> i32 {
        let Some(indices) = self.number_to_indices.get_mut(&number) else {
            return -1;
        };

        while let Some(index) = indices.first() {
            if self.index_to_number.get(index).is_some_and(|n| *n == number) {
                return *index;
            }

            indices.pop_first();
        }

        return -1;
    }
}

/**
 * Your NumberContainers object will be instantiated and called as such:
 * let obj = NumberContainers::new();
 * obj.change(index, number);
 * let ret_2: i32 = obj.find(number);
 */