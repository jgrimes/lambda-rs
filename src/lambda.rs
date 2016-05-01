use std::collections::HashSet;
use std::collections::hash_map::RandomState;

// Macro for providing literal set syntax
// modification of a literal hashmap macro found on stackoverflow
macro_rules! set(
    { $($key:expr),+ } => {
        {
            let mut m = ::std::collections::HashSet::new();
            $(
                m.insert($key);
            )+
            m
        }
     };
);

pub type Sym = &'static str;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Lambda {
    Var(Sym),
    Abs(Sym, Box<Lambda>),
    App(Box<Lambda>, Box<Lambda>)
}

pub use self::Lambda::*;

impl Lambda {
    pub fn free_vars(self) -> HashSet<Sym, RandomState> {
        match self {
            Lambda::Var(sym) => set!{sym}, // without set! macro: [sym].iter().cloned().collect(),
            Abs(sym, lam) => {
                lam.free_vars().difference(&set!{sym}).cloned().collect()
            },
            App(lam1, lam2) => {
                lam1.free_vars().union(&lam2.free_vars()).cloned().collect()
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Lambda::*;
    #[test]
    fn it_works() {
        let l1 = Var("x");
        assert_eq!(l1, l1);

        let l2 = Abs("x", Box::new(l1));
        println!("{:?}", l2);
    }

    #[test]
    fn test_free_vars() {
        let l1 = Var("x");
        assert_eq!(l1.free_vars(), ["x"].iter().cloned().collect());

        let l2 = Var("x");
        assert_eq!(l2.free_vars() == ["y"].iter().cloned().collect(), false);
    }
}
