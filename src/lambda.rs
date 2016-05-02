use std::collections::HashSet;
use std::collections::LinkedList;
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


// Based on the implementation in Lennart Augustsson's blog post "Simpler, Easier"
// There are about 1 million things that could be improved on, here, but it is working
// to some degree.
pub type Sym = String;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Lambda {
    Var(Sym),
    Abs(Sym, Box<Lambda>),
    App(Box<Lambda>, Box<Lambda>)
}

pub use self::Lambda::*;

impl Lambda {
    fn spine_whnf(self, xs:LinkedList<Lambda>) -> Lambda {
        match self {
            App(f,  a) => {
                let mut ys = xs.clone();
                ys.push_front(*a.clone());
                f.spine_whnf(ys)
            },
            Abs(s, e) => {
                let a = xs.front();
                match a {
                    None => xs.iter().fold(Abs(s.clone(), e.clone()), |sum, i| app_ref(sum, i)),
                    Some(a1) => {
                        let n = subst(&s, a1, e.as_ref());
                        let mut ys = xs.clone();
                        ys.pop_front(); // mutation is gross, but when in Rome...
                        n.spine_whnf(ys)
                    }
                }

            },
            f => xs.iter().fold(f, |sum, i| app_ref(sum, i))
        }
    }

    pub fn whnf(self) -> Lambda {
        self.spine_whnf(LinkedList::new())
    }

    fn spine_nf(self, xs:LinkedList<Lambda>) -> Lambda {
        match self {
            App(f,  a) => {
                let mut ys = xs.clone();
                ys.push_front(*a.clone());
                f.spine_nf(ys)
            },
            Abs(s, e) => {
                let a = xs.front();
                match a {
                    None => Abs(s, Box::new(e.nf())),
                    Some(a1) => {
                        let n = subst(&s, a1, e.as_ref());
                        let mut ys = xs.clone();
                        ys.pop_front();
                        n.spine_nf(ys)
                    }
                }

            },
            f => xs.iter().map(|x| x.clone().nf()).fold(f, |sum, i| app_ref(sum, &i))
        }
    }

    // Normal order reduction, which is like lazy evaluation without sharing
    pub fn nf(self) -> Lambda {
        self.spine_nf(LinkedList::new())
    }

    pub fn alpha_eq(&self, b: &Lambda) -> bool {
        match (self, b) {
            (&Var(ref v),       &Var(ref v1))         => v == v1,
            (&App(ref f,ref a), &App(ref f1, ref a1)) => f.alpha_eq(f1) && a.alpha_eq(a1),
            (&Abs(ref s,ref e), &Abs(ref s1, ref e1)) => e.alpha_eq(&subst_var(s1, s, e1)),
            _ => false
        }
    }

    pub fn beta_eq(&self, b: &Lambda) -> bool {
        self.clone().nf().alpha_eq(&b.clone().nf())
    }

    // TODO This should use shared ownership?
    pub fn free_vars(self) -> HashSet<Sym, RandomState> {
        match self {
            Var(sym) => set!{sym}, // without set! macro: [sym].iter().cloned().collect(),
            Abs(sym, lam) => {
                lam.free_vars().difference(&set!{sym}).cloned().collect()
            },
            App(lam1, lam2) => {
                lam1.free_vars().union(&lam2.free_vars()).cloned().collect()
            }
        }
    }

}

// TODO Is there a better way to do nice enum constructors?
pub fn app(l1: &Lambda, l2: &Lambda) -> Lambda {
    App(Box::new(l1.clone()), Box::new(l2.clone()))
}
pub fn app_ref(l1: Lambda, l2: &Lambda) -> Lambda {
    App(Box::new(l1), Box::new(l2.clone()))
}
pub fn abs(s: &str, l: Lambda) -> Lambda {
    Abs(s.to_string(), Box::new(l))
}
pub fn var(s: &str) -> Lambda {
    Var(s.to_string())
}

pub fn clone_sym(e: &Lambda, i: &Sym, vars: &HashSet<Sym, RandomState>) -> Sym {
    if vars.contains(i) {
        clone_sym(e, &((i.clone()) + "'"), vars)
    } else {
        i.clone()
    }
}

pub fn subst_var(s1: &Sym, s2: &Sym, e: &Lambda) -> Lambda {
    subst(s1, &Var(s2.clone()), e)
}

// replaces all free occurrences of v by x inside b
// b[v:=x]
pub fn subst(v: &Sym, x: &Lambda, b: &Lambda) -> Lambda {
    let fvx = x.clone().free_vars(); // TODO how to reference please
    let sub = |expr: &Lambda| {
        match *expr {
            Var(ref i) => {
                if *i == *v {
                    x.clone()
                } else {
                    expr.clone()
                }
            },
            App(ref f, ref a) => app(&subst(v, x, f), &subst(v, x, a)),
            Abs(ref i, ref e) => {
                if v == i {
                    Abs(i.clone(), e.clone())
                } else if fvx.contains(i) {
                    let i2 = clone_sym(e, i, &fvx);
                    let e2 = subst_var(i, &i2, e);
                    Abs(i2.clone(), Box::new(subst(v, x, &e2)))
                } else {
                    Abs(i.clone(), Box::new(subst(v, x, e)))
                }
            }
        }
    };
    sub(b)
}

#[cfg(test)]
mod tests {
    //use super::Lambda::*;
    use super::*;
    #[test]
    fn it_works() {
        let l1 = var("x");
        assert_eq!(l1, l1);

        let l2 = Abs("x".to_string(), Box::new(l1));
        println!("{:?}", l2);
    }

    #[test]
    fn test_free_vars() {
        let l1 = var("x");
        assert_eq!(l1.free_vars(), ["x".to_string()].iter().cloned().collect());

        let l2 = var("x");
        assert_eq!(l2.free_vars() == ["y".to_string()].iter().cloned().collect(), false);

        let l3 = Abs("y".to_string(), Box::new(var("x")));
        let l3free = l3.free_vars();
        println!("{:?}", l3free);
        assert_eq!(l3free == ["x".to_string()].iter().cloned().collect(), true);
    }
    #[test]
    fn test_clone_sym() {
        let x = var("x");
        let vars1 = ["x".to_string()].iter().cloned().collect();
        println!("clone_sym: {:?}",clone_sym(&x, &"x".to_string(), &vars1))
        // let y = var("y");
        // let l1 = abs("x", var("x"));
        // // subst(var(x), x, y) -> var(y)
        // assert_eq!(subst(&"x".to_string(), &x, &y), var("y"));
        // println!("{:?}", subst(&"x".to_string(), &y, &l1));
        // assert_eq!(subst(&"x".to_string(), &y, &l1), abs("x", var("x")));
        // assert_eq!(subst(&"x".to_string(), &y, &app(var("x"), var("z"))), app(var("y"), var("z")));
    }

    #[test]
    fn test_subst() {
        let x = var("x");
        let y = var("y");
        let l1 = abs("x", var("x"));
        // subst(var(x), x, y) -> var(y)
        assert_eq!(subst(&"x".to_string(), &x, &y), var("y"));
        println!("{:?}", subst(&"x".to_string(), &y, &l1));
        assert_eq!(subst(&"x".to_string(), &y, &l1), abs("x", var("x")));
        assert_eq!(subst(&"x".to_string(), &y, &app(&var("x"), &var("z"))), app(&var("y"), &var("z")));
    }

    #[test]
    fn test_whnf() {
        let id = abs("x", var("x"));
        let apply_id = app(&id.clone(), &var("oi"));
        println!("whnf? {:?}", app(&id, &var("oi")).whnf());
        assert_eq!(apply_id.whnf(), var("oi"));

        let (z,s,m,n) = (&var("z"), &var("s"), &var("m"), &var("n"));
        fn app2(f:&Lambda, x:&Lambda, y:&Lambda) -> Lambda {
            app(&app(f,x), y)
        }
        //let app2 = |f, x, y| app(&app(f,x), y);

        // Some Church encoded arithmetic
        let zero  = abs("s", abs("z", z.clone()));
        let one   = abs("s", abs("z", app(s, z)));
        let two   = abs("s", abs("z", app(s, &app(s, z))));
        let three = abs("s", abs("z", app(s, &app(s, &app(s, z)))));
        let plus  = abs("m", abs("n", abs("s", abs("z", app2(m, s, &app2(n, s, z))))));

        // 1 + 2 = 3, hooray!
        assert!(app2(&plus, &one, &two).beta_eq(&three));
    }
    #[test]
    fn test_nf() {
        let id = abs("x", var("x")); // \x -> x
        let apply_id = app(&id.clone(), &var("oi")); //(\x -> x)(oi) = oi
        println!("nf? {:?}", app(&id, &var("oi")).nf());
        assert_eq!(apply_id.nf(), var("oi"));
    }
}
