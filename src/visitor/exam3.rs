#[derive(Debug, PartialEq)]
pub enum Number {
    Int(Int),
    Float(Float),
}

impl Number {
    pub fn as_f64(&self) -> f64 {
        use Number::*;
        match self {
            Int(int) => int.0 as f64,
            Float(float) => float.0,
        }
    }

    pub fn accept(&mut self, visitor: &mut impl Visitor) {
        use Number::*;
        match self {
            Int(int) => visitor.visit_int(int),
            Float(float) => visitor.visit_float(float),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Int(pub i32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Float(pub f64);

pub trait Visitor {
    fn visit_int(&mut self, int: &mut Int);
    fn visit_float(&mut self, float: &mut Float);
}

pub struct Increaser {
    step: u16,
}

impl Increaser {
    pub fn new(step: u16) -> Self {
        Increaser { step }
    }
}

impl Visitor for Increaser {
    fn visit_int(&mut self, int: &mut Int) {
        int.0 += self.step as i32;
    }

    fn visit_float(&mut self, float: &mut Float) {
        float.0 += self.step as f64;
    }
}

pub struct Decreaser {
    step: u16,
}

impl Decreaser {
    pub fn new(step: u16) -> Self {
        Decreaser { step }
    }
}

impl Visitor for Decreaser {
    fn visit_int(&mut self, int: &mut Int) {
        int.0 -= self.step as i32;
    }

    fn visit_float(&mut self, float: &mut Float) {
        float.0 -= self.step as f64;
    }
}

pub struct Accumulator<'a, T> {
    acc: T,
    op: Box<dyn Fn(&mut T, &Number) + 'a>,
}

impl<'a, T> Accumulator<'a, T> {
    pub fn new<F>(init: T, op: F) -> Self
    where
        F: Fn(&mut T, &Number) + 'a,
    {
        Accumulator {
            acc: init,
            op: Box::new(op),
        }
    }

    pub fn result(&self) -> &T {
        &self.acc
    }
}

impl<'a, T> Visitor for Accumulator<'a, T> {
    fn visit_int(&mut self, int: &mut Int) {
        (self.op)(&mut self.acc, &Number::Int(*int));
    }

    fn visit_float(&mut self, float: &mut Float) {
        (self.op)(&mut self.acc, &Number::Float(*float));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visitor_exam3() {
        let mut numbers: Vec<Number> = vec![
            Number::Int(Int(1)),
            Number::Int(Int(2)),
            Number::Float(Float(3.5)),
        ];

        let mut increaser = Increaser::new(2);
        for n in numbers.iter_mut() {
            n.accept(&mut increaser);
        }
        let expected: Vec<Number> = vec![
            Number::Int(Int(3)),
            Number::Int(Int(4)),
            Number::Float(Float(5.5)),
        ];
        assert_eq!(numbers, expected);

        let mut decreaser = Decreaser::new(1);
        for n in numbers.iter_mut() {
            n.accept(&mut decreaser);
        }
        let expected: Vec<Number> = vec![
            Number::Int(Int(2)),
            Number::Int(Int(3)),
            Number::Float(Float(4.5)),
        ];
        assert_eq!(numbers, expected);

        let mut accumulator = Accumulator::new(0.0, |acc, n| *acc += n.as_f64());
        for n in numbers.iter_mut() {
            n.accept(&mut accumulator);
        }
        assert_eq!(accumulator.result(), &9.5);
    }
}