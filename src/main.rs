use std::fmt::Debug;

#[derive(Debug)]
struct Personnage {
    nom : String,
    age : u8,
    level : u8,
    vie : u32,
    force : u32,
    energie : u32,
    etat : String
}

enum MonErreur {
    Indisponible,
    Autre(String),
}

impl Drop for Personnage {
    fn drop(&mut self) {
        println!("Le personnage est mort");
    }
}

impl Personnage {
    fn new (nom: String) -> Personnage
    {
        Personnage {
            nom : nom,
            age : 10,
            level : 1,
            vie : 10,
            force : 10,
            energie : 10,
            etat : String::from("vivant")
        }
    }

    fn rename (&mut self, name: String) -> &mut Personnage
    {
        self.nom = name;
        return self;
    }
    
    fn levelUp (&mut self) -> &mut Personnage
    {
        self.level += 1;
        return self;
    }

    fn die (&mut self)
    {
        drop(self);
    }
}

fn main() 
{
    let personnage: Personnage = Personnage::new(String::from("Quentin"));
    {
        let mut personnage = personnage;
        println!("{:?}", personnage);
        personnage.levelUp();
        println!("{:?}", personnage);
        personnage.die();
    }    
}
