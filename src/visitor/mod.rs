pub mod exam1;
pub mod exam2;
pub mod exam3;

pub trait Element {
    // NOTE: If a method has generic type parameters or impl Trait types,
    //       the trait `Element` cannot be made into an object.
    // fn accept(&mut self, visitor: &mut impl Visitor);
    fn accept(&mut self, visitor: &mut dyn Visitor);
}

pub struct ElementA;

impl Element for ElementA {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_element_a(self);
    }
}

pub struct ElementB;

impl Element for ElementB {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_element_b(self);
    }
}

pub struct CompositeElement {
    children: Vec<Box<dyn Element>>,
}

impl Element for CompositeElement {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        for child in self.children.iter_mut() {
            child.accept(visitor);
        }
        visitor.visit_composite_element(self);
    }
}

pub trait Visitor {
    fn visit_element_a(&mut self, element: &mut ElementA);
    fn visit_element_b(&mut self, element: &mut ElementB);
    fn visit_composite_element(&mut self, element: &mut CompositeElement);
}

pub struct Visitor1;

impl Visitor for Visitor1 {
    fn visit_element_a(&mut self, _element: &mut ElementA) {
        println!("# Visitor1::visit_element_a");
    }

    fn visit_element_b(&mut self, _element: &mut ElementB) {
        println!("# Visitor1::visit_element_b");
    }

    fn visit_composite_element(&mut self, _element: &mut CompositeElement) {
        println!("# Visitor1::visit_composite_element");
    }
}

pub struct Visitor2;

impl Visitor for Visitor2 {
    fn visit_element_a(&mut self, _element: &mut ElementA) {
        println!("# Visitor2::visit_element_a");
    }

    fn visit_element_b(&mut self, _element: &mut ElementB) {
        println!("# Visitor2::visit_element_b");
    }

    fn visit_composite_element(&mut self, _element: &mut CompositeElement) {
        println!("# Visitor2::visit_composite_element");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visitor() {
        let mut elements: Vec<Box<dyn Element>> = vec![
            Box::new(ElementA),
            Box::new(ElementB),
            Box::new(CompositeElement {
                children: vec![Box::new(ElementA), Box::new(ElementB)],
            }),
        ];

        let mut v1 = Visitor1;
        for e in elements.iter_mut() {
            e.accept(&mut v1);
        }

        let mut v2 = Visitor2;
        for e in elements.iter_mut() {
            e.accept(&mut v2);
        }
    }
}
