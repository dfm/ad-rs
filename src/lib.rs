pub struct Dual<Primal, Tangent> {
    pub primal: Primal,
    pub tangent: Tangent,
}

impl<Primal, Tangent> std::ops::Add<Self> for Dual<Primal, Tangent>
where
    Primal: std::ops::Add<Output = Primal>,
    Tangent: std::ops::Add<Output = Tangent>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            primal: self.primal + rhs.primal,
            tangent: self.tangent + rhs.tangent,
        }
    }
}

impl<Primal, Tangent> std::ops::Add<Primal> for Dual<Primal, Tangent>
where
    Primal: std::ops::Add<Output = Primal>,
{
    type Output = Self;
    fn add(self, rhs: Primal) -> Self {
        Self {
            primal: self.primal + rhs,
            tangent: self.tangent,
        }
    }
}

impl<Primal, Tangent> std::ops::Sub<Self> for Dual<Primal, Tangent>
where
    Primal: std::ops::Sub<Output = Primal>,
    Tangent: std::ops::Sub<Output = Tangent>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            primal: self.primal - rhs.primal,
            tangent: self.tangent - rhs.tangent,
        }
    }
}

impl<Primal, Tangent> std::ops::Sub<Primal> for Dual<Primal, Tangent>
where
    Primal: std::ops::Sub<Output = Primal>,
{
    type Output = Self;
    fn sub(self, rhs: Primal) -> Self {
        Self {
            primal: self.primal - rhs,
            tangent: self.tangent,
        }
    }
}

pub trait DiffOps {
    fn exp(self) -> Self;
}

impl DiffOps for f64 {
    fn exp(self) -> Self {
        f64::exp(self)
    }
}

impl<Primal, Tangent> DiffOps for Dual<Primal, Tangent>
where
    Primal: DiffOps + Copy,
    Tangent: std::ops::Mul<Primal, Output = Tangent>,
{
    fn exp(self) -> Self {
        let primal = self.primal.exp();
        Self {
            primal,
            tangent: self.tangent * primal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let x = Dual {
            primal: 1.0,
            tangent: 1.0,
        };
        let y = 5.0;
        assert_eq!((x + y).primal, 6.0);
    }

    #[test]
    fn test_exp() {
        let x = Dual {
            primal: 5.0,
            tangent: 1.0,
        };
        assert_eq!(x.exp().primal, 5.0.exp());
    }
}
