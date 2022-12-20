//
// Implementation of a free rig with two generators, as described at
//
// https://mastodon.xyz/@johncarlosbaez@mathstodon.xyz/109544917481142671
//

use std::collections::HashMap;
use std::fmt;

const NUM_RIGS: usize = 4 * 4 * 4 * 4 * 4 * 4 * 4;

#[derive(Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd)]
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
    fn from(i: usize) -> Rig {
        Rig {
            i: i & 3,
            a: (i >> 2) & 3,
            b: (i >> 4) & 3,
            ab: (i >> 6) & 3,
            ba: (i >> 8) & 3,
            aba: (i >> 10) & 3,
            bab: (i >> 12) & 3,
        }
    }

    fn to_int(&self) -> usize {
        self.i
            + (self.a << 2)
            + (self.b << 4)
            + (self.ab << 6)
            + (self.ba << 8)
            + (self.aba << 10)
            + (self.bab << 12)
    }

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

    // Rules written out by hand because I'm silly.
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
        .normalise()
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
        .normalise()
    }

    // Due to idempotency, x + x = (x + x) * (x + x) = x + x + x + x,
    // so we can always reduce 4x to 2x.
    //
    // Moreover, 4x y = 4 xy = 2 xy = 2x y, so normalising down
    // doesn't change the "reachable" elements.
    fn normalise(&self) -> Rig {
        fn n(i: usize) -> usize {
            if i >= 4 {
                i % 2 + 2
            } else {
                i
            }
        }
        Rig {
            i: n(self.i),
            a: n(self.a),
            b: n(self.b),
            ab: n(self.ab),
            ba: n(self.ba),
            aba: n(self.aba),
            bab: n(self.bab),
        }
    }
}

// Combine pairs of existing elements, and add to the returned vector
// if they've not been seen before.
/*
fn find_new_elements(rigs: &[Rig]) -> Vec<Rig> {
    let mut seen_rigs = rigs.iter().cloned().collect::<HashSet<Rig>>();
    let mut res = Vec::new();
    for r1 in rigs.iter() {
        for r2 in rigs.iter() {
            let r_add = r1.add(r2);
            if !seen_rigs.contains(&r_add) {
                seen_rigs.insert(r_add);
                res.push(r_add);
            }
            let r_mul = r1.mul(r2);
            if !seen_rigs.contains(&r_mul) {
                seen_rigs.insert(r_mul);
                res.push(r_mul);
            }
        }
    }
    res
}
 */

// Implement union-find ourselves, again.
#[derive(Debug, Clone, Eq, PartialEq)]
struct RigUnion {
    ptrs: Vec<usize>,
}

impl RigUnion {
    fn new() -> RigUnion {
	// Initially, all pointers point to themselves.
	RigUnion {
	    ptrs: (0 .. NUM_RIGS).collect::<Vec<_>>(),
	}
    }

    fn union(&mut self, r1: &Rig, r2: &Rig) {
	// Not efficient, just get it done.
	let mut idx1 = r1.to_int();
	let mut idx2 = r2.to_int();

	// Dereference idx1's chain.
	let mut tgt1 = idx1;
	while self.ptrs[tgt1] != tgt1 {
	    assert!(self.ptrs[tgt1] < tgt1);
	    tgt1 = self.ptrs[tgt1];
	}
	// Dereference idx2's chain.
	let mut tgt2 = idx2;
	while self.ptrs[tgt2] != tgt2 {
	    assert!(self.ptrs[tgt2] < tgt2);
	    tgt2 = self.ptrs[tgt2];
	}
	let tgt = tgt1.min(tgt2);

	// Repoint idx1's chain.
	while self.ptrs[idx1] != idx1 {
	    let tmp = self.ptrs[idx1];
	    self.ptrs[idx1] = tgt;
	    idx1 = tmp;
	}
	self.ptrs[idx1] = tgt;
	// Repoint idx2's chain.
	while self.ptrs[idx2] != idx2 {
	    let tmp = self.ptrs[idx2];
	    self.ptrs[idx2] = tgt;
	    idx2 = tmp;
	}
	self.ptrs[idx2] = tgt;
    }

    fn to_classes(&mut self) -> Vec<Vec<Rig>> {
	let mut sets: HashMap<usize, Vec<Rig>> = HashMap::new();
	for i in 0..NUM_RIGS {
	    let rig = Rig::from(i);
	    // Normalise entry
	    self.union(&rig, &rig);
	    
	    let tgt = self.ptrs[i];
	    if !sets.contains_key(&tgt) {
		sets.insert(tgt, vec![rig]);
	    } else {
		sets.get_mut(&tgt).unwrap().push(rig);
	    }
	}
	sets.into_values().collect::<Vec<_>>()
    }
}

fn main() {
    let mut equiv_classes = RigUnion::new();

    for i in 0..NUM_RIGS {
	// Identify all rigs with their squares.
	let rig = Rig::from(i);
	let rigrig = rig.mul(&rig);
	equiv_classes.union(&rig, &rigrig);
    }

    for (idx, ec) in equiv_classes.to_classes().iter().enumerate() {
	print!("\n{}: ", idx);
	for elt in ec.iter() {
	    print!("{}, ", elt);
	}
    }
    
    /*
        let equiv_classes: Vec<Vec<Rig>> = Rig::basis().iter().map(|x| vec![*x]).collect();

        for ec in equiv_classes.iter() {
        for elt in ec.iter() {
            print!("{}", &elt);
        }
        println!("");
    }
         */
/*    let new_elts = find_new_elements(&Rig::basis());
    for elt in new_elts.iter() {
        println!("{}", &elt);
    }
*/
    /*
        in Rig::basis() {
            for r2 in Rig::basis() {
                println!("{} * {} = {}", r1, r2, r1.mul(&r2));
                println!("{} + {} = {}", r1, r2, r1.add(&r2));
            }
    }
        */
}
