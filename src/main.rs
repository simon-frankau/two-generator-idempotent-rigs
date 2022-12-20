//
// Implementation of a free rig with two generators, as described at
//
// https://mastodon.xyz/@johncarlosbaez@mathstodon.xyz/109544917481142671
//

use std::fmt;

#[derive(Eq, PartialEq)]
struct Rig {
    i: usize,
    a: usize,
    b: usize,
    ab: usize,
    ba: usize,
    aba: usize,
    bab: usize,
}

const ZERO: Rig = Rig {
    i: 0,
    a: 0,
    b: 0,
    ab: 0,
    ba: 0,
    aba: 0,
    bab: 0,
};

impl fmt::Display for Rig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if *self == ZERO {
            return write!(f, "0");
        }

        fn elt(acc: &mut Vec<String>, i: usize, s: &str) {
            if i == 1 && !s.is_empty() {
                acc.push(s.to_string());
            } else if i != 0 {
                acc.push(format!("{}{}", i, s));
            }
        }
        let mut v = Vec::new();
        elt(&mut v, self.i, "");
        elt(&mut v, self.a, "a");
        elt(&mut v, self.b, "b");
        elt(&mut v, self.ab, "ab");
        elt(&mut v, self.ba, "ba");
        elt(&mut v, self.aba, "aba");
        elt(&mut v, self.bab, "bab");
        write!(f, "{}", v.join(" + "))
    }
}

impl Rig {
    fn basis() -> Vec<Rig> {
        vec![
            Rig { i: 1, ..ZERO },
            Rig { a: 1, ..ZERO },
            Rig { b: 1, ..ZERO },
            Rig { ab: 1, ..ZERO },
            Rig { ba: 1, ..ZERO },
            Rig { aba: 1, ..ZERO },
            Rig { bab: 1, ..ZERO },
        ]
    }

    fn add(&self, other: &Rig) -> Rig {
        Rig {
            i: self.i + other.i,
            a: self.a + other.a,
            b: self.b + other.b,
            ab: self.ab + other.ab,
            ba: self.ba + other.ba,
            aba: self.aba + other.aba,
            bab: self.bab + other.bab,
        }
    }

    fn mul(&self, other: &Rig) -> Rig {
        Rig {
            i: self.i * other.i,

            a: self.i * other.a + self.a * other.i + self.a * other.a,

            b: self.i * other.b + self.b * other.i + self.b * other.b,

            ab: self.i * other.ab
                + self.ab * other.i
                + self.ab * other.ab
                + self.a * other.b
                + self.a * other.ab
                + self.a * other.bab
                + self.ab * other.b
                + self.ab * other.bab
                + self.aba * other.b
                + self.aba * other.ab
                + self.aba * other.bab,

            ba: self.i * other.ba
                + self.ba * other.i
                + self.ba * other.ba
                + self.b * other.a
                + self.b * other.ba
                + self.b * other.aba
                + self.ba * other.a
                + self.ba * other.aba
                + self.bab * other.a
                + self.bab * other.ba
                + self.bab * other.aba,

            aba: self.i * other.aba
                + self.aba * other.i
                + self.aba * other.aba
                + self.a * other.ba
                + self.a * other.aba
                + self.ab * other.a
                + self.ab * other.ba
                + self.ab * other.aba
                + self.aba * other.a
                + self.aba * other.ba,

            bab: self.i * other.bab
                + self.bab * other.i
                + self.bab * other.bab
                + self.b * other.ab
                + self.b * other.bab
                + self.ba * other.b
                + self.ba * other.ab
                + self.ba * other.bab
                + self.bab * other.b
                + self.bab * other.ab,
        }
    }
}

fn main() {
    for r1 in Rig::basis() {
        for r2 in Rig::basis() {
            println!("{} * {} = {}", r1, r2, r1.mul(&r2));
            println!("{} + {} = {}", r1, r2, r1.add(&r2));
        }
    }
}
