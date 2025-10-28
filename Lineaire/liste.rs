use std::fmt;

struct Node {
    valeur : u32,
    next : Box<Option<Node>>
  }

struct LinkedList {
        tete : Box<Option<Node>>,
        taille : u32

}

impl LinkedList {
    fn new() -> Self{
        Self { tete : Box::new(None), taille : 0}
    }

    fn est_vide(&self) -> bool {
        self.taille == 0
    }

    fn ajouter_tete(&mut self, elt : u32){
        let new_node : Node = Node {valeur : elt, next : Box::new(self.tete.take())};
        self.tete = Box::new(Some(new_node));
        self.taille += 1;
    } 

    fn len(&self) -> u32 {
        self.taille
    }

    fn pop(&mut self, i: usize) -> Option<u32> {
        if self.est_vide() || i >= self.taille.try_into().unwrap() {
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
        let mut current = self.tete.as_mut()?;
        for _ in 0..i - 1 {
            current = current.next.as_mut()?;  // emprunt mutable du next
        }

        // current pointe maintenant sur le nœud avant celui à supprimer
        let mut target = current.next.take()?; // on "retire" le next du chainage
        current.next = target.next.take();     // on relie le suivant du suivant

        Some(target.valeur)
    }

}
impl fmt::Display for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut texte = String::new();
        let mut current = self.tete.as_ref();

        while let Some(noeud) = current {
            texte.push_str(&format!("{} -> ", noeud.valeur));
            current = noeud.next.as_ref();
        }
        texte.push_str("_|_");

        write!(f, "{}", texte)

    }


}

fn main(){
    let mut liste = LinkedList::new();
    liste.ajouter_tete(5);
    liste.ajouter_tete(42);
    liste.ajouter_tete(420);
    println!("{}", liste);
}
