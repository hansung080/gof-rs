pub mod exam;

use std::result;
use crate::utils::iter::NoneIterator;

pub type Error = &'static str;
pub type Result<T> = result::Result<T, Error>;

pub type Id = u64;

pub trait Component {
    // Basic Operations
    fn id(&self) -> Id;
    fn operation(&self) -> Result<()>;

    // Collection Operations
    fn add(&mut self, #[allow(unused)] child: Box<dyn Component>) -> Result<()> {
        Err("function `Component::add` cannot be applied to a leaf component")
    }

    fn remove(&mut self, #[allow(unused)] child_id: Id) -> Result<()> {
        Err("function `Component::remove` cannot be applied to a leaf component")
    }

    fn children(&self) -> Box<dyn Iterator<Item = &Box<dyn Component + '_>> + '_> {
        Box::new(NoneIterator::new())
    }
}

pub struct Leaf {
    id: Id,
}

impl Leaf {
    pub fn new(id: Id) -> Self {
        Leaf { id }
    }
}

impl Component for Leaf {
    fn id(&self) -> Id {
        self.id
    }

    fn operation(&self) -> Result<()> {
        Ok(())
    }
}

pub struct Composite {
    id: Id,
    children: Vec<Box<dyn Component>>,
}

impl Composite {
    pub fn new(id: Id) -> Self {
        Composite {
            id,
            children: Vec::new(),
        }
    }
}

impl Component for Composite {
    fn id(&self) -> Id {
        self.id
    }

    fn operation(&self) -> Result<()> {
        for child in self.children.iter() {
            child.operation()?
        }
        Ok(())
    }

    fn add(&mut self, child: Box<dyn Component>) -> Result<()> {
        self.children.push(child);
        Ok(())
    }

    fn remove(&mut self, child_id: Id) -> Result<()> {
        let pos = self.children.iter().position(|child| child.id() == child_id);
        match pos {
            Some(pos) =>  {
                self.children.remove(pos);
                Ok(())
            },
            None => Err("child not found for id"),
        }
    }

    fn children(&self) -> Box<dyn Iterator<Item = &Box<dyn Component + '_>> + '_> {
        Box::new(self.children.iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composite() {
        let mut composite1 = Composite::new(1);
        composite1.add(Box::new(Leaf::new(2))).unwrap();

        let mut composite2 = Composite::new(3);
        composite2.add(Box::new(Leaf::new(4))).unwrap();
        composite2.add(Box::new(Leaf::new(5))).unwrap();
        composite2.add(Box::new(composite1)).unwrap();
        assert_eq!(composite2.children().count(), 3);

        composite2.remove(5).unwrap();
        assert_eq!(composite2.children().count(), 2);
    }
}
