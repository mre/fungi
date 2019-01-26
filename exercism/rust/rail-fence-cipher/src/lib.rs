pub struct RailFence {
    rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        return Self {
            rails: rails as usize,
        };
    }

    pub fn encode(&self, text: &str) -> String {
        let mut rls: Vec<String> = Vec::with_capacity(self.rails);
        for _ in 0..self.rails {
            rls.push(String::new());
        }

        // returns:
        // std::iter::Cycle<std::iter::Chain<std::slice::Iter<'_, usize>, std::slice::Iter<'_, usize>>>
        // let c = (0..m).into_iter().chain((1..m-1).into_iter().rev()).cycle().take(20).collect::<Vec<usize>>();
        let a: Vec<usize> = (std::ops::Range {
            start: 0,
            end: self.rails,
        })
        .collect::<Vec<usize>>();
        let b: Vec<usize> = (std::ops::Range {
            start: 1,
            end: self.rails - 1,
        })
        .rev()
        .collect::<Vec<usize>>();
        let mut i = a.iter().chain(b.iter()).cycle();

        for c in text.chars() {
            let idx: &usize = i.next().expect("cycle must work");
            rls[*idx].push(c);
        }
        rls.join("")
    }

    pub fn decode(&self, cipher: &str) -> String {
        println!("---");
        println!("{:?}", cipher);
        let mut rls: Vec<String> = Vec::with_capacity(self.rails);
        for _ in 0..self.rails {
            rls.push(String::new());
        }

        // returns:
        // std::iter::Cycle<std::iter::Chain<std::slice::Iter<'_, usize>, std::slice::Iter<'_, usize>>>
        // let c = (0..m).into_iter().chain((1..m-1).into_iter().rev()).cycle().take(20).collect::<Vec<usize>>();
        let a: Vec<usize> = (std::ops::Range {
            start: 0,
            end: self.rails,
        })
        .collect::<Vec<usize>>();
        let b: Vec<usize> = (std::ops::Range {
            start: 1,
            end: self.rails - 1,
        })
        .rev()
        .collect::<Vec<usize>>();
        // std::iter::Cycle<std::iter::Chain<std::slice::Iter<'_, usize>, std::slice::Iter<'_, usize>>>
        let mut i = a.iter().chain(b.iter()).cycle();

        let mut steps: Vec<(usize, usize)> = Vec::with_capacity(cipher.len());
        for (ic, c) in cipher.chars().enumerate() {
            let idx: &usize = i.next().expect("cycle must work");
            rls[*idx].push(c);
            steps.push((ic, *idx));
        }

        let mut start: usize = 0;
        for i in 0..self.rails {
            let end: usize = start + rls[i].len();
            println!(
                "({}): {:?} [{}..{}]",
                end,
                cipher.get(start..end),
                start,
                end
            );
            rls[i] = cipher.get(start..end).expect("broken slice").to_string();
            start = end;
        }

        let mut res: Vec<String> = Vec::with_capacity(self.rails);
        for _ in 0..self.rails {
            res.push(String::new());
        }
        for (_, ridx) in steps {
            res.push(rls[ridx].remove(0).to_string());
        }
        return res.join("");
    }
}
