#[derive(Default, Debug, PartialEq, Clone)]
pub struct Mat3x3 (pub [V3;3]);

impl Mat3x3 {
    pub fn prod(&self, m: &Self) -> Self {
        let mut r = Self::default();
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    r.0[i].0[j] += self.0[i].0[k] * m.0[k].0[j];
                }
            }
        }
        r
    }

    pub fn prod_vec(&self, v: &V3) -> V3 {
        let mut r = V3::default();
        for i in 0..3 {
            for j in 0..3 {
                r.0[i] += self.0[i].0[j] * v.0[j];
            }
        }
        r
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct V3 (pub [f64;3]);

impl V3 {
    fn dot(&self, v: &Self) -> f64 {
        self.0[0] * v.0[0] + self.0[1] * v.0[1] + self.0[2] * v.0[2]
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }
    pub fn y(&self) -> f64 {
        self.0[1]
    }
    pub fn z(&self) -> f64 {
        self.0[2]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_prod() {
        let x = Mat3x3([
            V3([1., 2., 3.]),
            V3([4., 5., 6.]),
            V3([7., 8., 9.]),
        ]);
        let ones = Mat3x3 ([
            V3([1., 1., 1.]),
            V3([1., 1., 1.]),
            V3([1., 1., 1.]),
        ]);
        let r1 = Mat3x3([
            V3([30., 36., 42.]),
            V3([66., 81., 96.]),
            V3([102., 126., 150.]),
        ]);
        let r2 = Mat3x3([
            V3([12., 15., 18.]),
            V3([12., 15., 18.]),
            V3([12., 15., 18.]),
        ]);
        let v1 = V3([1., 2., 3.]);
        let v2 = V3([14., 32., 50.]);
        assert_eq!(x.prod(&x), r1);
        assert_eq!(ones.prod(&x), r2);
        assert_eq!(x.prod_vec(&v1), v2);
    }
}
