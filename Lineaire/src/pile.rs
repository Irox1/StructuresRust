use std::fmt::{self, write};

#[derive(Debug)]
pub struct Pile {
    content: Vec<u32>,
    taille: u32,
}

impl Pile {
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
            taille: 0,
        }
    }

    pub fn from(elts: Vec<u32>) -> Self {
        let taille = elts.len();
        Self {
            content: elts,
            taille: taille as u32,
        }
    }

    pub fn est_vide(&self) -> bool {
        self.taille == 0
    }

    pub fn len(&self) -> u32 {
        self.taille
    }

    pub fn empiler(&mut self, elt: u32) {
        self.content.push(elt);
        self.taille += 1;
    }

    pub fn depiler(&mut self) -> Option<u32> {
        assert!(!self.est_vide(), "La Pile est vide !");
        let tmp: u32 = self.content.pop().unwrap();
        self.taille -= 1;
        Some(tmp)
    }
}

impl fmt::Display for Pile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for val in &self.content {
            write!(f, "{}\n", val)?;
        }

        Ok(())
    }
}
