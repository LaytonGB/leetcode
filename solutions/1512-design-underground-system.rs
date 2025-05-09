use std::collections::*;

struct UndergroundSystem {
    avg_times: HashMap<(String, String), (f64, u64)>,
    check_ins: HashMap<i32, (i32, String)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {

    fn new() -> Self {
        Self {
            avg_times: HashMap::new(),
            check_ins: HashMap::new(),
        }
    }
    
    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.check_ins.insert(id, (t, station_name));
    }
    
    fn check_out(&mut self, id: i32, name2: String, t2: i32) {
        if let Some((t1, name1)) = self.check_ins.remove(&id) {
            let e = self.avg_times.entry((name1, name2)).or_insert((0., 0));
            let (avg, count) = *e;
            let new_count = count + 1;
            let new_avg = count as f64 / new_count as f64 * avg + (t2 - t1) as f64 / new_count as f64;
            *e = (new_avg, new_count);
        } else {
            unreachable!();
        }
    }
    
    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        self.avg_times.get(&(start_station, end_station)).expect("Entry did not exist.").0
    }
}

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */