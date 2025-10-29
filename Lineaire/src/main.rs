mod liste;
mod pile;
use liste::LinkedList;
use pile::Pile;

fn main() {
    let mut liste = LinkedList::new();
    liste.ajouter_tete(5);
    liste.ajouter_tete(42);
    liste.ajouter_tete(420);
    println!("{}", liste);

    let mut p = Pile::new();
    p.empiler(50);
    p.empiler(14);
    p.empiler(69);

    println!("La pile est : \n{}", p);
}
