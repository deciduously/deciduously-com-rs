# A Tour of a Haskell App without Honor or Humanity

#### An exercise in re-reading old code I ostensibly wrote in a crazy language I've since forgotten and never really knew

## The intro

A little over a year ago I got it in my head to learn me a Haskell[1].  Admittedly it was just for the (minimal) street cred at the time, but even though I gave up on Haskell after a few months of exercises I was left with a lasting impression that's followed me as I explore more different kinds of programming languages, and I find myself missing all of the mind-blowing stuff this lnague had ready to go with a quick `$ ghci`.  Some day I'll dive back in armed with the knowledge I've built from getting some actual stuff built, and I'm sure I'll have a better go of it.

In the meantime, I don't have much to show for this experiment, having jumped ship for other hipster languages.  I did, though, as my final parting act, write a pretty mean ~100 lines of TicTacToe just to prove to myself I could do *something* after all that.  This was in spring 2017, which means it should be no problem to tell you how it works here now, a little over a year later.  No problem.

I am not remotely qualified by any metric to write this post, but as it turns out Haskell is just not that bad.  What's can be scary is how different it can be to work with than what you're used to, so you hit a lot more walls at the very beginning, and it can feel difficult knowing how to even go about implementing something simple like this.  Hopefully seeing it in English too to will help you (and me, again) get going!

This is **not** a tutorial - I don't think.  I do think it might be useful for building other little Haskell programs for getting your feet wet, but I don't build this up in the way a traditional tutorial would, making sure it compiles along the way.  This is top-down, entry-point first, let's see what's here.    I'd argue it's an equally important skill, but then again, I wrote the damn thing so who'd take my word for it.

## The program

This is a dirt simple TicTacToe game played on the command line against a randomly playing computer opponent.  Fun, right?  Hours, nay, DAYS of entertainment await.  A project like this is usually my go-to "hello world" in a new language, because at the end it demonstrates you can leverage the language's various facilities at least a little, like control flow and IO and overall structure.  For Haskell, it was more a "TTFN, world", but the point stands.  The full source can be found [here](https://github.com/deciduously/tictactoe-hs/blob/master/src/Main.hs), the entirety of which will appear in snippet-form below.

The squares are labelled like so (BEN SEE IF YOU CANT GET THE NUMBERS ON FOR THIS POST).

```
1 2 3
4 5 6
7 8 9
```

Here's a sample game:
```
 _  _  _
 _  _  _
 _  _  _
 
Your move: 3
 _  O  X
 _  _  _
 _  _  _

Your move: 7
 _  O  X
 _  _  _
 X  O  _
 
 Your move: 5
 _  O  X
 _  X  _
 X  O  _

Human won!
```

Suck it, random number generator.

## The walkthrough

My aim here is for this to be easy to follow if you've never seen a line of Haskell in your life.  If anything, I'd like to leave you with an understanding of how to think about a typed pure functional program, which can apply in all sorts of different languages.  Haskell is really great at making you learn how to do it right by not even compiling unless you have, and I'd recommend at least playing with it to any programmer.  In the course of untangling this program, I'm going to get down and dirty with just enough Haskell goodness to explain the code I have - no further, but I'm not going to try to gloss over tricky concepts.  This game is a simple end goal, but it's gonna get a little conceptual so get ready to think a little, especially if you're new to either functional programming or strong type systems.

As should be abundantly clear, I don't atually know Haskell.  This little thing is to date the most significant thing I've built with it.  So this post isn't really for Haskell people.  They'll likely be pissed off or appalled.  In fact, they'll surely be both.  However, if any Haskellers do read it and notice something outrageously dumb that simply cannot stand, please let me know so I can correct it!  I'm going for pared down and simplified for the domain, not dead wrong.

All in all, this post is really for me, as an exercise in digging through past thought processes, sprinkled with a whiff of nostalgia for the glass castle.  If anyone else reads it and gets something out of it, all the better.  Instead of going line by line or function by function, I'm going to follow the flow of logic as well as I can.  This is a very short program, so I will cover it in its entirety.  There are a few digressions that dig a little bit into some of why Haskell is the way that it is, but I'm *hoping* these are optional, or at least skimmable.  I know there's a lot here for relatively little output - but Haskell is for Life!

I will say that if you've never programmed before, this might be hard to read and boring, not necessarily in that order.  Do with that what you will.  I do assume familiarity with programming concepts in a general sense like loops and control flow

I do not remember how this thing works (or much of how Haskell works), so I'm going to write this as I read it and see if we get there in the end.  ¡Vámonos!

### First steps

Haskell programs are organized into modules.  Opening up `Main.hs`, we see line one states: `module Main where`, followed by several imports.  Ok, fairly standard stuff outside of Haskell too.  Functionality is brought in explicitly from other modules as needed.  The module is the first term, followed by the functions we're importing.  This program only has the one module, but if there were more, Main would be a good place to start our walkthrough anyway.  Programmers intuition or something.

I see some type declarations right under the import statements, but I don't really understand what needs modelling yet, so instead I'm going to skim down and see which actual function is called first when you execute this.  This is significant in any language, but in Haskell your whole program *is* this value.  Or your module export, at least.  Your task is to define the result of computing this value.  In `Main.hs`, this value is also called `main` and lives at the bottom of the file:

```haskell
main :: IO ()
main = do
  let board = freshBoard
  runGame board
```

In Haskell, every value (functions count, they evaluate to values) is tagged with its type annotation.  Haskell goes hard on the types, in a way that you've likely never come across if things like Java and C++ are as heavy a type system as you've ever worked with.  The compiler is actually magic[3] and does not require these - it's considered good style for top-level functions in a module but they can be omitted for internal values.  However, they are a huge help if you start getting bogged down in compiler errors!  A type annotation has the name first, followed by the double colon `::`, followed by the type, and you'll see them all over Haskell code.

Our `main` value has the type `IO ()`.  Right off the bat, we get a taste of some of funky fresh Haskell weirdness, and I'm actually going to have to digress for a moment to set the scene.  Stay with me, I promise it's just a little bit.

#### Setting the scene: A Digression on `IO ()`

I'm going to preface this by saying I am *not* making this a blog post about Monads.  I do, though, need to talk about them at least a little (we can gloss through most of the details but theres a `(>>=)` or two just sitting there), and they're really not a scary thing at all.  This is the super simple shakedown, and it's only a shakedown becase I thought it sounded good after "super simple".  If you have no clue what I'm talking about, on either point, don't worry about it.

IO is a monad.  This means it's a type that can have other things that are also monads inside of it.  In fact, it sometimes *is* those other things!  Our whole program, `ttt`, is an `IO ()`, which is a `Monad`.

In Haskell, every function is a *pure* function.    If you're not familiar with the terminology, "pure" means that the function does not rely upon or act on values outside of its own body.  Put another way, the function will always return the same output for a given input because there is nothing else the output depends on, and you're guaranteed that nothing outside of itself will change in the course of running it.  The savvy among you might already be asking "but wait!   There are all kinds of things a function might want to do outside of itself.  How about printing a letter to the screen?"  To which Haskell says "Oh, shoot.  We hadn't thought of that.  Pack it up!"  Good post everyone.

...Hah!  Got you, didn't I.  The Haskell solution for this little technicality of actually having to be useful has to do with the types that I raved about.  I read this as "IO Unit".  The first part means it's of type IO, so it does something with IO (Input/Output).  But this is Haskell, and we need to know what *type* this function returns so that we can use it within our typed functional program (spoiler alert: specifically other monads).  Our compiler is just doing it's happy ol' thing evaluating values, we've got to keep the computation rollin'! .  "Doing IO" isn't a type, so Haskell has something called the IO Monad.  For this program right now what you need to know is that an IO monad like `main` will do something with IO but also evaluates to something.  The `Monad` is a way of encapsulating that idea -  whatever it does will happen inside of it and then you get this second type back. A monad can be thought of as an "action" or "computation".  It isn't the action itself, it's just the concept of carrying out that action.  It's a noun through and through, just a "thing" we can pass around in our program, but it's a little weird so don't worry if that's not sitting well with you yet.  Monads turn out to be a great way to compose functionality especially where stuff gets messy, like responding to input from the user in a language that only allows pure functions.

Don't worry if I lost you here, really.  For our use in this program, the IO Monad is the (slightly confusing) type of "doing IO".  The "something" it carries is the second term.  For `main`, we don't have anything, so we return `()`, the empty tuple.  Putting them together, we have our type `IO ()`.  This is akin to `void` in C, or, well, `unit | ()` in a bunch of diferent languages.  Zilch.

For a blitz Monad (etc.) run-through with pictures, [this blogpost](http://adit.io/posts/2013-04-17-functors,_applicatives,_and_monads_in_pictures.html) will get you up to speed surprisingly quickly, from the author of ["Grokking algorithms"](http://a.co/ba5icnv).

### Back to the Grizz

That wasn't too terrible, right?  Or maybe it was, I don't have a response mechanism from any sort of audience as I'm writing this.  Let me know.  I definitely regret saying "Grizz" already, no need to mention that.  It's not a real word.  Anyway, let's look back up at `main :: IO ()`.  From the type, we know it will perform IO, and give us NOTHING `()` back.  Friggin' `main`, pull a little weight once in a while, huh?

The definition is directly below.  You'll note I haven't called it a function - it isn't one.  It's just an `IO ()`.  A noun, not a verb - a thing that performs some IO to evaluate.  My favorite kind of TicTacToe, luckily!  We can tell because the type doesn't have an arrow `->` in it.  All functions are mappings from a type to another type (or more), like `Int -> Int` or `Int -> ()`.  Main just does our IO and has `()` to show for it.

The name to the left of the defintion, the `=`, and the body to the right.  Haskell is like Python in that its scopes are whitespace delimited.  Anything indented is indside the parent scope.  Main is going to `do` a few things.

#### Gettin Your Sequence On: A Second Digression on `do`

  `do` is actually syntactic sugar for some more monadic jazz, so true to form this is not a full explanation, but rather a taste.  We can use this structure inside any monad (like IO) (if you didn't follow me down the first digression, IO is a monad, we're in one right now, it's going to be OK), and it lets us "fake" an imperative style of programming.   Those same savvy from up above may have noticed `main` doesn't look like what you'd think a functional program should, doing things *and then* other things all imperatively and stuff - in fact, it's all just a big IO monad defined by one expression chained together with the [`(>>)`/'then'](https://en.wikibooks.org/wiki/Haskell/do_notation) operator.  Pure and strongly typed, like GHC demands.  The `do` notation just helps it look cleaner.  If I lost you there, that's ok.  The takeaway is that if you're in a monad like `main :: IO ()`, you can use generally `do` to do some things sequentially and that's A-OK with Haskell.  This is what allows monads to, for instance, respond to input.  Inside the do block, both things I call out to are also `IO` monads.  The total value of `main`, i.e. the result of running the executable, relies on some external input to compute, and it's going to need to respond based on whatever input it receives.

Whew.  Another token, another paragraph of exposition.  So, what is it we're doing?  The first statement I finally don't have a whole paragraph about.  With the line `let board = freshBoard` We're creating a binding of the name `board` and assigning it the value `freshBoard`.  What's `freshBoard`, you ask?  Why, lines 27 and 28 of `Main.hs` of course!

### Leaving `main`

```haskell
freshBoard :: Board
freshBoard = Board $ replicate 9 Nothing
```

So, `freshBoard` is a `Board`.  I don't even want to know what a fresh one of these bad boys is without know what a `Board` looks like in general, so now lets go back up to the top and see what types I've defined.

```haskell
newtype Board = Board [Maybe Player]
data Player = Human | Computer deriving (Eq, Show)
```

And there you have it.  The brackets around `Maybe Player` mean that it's a list of `Maybe Player.`.  Obvious, right?  I'm joking, I'll talk about it.  A `Maybe` is a useful type allowing you to encode the concept of nullablillity into the type system, instead of as a `null` value that can get thrown around.  A `Maybe` can either be `Nothing` or a `Just <something>`, in our case a `Player` from the type.  `Maybe Player` is actually also a type - `Maybe` is a *higher-kinded type* meaning it can be (nay, WILL be!  MUST be!) parameterized with a type.  Without a type parameter, `Maybe` is not a complete, usable type at all - every `Maybe` will carry a specific type.  `Maybe` has *kind* `* -> *`, meaning a mapping from something to something, and when `Player` is that `*` something type, it becomes the fully resolved type `Maybe Player`, with *kind* `*` that can be fully evaluated in other functions in our quest for the One True Value.  Remember earlier when I called the compiler magic?  It goes further... `Either`, which is *kind* `* -> * -> *`takes two type-level arguments, can (and does) create curried types by only supplying one parameter!  For example `Either Player` is a partially resolved type that still has kind `* -> *`.  These are *type-level functions*.  It's cool stuff.

Alright, armed with that knowledge, we can take a look at `Board $ replicate 9 nothing`.  This is nice and neat in that even though it looks a little incantation-y, it's got a nice English ring to it.  It's almost like reading a sentence, or at least pseudocode.  You'll want to know, going forward, about `$` - this is just function application with different precedence/associatvity rules.  Its `Board(replicate 9 nothing)`.  It seems redundant at first, but the low precedence and right-associativity let you omit parens: `f $ g $ h x  =  f (g (h x))`[5].  It looks funky but if I recall it felt natural pretty quickly.  Buckle up, because there's a little more token soup below.  Haskell is not shy about esoteric operators.

`replicate 9 nothing` isn't too hard to tease apart.  Function application is just spaces in Haskell (it's a function-oriented language, after all), so we're calling `replicate` with the arguments `9` and `Nothing`.  And `Board` wanted a list of `Maybe Player`s.  `replicate` makes uses the first argument to decide how many "replicas" of the 2nd to make, and returns them as a list.  Which is what we said a `Board` held. Ok, cool, so a `freshBoard` is a `Board` has nine cells that *can* hold a `Player`, but don't currently.  That's the whole data structure for the app.  We get a lot of guarantees for free at compile time already from the definition- definitely more than your average vector or array.

While we're up here, it makes sense to get familiar with `Player`, since we're going to be up here a lot.  This is a union type, like an enum.  If you've never worked with those, it's just a bit of data that can either be a `Human` or a `Computer` and nothing else, and we've auto-derived some typeclasses for it that let us compare `Players` for equality, i.e. tell if `Human == Human`, etc, and to display them to the console as-is.  These are the only possible values for each cell in the board.

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

Wow - that's a bulky one.  Let's take it one step at a time.  For starters, the type itself should look familiar enough by now.  `runGame` is a `Board -> IO ()`, which is to say a function (because of the `->` we know it's a mapping from one thing to another) that takes a `Board`, and returns an IO monad carrying Unit, or nothing at all, just like `main`.

Diving in to the defintion, we see we're going to define another `do` block, but it's going to get wrapped inside a `forever`.  If you recall, the `$` operator is just regular old function application, so everything after it in our definition is inside the `forever`.  Back at the top of the file, you can see we brought it in from the `Control.Monad` module, so, you guessed it, it's a monad thing.  Luckily this one is simple - it just means we want to execute this monad forever.  I bet you already got that.  If you've made any kind of game before, you'll recognize this as the game loop, just functional flavored.  We're going to do whatever's inside this function over and over again until something somewhere tells us the game is over.

What's inside this function, then?  The next line immediately calls out to another function called `gameOver` and passes it the board, which right now is fresh.  Let's look at `gameOver`.

```haskell
gameOver :: Board -> IO ()
gameOver board@(Board b) =
  when ( all isJust b) $ do
    print board
    putStrLn "Draw!"
    exitSuccess
```

Well, that type signature should be getting repetitive.  This is another one that takes a `Board`, does some sort of IO, and doesn't pass anything back to the caller.  The token soup in the second line is just destructuruing syntax - remembering that our `Board` is the only argument, all `board@(Board b)` does is allows us to refer to both the whole structure as `board` as well as specifically the inside list of cells as `b`.  The body of this function is straightforward to read.  `when ( all isJust b)` we're going to `do` something.  `when` is another thing we imported from `Control.Monad`, but it's also not scary and does what you'd expect - checks the predicate and enters the block if true.  Remember that each one off the nine cells is a type of `Maybe Player`, and a `Maybe a` can be either `Just a` or `Nothing`, using `a` as a stand-in for any type.  `isJust` is a helper predicate from `Data.Maybe` (imported, like a fine wine) that returns true if what was passed in is the `Just` variety of `Maybe`.  We passed it along with our list of cells `b` into `all`, which is like a big ol' `AND`/`&&` - it returns the false the first time it hits a false, or the whole expression is true.  So when every cell has a player in it, `gameOver` will notice that it's time to pack it up and end the game.  Specifically, it will show you the board with `print` (details below) and tell you the game was a draw with `putStrLn`.  These only work in an IO Monad, and finally justify all that hullaballoo about monads before we could dive in!  Remembering that `do` is secretly chaining together its children with a `then`, this ends up looking a lot like your garden variety imperative, impure stuff, but never breaks any rules to do so.  It's all one big IO monad built from the inner results of calling each of these functions, which themselves return IO monads making it all work.  That's why `main` has to be an IO monad as well even though it doesn't perform any IO explicitly - it's built from functions that do.  When the printing is over, we just `exitSuccess`, terminating the program with status code 0.

So `gameOver` just makes sure there's still a game to play on the board before diving in and trying to run a turn.  If we're done, the whole process ends, and if not this function doesn't do or return anything at all so `runGame` can progress.  We've just begun our journey, so when we passed in the `Board`, `all` of it was most definitely not `isJust`.  Moving on, what does a run of the game loop look like?

First, it looks like we `print` it out.  Groovy.  But wait!  Slow down.  How does the compiler know what a `Board` should look like?  We made that type up ourself (details follow, as promised).  Well, in Haskell `printablility` is expressed as a *typeclass* called `Show`.  We've been using typeclasses this whole time - they're (to me) whole point of learning Haskell in the first place.

### Digression: Typeclasses: types, with class

I know I said I wouldn't go too much into it, but this is fun and quick.  It's got all kinds of typeclass goodness to unwrap.  We already know `Maybe` is a higher-kinded type, specifically of *kind* `* -> *`, which means its one of those fancy type-level functions - those asterisks stand in for any type.  This syntax is used to describe the *kind*s of types.  What we didn't talk about with `Maybe` is that it's a member of several useful typeclasses.  We've already talked about some: `Monad`, you may (or may not, I don't know) hae guessed, `Eq`, `Show`.  These apply to specific types like `Int` or `Maybe` and define what happens to them in certain situations.  For now, you can think of them as not unlike interfaces in an object-oriented settings, but they're really so much more than just interfaces.  The compiler knows how to derive simple ones for us for simple types - for instance, when you want to print a `7` to the screen, you pretty much always want to write that numeral to stdout.  If you ask if that `7` is `==` another `7`, it's reasonable to assume the compiler can tell you it, in fact, is.

For union types like `Player`, we can tell the compiler to assume we just want to print out the name of the variant like `Human` or `Computer`.  But if we wanted to do something crazy, we could easily just define our own instance of `Show` that has code to manipulate it.  With a more complicated type, like `Board`, we want to have that control.  Here's our definition of `Show` for `Board`, which `print :: IO ()`[6] is currently asking for in order to evaluate:

```haskell
instance Show Board where
  show (Board cs) = foldr spaceEachThird [].withIndicesFrom 1.fmap showCell $ cs
    where spaceEachThird a = (++) (bool (snd a) (snd a ++ "\n") (fst a `rem` 3 == 0))
```

My god, Ben, what have you written.  This should be good.  Lets tease this apart.  That first list just says we're defining what `Show` should do for `Board`.  So every time a caller needs to `Show` a board, it will come here and evaluate what's inside.

To define a typeclass instance, you need to define the functions the typeclass requires.  `Show` is an easy one to define - there's just the one, `show a`, where in this case `a` will be `Board`.  And as you'd expect, we can see the left half of the definition agrees: this function will `show (Board cs)`, so if we pass in ur `Board` newtype, `cs` will refer to the list of cells inside.

Luckily, Past Ben seems to have golfed this one, the bastard.  No comments or anything.  To be fair, I don't think Past Ben expected Present Ben (Future Ben?) to write this post, ansd Haskell is a lot of fun to golf.  No matter.  That first function, `foldr`, gives me an idea what I'm getting at already.  Lets talk about folding.

So, this whole time we've been talking about Haskell is *functional* and not *imperative* - the unit of computation is the function, and you construct computations by composing functions.  However, I immediately threw that `do` thing at you which does kinda-sorta let you code imperatively, but that's still just a special syntax for describing a purely functional set of computations.  We're going to run in to a problem if we want to, say, perform the same action on a list of things.  Which is exactly what needs to happen.

In a C-style language, to solve this problem of printing each cell to the screen, you'd iterate over the cells with something like a `for` loop.  In Haskell, ther'es no such thing.  A loop isn't a function or a value, and those comprise our whole toolbox.  But we still have to solve this problem.  Luckily Haskell provides a rich set of tools for approaching this type of problem functionally using *recursion*, and the `fold` operation is a building block that makes this easier than writing it out by hand.

By the way, this whole bit is not at all Haskell specific.  Recursion and folds will show up in all sorts of places, Haskell just happens to be an excellent evirnoment for really getting familiar with how to build them.

#### A Digression on `foldr`

The way we take a collection values and make sure we do something with every member of the collection is to consume the collection recursively.  That is, we're going to pass our whole collection into some sort of function which is going to do some sort of processing.  At the end of the function, it's going to call itself again, just with a smaller part of the list - the part we haven't processed.  It will do this again and again, just calling itself with smaller and smallr parts of the collection, until the whole thing is processed.  Easy peasy.  A `fold` is a specific type of recursive function that takes in a data structure, a collection of some type, and a function to use for each member.  It eventually yields just one single value - the eventual result of calling that function on the member and the result of all the previous runs through our recursive function.  The `reduce` operation is a special case of a `fold`, if you've come cross that in, say, JavaScript or Python.

Types are one thing that are at least for me more confusing in english.  If looking at types helps you out, here's the type signature for `foldr`:


```haskell
foldr :: (a -> r -> r) -> r -> [a] -> r
```

It's fine if you stared blankly at that, that's usually step one of unravelling a type signature.  They all work the same way, though, so we can tease it apart slowly.  We know this is a function that takes three arguments, because eveything evaluates to one value in the end - so the compiler will expect three bits of information while processing this to get to that final `r`.  The second unknown type is conventionally shown with a `b` - I'm using `r` to indicate it's our return type.  It doesn't matter what type - it could be anything.  It could even be another `a`, and often is, but it doesn't *have* to be so we use a different letter.

The first thing is our processing function, with signature `a -> r -> r`.  This itself is a function, which takes two arguments, by the same logic as above.  It takes in a single element of our `[a]`, that is, list of `a` types, and some value of the type that we're returning, and returns a new return type.  When you pass in one cell of our `Board`, this function will give back the next accumulated result.  The next argument is a single instance of that return type - the "destination" so to speak.  We know we're going to be getting a single value from this fold, and we have a function that takes a cell and our current running result and gives us back the new result, so we can drop that cell from the next run through the recursion.  But the firrst run through, we need somewhere to deposit the result of the computation - so `foldr` asks for a container as the second argument of type `r` to apply the result to.  This initial value we pass in is going to be transformed every run through the function and is eventually what gets returned.

If this all was too abstract, here's a simple example that might look more familiar - let's fold some basic addition into a collection:

```haskell
nums :: [Int]
nums = [1, 2, 3, 4, 5]

addEmUp ns :: [a] -> r
addEmUp ns = foldr (+) 0 ns
```

That's a lot less noisy.  In this example, calling `addEmUp nums` will yield `15:: Int`.  First, I defined a `[Int]`, that is, a list of `Int`s, called `nums`.  Then I created a function `addEmUp` which is really just an alias for a specific `fold` - notice how it doesn't do anything else, just specifies which arguments to use with the fold.  That's why the type signature for `addEmUp` is a lot simpler - it only takes the `[a]` collection, in this case `nums`.  So our `a` is `Int`.  The first argument, the prosessor, is `(+)` - just the addition operator.  Operators are functions, and this one takes in two values and produces a third.  Let's compare to our expected tpe: `a -> r -> r`.  Well, in this case, `a` is `Int`, and also we want an `Int` at the end, so we can substitute it in for `r` too.  If you add an `Int` to an `Int`, lo and behold, an `Int` will pop out.  So our processor, addition, has type `Int -> Int -> Int`, which fits!  Remember, it's totally fine if `a` and `r` or any two unspecified types are the same, we just note that they don't *have* to be.

Our second argument was just a `0` - an `Int`.  We just decided that's a perfectly fine `r` type, so the second argument makes sense as an initializer for our return type, and that just leaves us with `[a]`.  Thankfully we've left that part of the type intact, and are passing it in!  So for this simple example, the fully qualified type of this `foldr` reads: `(Int -> Int -> Int) -> Int -> [Int] -> Int`.  Just a bunch of `Int`s.

When Haskell goes to evaluate it, it will start with the full collection.  When we get to the first run through, the processor will grab the first cell, and then look for our accumulated result.  We haven't done anything yet, so it's just `0` - we told it that in the second argument.  The first value is `1`, so our accumulator added to our base value is `1`.  Then, we recur!  Only this time we've already processed the one, so we're calling this same function again but a little different:

```haskell
foldr (+) 0 [1, 2, 3, 4, 5]
foldr (+) 1 [2, 3, 4, 5]
```

See what happened there?  We processed the one and dropped it, so our collection got shorter and we have a running total.  Expanding:

```haskell
  foldr (+) 3 [3, 4, 5]
= foldr (+) 6 [4, 5]
= foldr (+) 10 [5]
= foldr (+) 15 []
= 15
```

When a recursive function tries to recur on an empty list, it knows it's done and returns the final value - in this case `15`.  We've managed to iterate without looping!  We were able to reuse the same exact function over and over again, only changing what we pass in based on the output of the previous run.  Recursion, yo.

If this sounds outrageously inefficient, calling loads and loads of functions all the time with very similar values, you're correct.  Haskell performs something called "[tail-call](https://en.wikipedia.org/wiki/Tail_call) optimization", which I won't detail here but essentially means that instead of allocating a new stack frame for each successive call, it's able to reuse the same stack frame and substitute the new vals, and then just jump execution back up.  If you're not familiar with stack frames, we're getting way beyond the scope of this post - it's not required knowledge here but interesting in general and important to understand if you'd like to use a functional language in anger, so I recommend you do some poking around!

As an aside, this example could have been rewritten: `addEmUp = foldr (+) 0` - if the argument is the final term in the definition and the argument list, it can be dropped.  This process is known as an [eta-reduction](https://en.wikipedia.org/wiki/Lambda_calculus#%CE%B7-conversion) in the lambda calculus lingo.  The compiler instead sees this definition as a curried function expecting one more value, and if it gets called with that value, it will fully evaluate the expression.

#### Back to `Show`

That digression got away from me, but now we're armed to dive in to this bigger, messier fold.  We know its going to do the same basic type of thing as `addEmUp`.  So the first thing to look for is those three elements we know we'll need: the processing function, the starting value to accumulate in to, and the collection to process.  As a reminder, here's the first line of our `show` definition:

```haskell
show (Board cs) = foldr spaceEachThird [].withIndicesFrom 1.fmap showCell $ cs
```

The final part, the collection, is easy.  Remembering that `$` is function application, we know we're going to apply this fold to `cs`, which we know from the argument list is our list of cells: `Board cs`.  Then we can just follow the types.  We have a word `spaceEachThird` in the first position, which must be our processing function, and the rest of it must just define our starting accumulator.  It's a lot more to look at than a nice neat `0` but I bet (well, I hope, again - I don't remember this code) that it's going to evaluate to a value, not a function.

I think I want to explore that `spaceEachThird` shindig first - this is what's going to happen to every cell.

```haskell
where spaceEachThird a = (++) (bool (snd a) (snd a ++ "\n") (fst a `rem` 3 == 0))
```

The `where` just means we're defining `spaceEachThird` locally for this function only - it isn't needed outside of this exact context.  We could have defined it inline using Haskell's anonymous function syntax (`\x -> x + 1`), but even I must have decided that was too hard to read and split it out.

Anyway, `spaceEachThird` has been defined as taking a single argument, `a`.  In this case, `a` is going to be our current cell - conveniently it matches what we've been using as a stand-in type.  We know the processor acts on two input values, and the other one is our accumulator, so in our definition it's going to look like we're missing an argument.  It's going to be the accumulator.

The first part of the definition is `(++)` is concatenation.  There's a clue to where our other type goes - we're going to have whatever we're doingwith `a`, the active cell, on one side, and it's going to get concatenated to the accumulator.  That makes sense - it's kind of like adding an `Int` to the accumulator.  The accumulator will now hold information from both operands.  What on earth are we adding, though?

I've grabbed the `bool` function from `Data.Bool` and it's really just some control flow.

TODO COME BACK WHEN YOU UNDERSTAND YOUR CODE

### Beyond `Show`



### Footnotes

[1] I hesitated to say [Great Good](http://learnyouahaskell.com/) because that's pretty wishful thinking in my case - Hopefully Not For Nothing is more accurate.  This is a great book nonetheless

[2] Actually, I don't think Haskell is *inherently* complicated.  It's difficult to get your head around if you've used C-style imperative languages and little else.  I think Haskell might be an excellent first language.

[3] I do not mean this figuratively

[4] I had to fight down the urge to start this next sentence "The IO Monad can be thought of as a..."

[5] Haskell [Prelude](https://hackage.haskell.org/package/base-4.11.1.0/docs/Prelude.html#v:-36-)

[6] We know it's an `IO ()` because we're inside a `do` block in an `IO ()`, it performs IO of its own, and it doesn't have any value coming back up.  It just exists to print the value to the console.  So when the compiler comes hungrily munchng through `runGame`, `print` just evaluates to `()`.
