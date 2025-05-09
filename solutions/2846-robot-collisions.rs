use std::cmp::Ordering as O;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
}
impl TryFrom<char> for Dir {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone)]
struct Bot {
    pub id: usize,
    pub pos: i32,
    pub hp: i32,
    pub dir: Dir,
}

impl Solution {
    pub fn survived_robots_healths(positions: Vec<i32>, healths: Vec<i32>, directions: String) -> Vec<i32> {
        let n = positions.len();
        let mut bots: Vec<Bot> = positions.into_iter()
            .zip(healths.into_iter())
            .zip(directions.chars())
            .enumerate()
            .map(|(i, ((p, h), d))| Bot {
                id: i,
                pos: p,
                hp: h,
                dir: Dir::try_from(d).unwrap(),
            }).collect();
        bots.sort_unstable_by_key(|b| b.pos);

        let mut res: Vec<Bot> = Vec::with_capacity(n);
        for mut b in bots {
            if b.dir == Dir::Right || res.is_empty() || res[res.len() - 1].dir == Dir::Left {
                res.push(b);
                continue;
            }

            if b.dir == Dir::Left {
                let mut add = true;
                while let Some(last) = res.last_mut() {
                    if last.dir != Dir::Right || !add {
                        break;
                    }
                    
                    match last.hp.cmp(&b.hp) {
                        O::Less => {
                            res.pop();
                            b.hp -= 1;
                        }
                        O::Equal => {
                            res.pop();
                            add = false;
                        }
                        O::Greater => {
                            last.hp -= 1;
                            add = false;
                        }
                    }
                }

                if add {
                    res.push(b);
                }
            }
        }

        res.sort_unstable_by_key(|b| b.id);
        res.into_iter()
            .map(|b| b.hp)
            .collect()
    }
}