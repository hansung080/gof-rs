use std::fmt::Debug;
use dyn_ord::DynEq;

pub trait Number: Debug + DynEq {
    fn as_dyn_eq(&self) -> &dyn DynEq;
    fn as_f64(&self) -> f64;
    fn accept(&mut self, visitor: &mut dyn Visitor);
}

impl PartialEq for dyn Number + '_ {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other.as_dyn_eq())
    }
}

#[derive(Debug, PartialEq)]
pub struct Int(pub i32);

impl Number for Int {
    fn as_dyn_eq(&self) -> &dyn DynEq {
        self
    }

    fn as_f64(&self) -> f64 {
        self.0 as f64
    }

    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_int(self);
    }
}

#[derive(Debug, PartialEq)]
pub struct Float(pub f64);

impl Number for Float {
    fn as_dyn_eq(&self) -> &dyn DynEq {
        self
    }

    fn as_f64(&self) -> f64 {
        self.0
    }

    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_float(self);
    }
}

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
    op: Box<dyn Fn(&mut T, &dyn Number) + 'a>,
}

impl<'a, T> Accumulator<'a, T> {
    pub fn new<F>(init: T, op: F) -> Self
    where
        F: Fn(&mut T, &dyn Number) + 'a,
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
        (self.op)(&mut self.acc, int);
    }

    fn visit_float(&mut self, float: &mut Float) {
        (self.op)(&mut self.acc, float);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visitor_exam2() {
        let mut numbers: Vec<Box<dyn Number>> = vec![
            Box::new(Int(1)),
            Box::new(Int(2)),
            Box::new(Float(3.5)),
        ];

        let mut increaser = Increaser::new(2);
        for n in numbers.iter_mut() {
            n.accept(&mut increaser);
        }
        let expected: Vec<Box<dyn Number>> = vec![
            Box::new(Int(3)),
            Box::new(Int(4)),
            Box::new(Float(5.5)),
        ];
        assert_eq!(numbers, expected);

        let mut decreaser = Decreaser::new(1);
        for n in numbers.iter_mut() {
            n.accept(&mut decreaser);
        }
        let expected: Vec<Box<dyn Number>> = vec![
            Box::new(Int(2)),
            Box::new(Int(3)),
            Box::new(Float(4.5)),
        ];
        assert_eq!(numbers, expected);

        let mut accumulator = Accumulator::new(0.0, |acc, n| *acc += n.as_f64());
        for n in numbers.iter_mut() {
            n.accept(&mut accumulator);
        }
        assert_eq!(accumulator.result(), &9.5);
    }
}