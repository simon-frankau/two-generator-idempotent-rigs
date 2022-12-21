# Element counter for a free idempotent rig on two generators

Bit of a mouthful, yes? This is something I played around with based
off a series of Mastodon posts by @johncarlosbaez@mathstodon.xyz at
https://mastodon.xyz/@johncarlosbaez@mathstodon.xyz/109544917481142671 .

To quote those posts:

```
Here's a puzzle about rigs I don't know the answer to!

A 'rig' R has a commutative associative addition, an associative multiplication that distributes over addition, an element 0 with r+0 = r and 0r = 0 = r0 for all r ∈ R, and an element 1 with 1r = r = r1 for all r ∈ R.

A rig is 'idempotent' if rr = r for all r ∈ R.

The free idempotent rig on two generators is finite, according to @rogers.  But how many elements does it have?

It has at most 4⁷ elements, but in fact far fewer.

(1/n)
```

```
We can start by taking two elements a and b and multiplying them in all ways subject to associativity and the idempotent law.   We get just 7 elements:

1, a, b, ab, ba, aba, bab

Then we start adding these.   In an idempotent rig r + r + r + r = r + r for any element r, so we get at most 4⁷ elements.

Why is that equation true?  Because 

(1+1)² = 1 + 1

(2/n)
```

```
But the free idempotent rig on two generators has far fewer than 4⁷ elements!  After all, it obeys many more relations, like 

a + ab + ba + b = a + b

(Notice that we can't get from this to ab + ba = 0, since we can't subtract.)

So, how many elements does it have?

By the way, the free idempotent rig on 3 generators is probably infinite, since there are infinitely many 'square-free words' with 3 letters:

https://en.wikipedia.org/wiki/Square-free_word

(3/n, n = 3)
```

This code attempts to answer that question.

## And?

The answer, if I haven't made enough mistakes, is 284.

If you want to know more, I have some prepopulated results:

 * [all_classes.txt](all_classes.txt) lists the elements in
   equivalence classes, one line per class.
 * [lexical_element.txt](lexical_element.txt) selects one element per
   equivalence class, choosing the minimal one according to Rust's
   auto-derived comparison operator, which I believe should give that
   lexicographically minimal element per class.
 * [small_element.txt](small_element.txt) selects a "smallest"
   element, with where the number of non-zero coefficients is zero,
   tie-broken on sum of the coefficients.
 
## How did you approach this?

As John Carlos Baez points out, x + x + x + x = x + x, so we can
represent any element as

```
[0..3] + [0..3]a + [0..3]b + [0..3]ab + [0..3]ba + [0..3]aba + [0..3]bab
```

which is a pleasantly tractable number of elements. All we need to do
is work out which elements are equivalent - i.e. construct equivalent
classes over these sets. To create equivalence classes, I used a very
basic union-find algorithm.

There were three sources of equivalence classes:

 * If X * X = Y, X and Y go in the same equivalence class (X == Y).
 * If X_1 * Y_1 = Z_1 and X_2 * Y_2 = Z_2, and X_1 == X_2 and Y_1 ==
   Y_2, then Z_1 == Z_2.
 * Same, but for addition.

I chucked in an extra "iterate until fixed point", as I'm paranoid,
but I'm pretty sure this isn't necessary due to the magic of
unification.

It's surprisingly slow. I recommend running it with `cargo run
--release`.

## No tests?

Not my finest hour, I'll admit. I did ad-hoc tests of each piece of
code as I went along, but not what I'd call real tests. Maybe another
day.
