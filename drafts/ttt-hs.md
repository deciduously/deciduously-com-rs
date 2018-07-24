# Walkthrough - TicTacToe in Haskell

## An exercise in re-reading old code I ostensibly wrote in a crazy language I've since forgotten

## The intro

A little over a year ago I got it in my head to learn me a Haskell[1].  Admittedly it was just for the street cred at the time, though I've come to appreciate that specific glass-castle style beauty more and more as I spend more time working in more forgiving systems.  I gave up on Haskell after a few months of exercises, but I was left with a lasting impression that's followed me as I explore more different kinds of programming languages, and I find myself missing all of the mind-blowing abstractions available therein with a quick `$ ghci`.  Some day I'll dive back in armed with the knowledge I've built from getting some actual stuff built, and I'm sure I'll have a better go of it.

I don't have much to show for this experiement, having jumped ship for other hipster languages.  I did, though, as my final parting act, write a pretty mean ~100 lines of crappy TicTacToe just to prove to myself I could do *something* after all that.  This was in spring 2017, which means it should be no problem to tell you how it works here now, a little over a year later.  No problem.

## The program

This is a dirt simple TicTacToe game played on the command line against a randomly playing computer opponent.  Fun, right?  Hours, nay, DAYS of entertainment await.  A project like this is usually my goto "hello world" in a new language, because at the end it demonstrates you can leverage the language's various facilities at least a little.  For Haskell, it was more a "TTFN, world".  The full source can be found [here](https://github.com/deciduously/tictactoe-hs/blob/master/src/Main.hs), the entirety of which will appear in snippet-form below.
TODO INSERT EXAMPLE GAMEPLAY!

## The walkthrough

This is **not** a tutorial.  I guess it kinda sorta is if you specifically want to make this version of TicTacToe and nothing else.  It is not intended to teach you Haskell.  There's [books](http://haskellbook.com) that are far better at teaching Haskell that than I could ever be.  If learning Haskell is your goal, you've come to a god-awful resource and should probably stop here lest I poison your mind at the get-go.  However, if you're curious about Haskell, or have maybe heard of Haskell and its constraints but can't imagine how to do even basic things, it might be fun to see an example of a toy program which actually does something that you could imagine writing easily in the language of your choice.  IO in Haskell is not obvious to the newcomer, but not as scary as the terminology around it makes it seem.  Heck, I didn't make it past line 3 without mentioning monads.  Fret not!  It's not hard, I'll walk us through enough so that it makes sense.  My aim here is for this to be easy to follow if you've never seen a line of Haskell in your life.  If anything, I'd like to leave you with an understanding of how to think about a typed pure functional program, which can apply in all sorts of different languages.  You could do worse than Scala as a next language and a lot of Haskell will directly apply.  Haskell is really great at making you learn how to do it right, by not even compiling unless you have, and I'd recommend at least playing with it to any programmer.

As should be abundantly clear, I don't atually know Haskell.  This little thing is to date the most significant thing I've built with it.  So this post isn't really for Haskell people.  They'll likely be bored or appalled.  In fact, they'll surely be both.  However, if you do red it and notice something outrageously dumb, please let me know!

All in all, this post is really for me, as an exercise in digging through past thought processes.  If anyone else reads it and gets something out of it, all the better.  Instead of going line by line or function by function, I'm going to follow the flow of logic as well as I can.  This is a very short program, so I will cover it in its entirety.

I will say that if you've never programmed before, this might be hard to read and boring, not necessarily in that order.  Do with that what you will.

I do not remember how this thing works (or much Haskell), so I'm going to write this as I read it and see if we get there in the end.  ¡Vámonos!

### First steps

Haskell programs are organized into modules.  Opening up `Main.hs`, we see line one states: `module Main where`, followed by several imports.  Ok, fairly standard stuff outside of Haskell too.  Functionality is brought in explicitly from other modules as needed.  The mudle is the first term, followed by the functions we're importing.  This program only has the one module, but if there were more, Main would be a good place to start our walkthrough anyway.

I see some type declarations right under the import statements, but I don't really understand what needs modelling yet, so instead I'm going to skim down and see which actual function is called first when you execute this.  Functional programs are one large function composed of many smaller ones.  In `Main.hs`, this function is also called `main` and lives at the bottom of the file:

```haskell
main :: IO ()
main = do
  let board = freshBoard
  runGame board
```

In Haskell, every value (functions are values, too) is tagged with its type annotation.  Haskell goes hard on the types, in a way that you've likely never come across if thinkgs like Java and C++ are as heavy a type system as you've ever worked with.  The compiler is actually magic[3] and does not require these - it's considered good style for top-level functions in a module but they can be omitted for internal values.  However, they are a huge help if you start getting bogged down in compiler errors!  A type annotation has the name first, followwed by the double colon `::`, followed by the type, and you'll see them all over Haskell code.

Our `main` value has the type `IO ()`.  Right off the bat, we get a taste of some of funky fresh Haskell weirdness, and I'm actually going to have to digress for a moment to set the scene.  Stay with me, I promise it's just a little bit.

### Setting the scene: A Digression on `IO ()`

In Haskell, every function is a pure function.  If you're not familiar with the terminology, that means that the function does not rely upon or act on values outside of its own body.  Put another way, the function will always return the same output for a given input because there is nothing else the output depends on.  The savvy among you might already be asking "but wait!   There are all kinds of things a function might want to do outside of itself.  How about printing a letter to the screen?"  To which Haskell says "Oh, shoot.  We hadn't thought of that.  Pack it up!"  Just kidding, of course.

The Haskell solution for this little technicality of actually having to be useful has do do with the types that I raved about a bit ago.  This type is read as "IO Unit".  The first part means it's of type IO, so it does something with IO.  But this is Haskell, and we need to know what *type* this function returns so that we can use it within our typed functional program.  "Doing IO" isn't a type, so Haskell has something called the IO Monad.  Explaining Monads is outside the scope of this blog post[4], so for this program what you need to know is that an IO monad like `main` will do something with IO and then return something.  For our use here, the IO Monad is the slightly confusing type of "doing IO".  That "something" it returns is the second term.  For `main`, we don't have anything to return, so we return `()`, the empty tuple.  Putting them together, we have our type `IO ()`.  This is akin to `void` in C, or, well, `unit | ()` in a bunch of diferent languages.  Zilch.

For a blitz Monad run-through with pictures, [this](http://adit.io/posts/2013-04-17-functors,_applicatives,_and_monads_in_pictures.html) will get you up to speed surprisingly quickly, from the author of ["Grokking algorithms"](http://a.co/ba5icnv).

### Now Where Were We

That wasn't too terrible, right?  Or maybe it was, I don't have a response mechanism from any sort of audience as I'm writing this.  Let me know.  Anyway, let's look back up at `main :: IO ()`.  From the type, we know it will perform IO, and give us NOTHING.  Friggin' `main`.

The definition is directly below.  You'll note I haven't called it a function - it isn't one.  It's just a monad.  We can tell because the type doesn't have an arrow `->` in it.  All functions are mappings from a type to another type (or more), like `Int -> Int`.  Main just does our IO and has `()` to show for it.

The name to the left of the defintion, the `=`, and the body to the right.  Haskell is like Python in that its scopes are whitespace delimited.  Anything indented is indside the parent scope.  Main is going to `do` a few things.

### Gettin Your Sequence On: A Second Digression on `do`

  `do` is actually syntactic sugar for some more monadic jazz, so true to form I'm refusing to talk about it in any real detail.  We can use it inside any monad like IO (if you didn't follow me down the first digression, just...IO is a monad, it's going to be OK), and it lets us "fake" an imperative style of programming.   Those same savvy from up above may have noticed `main` doesn't look like what you'd think a functional program should, doing things *and then* other things all imperitavely and stuff - in fact, it's all just a big IO monad defined by one expression chained together with the [`(>>)`/'then'](https://en.wikibooks.org/wiki/Haskell/do_notation) operator.  Pure and strongly typed, like GHC demands.  The `do` notation just helps it look cleaner.  If I lost you there, that's ok.  The takeaway is that if you're in a monad like `main :: IO ()`, you can use generally `do` to do some things sequentially and that's A-OK with Haskell.  This is what allows monads to, for instance, respond to input.

Whew.  Another token, another paragraph of exposition.  So, what is it we're doing?  The first statement I finally don't have a whole paragraph about.  We're creating a binding of the name `board` to `freshBoard`.  What's `freshBoard`, you ask?  Why, lines 27 and 28 of `Main.hs` of course!

### Leaving `main`

```haskell
freshBoard :: Board
freshBoard = Board $ replicate 9 Nothing
```

So, `freshBoard` is a `Board`.  I don't even want to know what a fresh one of these bad boys is without know what they look like, so now lets go see what types I've defined.

```haskell
newtype Board = Board [Maybe Player]
data Player = Human | Computer deriving (Eq, Show)
```

And there you have it.  I think.  This is where it starts looking a little foreign, having since descended back into the world of the living.  The brackets around `Maybe Player` mean that it's a list of `Maybe Player.`.  A `Maybe` is a useful type allowing you to encode the concept of nullablillity into the type system, instead of as a `null` value that can get thrown around.  A `Maybe` can either be `Nothing` or a `Just <something>`, in our case a `Player` from the type.  `Maybe Player` is actually also a type - `Maybe` is a *higher-kinded type* meaning it can be parameterized with a type.  Remember earlier when I called the compiler magic?  It's cool stuff.  It goes further... `Either`, which takes two type-level arguments,  can create curried types by only suplied one parameter!  They are *type-level functions*.  It's cool stuff.

Alright, armed with that knowledge, we can take a look at `Board $ replicate 9 nothing`.  This is nice and neat in that even though it looks a little incantation-y, it's got a nice English ring to it.  It's almost like reading a sentence, or at least pseudocode.  You'll want to know, going forward, about `$` - this is just function application with different precedence/associatvity rules.  Its `Board(replicate 9 nothing)`.  It seems redundant at first, but the low precedence and right-associativity let you omit parens: `f $ g $ h x  =  f (g (h x))`[5].  It looks funky but if I recall it felt natural pretty quickly.  Buckle up, because there's a little more token soup below.

`replicate 9 nothing` isn't hard to get your head around.  Function application is just spaces in Haskell (it's a function-oriented language, after all), so we're calling `replicate` with the arguments `9` and `Nothing`.  And `Board` wanted a list of `Maybe Player`s.  `replicate` makes uses the first argument to decide how many "replicas" of the 2nd to make, and returns them as a list.  Which is what we said a `Board` held. Ok, cool, so a `freshBoard` is a `Board` has nine cells that *can* hold a `Player`, but don't currently.  That's a lot of guarantees for free at compile time already from the definition.

While we're up here, it makes sense to get familiar with `Player`, since we're going to be up here a lot.  This is a union type, like an enum.  It can either be a `Human` or a `Computer`, and we've auto-derived some typeclasses for it that let us compare `Players`, i.e. tell if `Human == Human`, etc, and to display them to the console as-is.  These are the only possible values for each cell in the board.

Great!  So to recap, we've now stored a `Board` of 9 cells that might contain a `Human` or a `Computer`, but are currently empty.  What say you we move on to the *third* line of `main`?

### The Third Line of `main`

Now we're truckin' along!  Our `freshBoard` is ready for some killer moves.  The next line is a simple function call reads `runGame board` - easy enough.  We're going to pass our new board into the `runGame` function.  What does that look like?

```haskell
runGame :: Board -> IO ()
runGame board = forever $ do
  gameOver board
  print board
  putStr "Your move: "
  hFlush stdout
  n <- getLine
  case n of
    [c] ->
      if [c] `elem` map show [(1::Integer)..9]
      then do
          let n' = digitToInt c
          if openCell board n'
          then handleInput board n' >>= compTurn >>= runGame
          else putStrLn "That's taken!"
      else putStrLn "1-9 only please"
    _   -> putStrLn "Only one digit allowed!"
```

Wow - that's a bulky one.  Let's take it one step at a time.  For starters, the type itself should look familiar enough by now.  `runGame` is a `Board -> IO ()`, which is to say a function (because of the `->` it's a mapping from one thing to another) that takes a `Board`, and returns an IO monad carrying Unit, or nothing at all, just like `main`.

Diving in to the defintion, we see we're going to define another `do` block, but it's going to get wrapped inside a `forever`.  If you recall, that `$` is just regular old function application, so everything after it inour definition is inside the `forever`.  Back at the top of the file, you can see we brought it in from the `Control.Monad` module, so, you guessed it, it's a monad thing.  Luckily this one is simple - it just means we want to execute this monad forever.  I bet you already got that.  If you've made any kind of game before, you'll recognize this as the game loop, just functional flavored.

TODO

[1] I hesitated to say [Great Good](http://learnyouahaskell.com/) because that's pretty wishful thinking in my case - Hopefully Not For Nothing is more accurate.  This is a great book nonetheless
[2] Actually, I don't think Haskell is *inherently* complicated.  It's difficult to get your head around if you've used C-style imperative languages and little else.  I think Haskell might be an excellent first language.
[3] I do not mean this figuratively
[4] I had to fight down the urge to start this next sentence "The IO Monad can be thought of as a..."
[5] Haskell [Prelude](https://hackage.haskell.org/package/base-4.11.1.0/docs/Prelude.html#v:-36-)
