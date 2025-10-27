struct Node {
    valeur : u32;
    next : Box<Option<Node>>;
  }

struct LinkedList {
        tete : Box<Option<Node>>;
        taile : u32;

}

impl LinkedList {
    fn new(&self) -> Self{
        Self { tete : Box::new(Some(None)), taille : 0}
    }

    fn est_vide(&self) -> bool {
        self.taille == 0;
    }

    fn ajouter_tete(&mut self, elt : u32){
        
    } 
}
