# Algar

[![Crates.io][crates-badge]][crates-url]
[![Build][actions-badge]][actions-url]
[![docs-badge]][docs-url]

[crates-badge]: https://img.shields.io/crates/v/algar.svg
[crates-url]: https://crates.io/crates/algar
[actions-badge]: https://img.shields.io/github/actions/workflow/status/cando/Algar/rust.yml
[actions-url]: https://github.com/cando/Algar/actions?query=branch%3Amain
[docs-badge]: https://img.shields.io/docsrs/algar?color=green
[docs-url]: https://docs.rs/algar/

Algebric structures, higher-kinded types and other category theory bad ideas.


## Why?

I wrote this library for two reasons: first, mainly as a playground for learning *Category Theory* and *Rust*, second to see if it was even possible to
implement such general abstract nonsense in Rust.

## Does category theory make you a better programmer ?

I think it does. Category theory centers around *abstraction* and *composition* and I will argue strongly that abstraction and composition are the essence of programming.

### Abstraction

Abstraction is essentially the core of computer science and exceptionally important in everyday programming: learning this sort of mathematics allows you to *unlock* a higher level of abstraction.

Since Category theory is the most abstract branch of math, it's no surprise that it lends itself to great programming abstractions and then to extremely useful programming ideas. Haskell programmers have been tapping this resource for a long time, and the ideas are percolating into other languages.

### Composition

All software development is composition. 
The act of breaking a complex problem down to smaller parts, and then composing those smaller solutions together to form structures and patterns, hence your application, well
that's what programming is all about.

> We’ve been composing things forever, long before some great engineer came up with the idea of a subroutine. Some time ago the principles of structured programming
> revolutionized programming because they made blocks of code composable. Then came object oriented programming, which is all about composing objects. Functional programming is
> not only about composing functions and algebraic data structures — it makes concurrency composable — something that’s virtually impossible with other programming paradigms.
> -- <cite>Bartosz Milewski</cite>

## Interested in learning more? 

I heavely recommend:
- [Category Theory for Programmers](https://bartoszmilewski.com/2014/10/28/category-theory-for-programmers-the-preface/)
- [A Pragmatic Introduction to Category Theory—Daniela Sfregola](https://www.youtube.com/watch?v=Ss149MsZluI)
- [Functors, Applicatives, And Monads In Pictures](https://www.adit.io/posts/2013-04-17-functors,_applicatives,_and_monads_in_pictures.html)

Walking through those resources probably won't change your code overnight. Some people call it
[general abstract nonsense](https://en.wikipedia.org/wiki/Abstract_nonsense)
for a reason. That said, it does provide a nice framework for thinking about
these abstract ideas, and is a recommended pursuit for all that are curious.

## Prior Art

This library draws heavy inspiration from mathematics and other Rust and Elixir libraries: let me mention them here.

The [`Witchcraft`](https://github.com/witchcrafters/witchcraft) Elixir library is why I started this journey.

[`Fp-core.rs`](https://github.com/JasonShin/fp-core.rs) and [`higher`](https://github.com/bodil/higher)
have been invaluable resources to help me to port category theory concepts in Rust. 

[`The Fantasy Land Spec`](https://github.com/fantasyland/fantasy-land) is a spec for
projects such as this one, but targeted at Javascript. It does not come with its
own implementation, but provides a [helpful chart](https://github.com/fantasyland/fantasy-land/raw/master/figures/dependencies.png)
of class hierarchies.

Obviously the Haskell [`Prelude`](https://hackage.haskell.org/package/base-4.10.0.0/docs/Prelude.html)
deserves mention. Haskell has inspired so many programmers to write clean,
declarative, functional code based on principled abstractions.



