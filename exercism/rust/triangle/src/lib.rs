pub struct Triangle {
    sides: [u64; 3],
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides.iter().any(|&x| x > 0) {
            if [(0, 1, 2), (0, 2, 1), (1, 2, 0)]
                .iter()
                .all(|&(a, b, c)| sides[a] + sides[b] >= sides[c])
            {
                return Some(Triangle { sides });
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    pub fn is_equilateral(&self) -> bool {
        if self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2] {
            return true;
        }
        return false;
    }

    pub fn is_scalene(&self) -> bool {
        if self.sides[0] == self.sides[1]
            || self.sides[0] == self.sides[2]
            || self.sides[1] == self.sides[2]
        {
            return false;
        }
        return true;
    }

    // [(0, 1), (0, 2), (1, 2)].iter().filter(|&&(a, b)| self.sides[a] == self.sides[b]).count() == 2
    pub fn is_isosceles(&self) -> bool {
        if self.sides[0] == self.sides[1]
            || self.sides[0] == self.sides[2]
            || self.sides[1] == self.sides[2]
        {
            return true;
        }
        return false;
    }
}
