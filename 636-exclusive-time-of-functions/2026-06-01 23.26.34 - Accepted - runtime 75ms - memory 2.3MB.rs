use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Op {
    Start,
    End,
}

impl FromStr for Op {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            _ => Err(()),
        }
    }
}

impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut stack = vec![];
        let mut res = vec![0; n as usize];
        let mut curr = None;
        let mut last_time = 0;
        logs.into_iter().for_each(|l| {
            println!("{:?}", res);
            let mut parts = l.split(':');
            let (id, op, time) = (
                parts.next().unwrap().parse::<usize>().unwrap(),
                parts.next().unwrap().parse::<Op>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            );
            match op {
                Op::Start => {
                    if let Some(curr_id) = curr {
                        res[curr_id] += time - last_time - 1;
                        stack.push(curr_id);
                    };

                    curr = Some(id);
                    last_time = time;
                }
                Op::End => {
                    if let Some(curr_id) = curr {
                        res[curr_id] += time - last_time + 1;
                    };
                    last_time = time;
                    curr = stack.pop();
                }
            }
        });
        res
    }
}