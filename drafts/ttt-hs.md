# Walkthrough of a small Haskell program

## Introduction and Background

A little over a year ago I got it in my head to learn me a Haskell[1].  Admittedly it was just for the street cred at the time, though I've come to appreciate that glass-castle style beauty more and more as I spend more time working in more forgiving systems.

So, while I gave up on Haskell after a few months of exercises, I was left with a lasting impression that's followed me as I explore more different kinds of programming languages, and I i find myself missing all of the mind-blowing abstractions available with a quick `$ ghci`.  Some day I'll dive back in armed with the knowledge I've built from getting some actual stuff built, and I'm sure I'll have a better go of it.

Aside from category theory drills and a notebook full of Church numerals and eta reductions, I don't have a lot to show for my burst of Haskell enthusiasm.  Discovering Haskell led me directly to other fun, popwerful things like OCaml and Scala and Clojure, and pushed me to re-try learning Rust where I've decided to make my home for the time being, but I was never motivated to stick around through a alrger project.

I did, though, write a pretty mean ~100 lines of TicTacToe in Haskell to promptly forget about, which brings us to the main event.
## The Main Event

This is **not** a tutorial.  It is not intended to be.  Therefore, this isn't supposed to teach you Haskell, there's books that are far better at that than I could ever be.  However, if you're curious about Haskell, it might be fun to see an example of a toy program which actually does something.  IO in Haskell is not obvious to the newcomer, but not as scary as the terminology around it makes it seem.  Heck, I didn't make it past line 3 without dropping the M word.  It can help to look at examples before trying to fully grok Monads or why they're useful[2].  I have aimed to make this post accessible even if you've never actually seen single line of Haskell in your life without getting bogged down in minutia.

Conversely, I don't atually know Haskell.  This little thing is to date the most significant thing I've built with it.  So this isn't really for Haskell people either either.  In fact, they'll likely be bored and/or appalled.  In fact, I go out of my way to explain some very basic Haskell ideas.  This post is probably not for you.

All in all, this post is really for me, as an exercise in digging through past thought processes.  If anyone else reads it and gets something out of it, all the better.  Instead of going line by line or function by function, I'm going to follow the flow of logic as well as I can.  This is a short program, so I will attempt to cover it in its entirety.

With the exposition out of the way, lets take a look, whoever you are!

### The Main Main Event
Haskell programs are organized into modules.  Opening up `Main.hs`, we see line one states: `module Main where`, followed by several imports.  Ok, fairly standard stuff outside of Haskell too.  This program only has the one module, but if there were more, Main would be a good place to start looking.

I see some type declarations right under the import statements, but I don't really understand what needs modelling yet, so instead I'm going to skim down and see which actual function is called first when you execute this.  Functional programs are one large function composed of many smaller ones.  In `Main.hs`, this function is also called `main` and lives at the bottom of the file:
```haskell
main :: IO ()
main = do
  let board = freshBoard
  runGame board
```

[1] [Great Good](http://learnyouahaskell.com/) is an overestatement - Hopefully Not For Nothing is more accurate in my case, but this is a great book nonetheless