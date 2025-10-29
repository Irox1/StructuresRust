use std::fmt;

pub struct Node {
    pub valeur: u32,
    pub next: Option<Box<Node>>,
}

pub struct LinkedList {
    pub tete: Option<Box<Node>>,
    pub taille: u32,
}

impl LinkedList {
    pub fn new() -> Self {
        Self {
            tete: None,
            taille: 0,
        }
    }

    pub fn est_vide(&self) -> bool {
        self.taille == 0
    }

    pub fn ajouter_tete(&mut self, elt: u32) {
        let new_node = Node {
            valeur: elt,
            next: self.tete.take(),
        };
        self.tete = Some(Box::new(new_node));
        self.taille += 1;
    }

    pub fn len(&self) -> u32 {
        self.taille
    }

    pub fn pop(&mut self, i: usize) -> Option<u32> {
        if self.est_vide() || i >= self.taille as usize {
            return None;
        }

        self.taille -= 1;

        // Cas spécial : suppression de la tête
        if i == 0 {
            let tete = self.tete.take().unwrap();
            self.tete = tete.next;
            return Some(tete.valeur);
        }

        // Parcours jusqu’à l’élément précédent
        let mut current = self.tete.as_mut().unwrap();
        for _ in 0..i - 1 {
            current = current.next.as_mut().unwrap();
        }

        // Supprime le noeud ciblé
        let mut target = current.next.take()?;
        current.next = target.next.take();

        Some(target.valeur)
    }
}

impl fmt::Display for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut texte = String::new();
        let mut current = self.tete.as_deref();

        while let Some(noeud) = current {
            texte.push_str(&format!("{} -> ", noeud.valeur));
            current = noeud.next.as_deref();
        }

        texte.push_str("_|_");
        write!(f, "{}", texte)
    }
}
