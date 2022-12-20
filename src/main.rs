//
// Implementation of a idempotent free rig with two generators, as
// described at
//
// https://mastodon.xyz/@johncarlosbaez@mathstodon.xyz/109544917481142671
//

use std::collections::HashMap;
use std::fmt;

const NUM_RIGS: usize = 4 * 4 * 4 * 4 * 4 * 4 * 4;

// Implementation of a free rig with idempotency and two generators.
#[derive(Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
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
    // The set of rigs can be converted to/from numbers 0..NUM_RIGS.
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

    // Rules for addition and multiplication written out longhand
    // because I'm silly.

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
    // Moreover, 4x y = 4 xy = 2 xy = 2x y, so always normalising down
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

// Implement union-find ourselves, yet again.
#[derive(Debug, Clone, Eq, PartialEq)]
struct RigUnion {
    ptrs: Vec<usize>,
}

impl RigUnion {
    fn new() -> RigUnion {
        // Initially, all pointers point to themselves.
        RigUnion {
            ptrs: (0..NUM_RIGS).collect::<Vec<_>>(),
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
        // Use lowest index as target.
        let tgt = tgt1.min(tgt2);

        // Repoint idx1's chain to target.
        while self.ptrs[idx1] != idx1 {
            let tmp = self.ptrs[idx1];
            self.ptrs[idx1] = tgt;
            idx1 = tmp;
        }
        self.ptrs[idx1] = tgt;
        // Repoint idx2's chain to target.
        while self.ptrs[idx2] != idx2 {
            let tmp = self.ptrs[idx2];
            self.ptrs[idx2] = tgt;
            idx2 = tmp;
        }
        self.ptrs[idx2] = tgt;
    }

    // Break our data structure down into an array of equivalence
    // classes.
    fn get_classes(&mut self) -> Vec<Vec<Rig>> {
        let mut sets: HashMap<usize, Vec<Rig>> = HashMap::new();
        for i in 0..NUM_RIGS {
            let rig = Rig::from(i);
            // Normalise entry
            self.union(&rig, &rig);

            let tgt = self.ptrs[i];
	    sets.entry(tgt).or_insert(Vec::new()).push(rig);
        }
        sets.into_values().collect::<Vec<_>>()
    }
}

fn main() {
    let mut equiv_classes = RigUnion::new();

    // First of all, generate the equivalence classes over rigs and
    // their squares.
    for i in 0..NUM_RIGS {
        // Identify all rigs with their squares.
        let rig = Rig::from(i);
        let rigrig = rig.mul(&rig);
        equiv_classes.union(&rig, &rigrig);
    }

    // And then identify all the results of addition and
    // multiplication - that is, if A and B are equivalence classes,
    // ensure A_i * B_j are all in the same class, and A_i + B_j are
    // also in the same class.

    // I'm not absolutely totally sure one pass does here (I think it
    // does, since union-find should do its magic), so iterate until
    // fixed point, just in case.
    let mut old = RigUnion::new();
    while equiv_classes != old {
        old = equiv_classes.clone();

        // Identify different variants over addition
        for i in 0..NUM_RIGS {
            // Slow enough that you want to run in release mode, and
            // displaying lots of numbers makes it feel like
            // progress. This is inefficient code!
            eprintln!("a{}", i);
            for j in 0..NUM_RIGS {
                let tgti = equiv_classes.ptrs[i];
                let tgtj = equiv_classes.ptrs[j];

                if tgti != i || tgtj != j {
                    let rigi = Rig::from(i);
                    let rigj = Rig::from(j);
                    let rigij = rigi.add(&rigj);

                    let trigi = Rig::from(tgti);
                    let trigj = Rig::from(tgtj);
                    let trigij = trigi.add(&trigj);

                    equiv_classes.union(&rigij, &trigij);
                }
            }
        }

        // Identify different variants over multiplication
        for i in 0..NUM_RIGS {
            eprintln!("m{}", i);
            for j in 0..NUM_RIGS {
                let tgti = equiv_classes.ptrs[i];
                let tgtj = equiv_classes.ptrs[j];

                if tgti != i || tgtj != j {
                    let rigi = Rig::from(i);
                    let rigj = Rig::from(j);
                    let rigij = rigi.mul(&rigj);

                    let trigi = Rig::from(tgti);
                    let trigj = Rig::from(tgtj);
                    let trigij = trigi.mul(&trigj);

                    equiv_classes.union(&rigij, &trigij);
                }
            }
        }
    }

    // Could print out all the equivalence classes...
    if false {
        for (idx, ec) in equiv_classes.get_classes().iter().enumerate() {
            print!("\n{}: ", idx);
            for elt in ec.iter() {
                print!("{}, ", elt);
            }
        }
    }

    // But let's just print out class sizes and # classes:
    let mut classes = equiv_classes
        .get_classes()
        .iter()
        .map(|x| x.len())
        .collect::<Vec<usize>>();
    classes.sort();
    classes.reverse();
    println!("Class sizes: {:?}", &classes);
    println!("\nTotal number of elements: {}", classes.len());
}
