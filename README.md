# Algar
Algebric structures, higher-kinded types and other category theory bad ideas.

## Why?

I wrote this library for two reasons: first, mainly as a playground for learning *Category Theory* and *Rust*, second to see if it was even possible to
implement such general abstract nonsense in Rust.

## Prior Art and Further Reading

This library draws heavy inspiration from mathematics, other languages,
and other Rust and Elixir libraries; let me mention them here.
There is much, much more out there, but these are our highlights and inspirations.

The [`Witchcraft`](https://github.com/witchcrafters/witchcraft) Elixir library is the reason why I started this journey.

[`Fp-core.rs`](https://github.com/JasonShin/fp-core.rs), and [`higher`](https://github.com/bodil/higher),s
have been invaluable resources to help me to port category theory concepts in Rust. 

[The Fantasy Land Spec](https://github.com/fantasyland/fantasy-land) is a spec for
projects such as this one, but targeted at Javascript. It does not come with its
own implementation, but provides a [helpful chart](https://github.com/fantasyland/fantasy-land/raw/master/figures/dependencies.png)
of class hierarchies.

Obviously the Haskell [`Prelude`](https://hackage.haskell.org/package/base-4.10.0.0/docs/Prelude.html)
deserves mention. Haskell has inspired so many programmers to write clean,
declarative, functional code based on principled abstractions.

## Interested in learning more of the underlying ideas? 

I heavely recommend:
- [Category Theory for Programmers](https://bartoszmilewski.com/2014/10/28/category-theory-for-programmers-the-preface/)
- [Functors, Applicatives, And Monads In Pictures](https://www.adit.io/posts/2013-04-17-functors,_applicatives,_and_monads_in_pictures.html)
- [A Pragmatic Introduction to Category Theory—Daniela Sfregola](https://www.youtube.com/watch?v=Ss149MsZluI)

Walking through those resources probably won't change your code overnight. Some people call it
"[general abstract nonsense](https://en.wikipedia.org/wiki/Abstract_nonsense)"
for a reason. That said, it does provide a nice framework for thinking about
these abstract ideas, and is a recommended pursuit for all that are curious.
