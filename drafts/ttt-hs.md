# Some Haskell, English'd
#### A TicTacTour Without Honor or Humanity

## The Intro

I am not remotely qualified by any metric to write this post.  This is an exercise in re-reading old code I ostensibly wrote in a crazy language I've since forgotten and never really knew, by a guy who's still pretty new to this whole "learning programming" shindig.  Let's see what happens!

A little over a year ago I got it in my head to learn me a Haskell[1], just for the (minimal) street cred, and I spent a couple weeks with the (rather excellent) [book](http://haskellbook.com/). After several weeks of exercises, I wrote a pretty mean ~100 lines[1] of TicTacToe just to prove to myself I could do *something* after all that, and promptly walked off into the soft, cozy, alluring parens of Clojure.  This was in spring 2017, which means it should be no problem to tell you how it works here now, a little over a year later, having not touched any Haskell at all since.  No problem.

As it turns out Haskell is just not that bad.  What can be scary is how different it can be to work with than what you're used to, so you hit a lot more walls at the very beginning, and it can feel difficult knowing how to even go about implementing something simple like this.  Your instincts won't all apply anymore.  Hopefully seeing it in English too to will help you (and me, again) get going!

As opposed to a traditional tutorial, this is top-down, entry-point first, let's see what's here sort of deal.  I will step through every line of code as it's called and explain what's going on.

My aim here is for this to be easy to follow if you've never seen a line of Haskell in your life, but in the course of untangling this program, I'm going to get down and dirty with just enough Haskell goodness to explain why the code I have is the way it is.  I won't go very deep into anything, but I'm not going to try to gloss over tricky concepts.  This game is a very simple end goal, but it's gonna have to get a little conceptual here and there so get ready to think some, especially if you're new to either functional programming or strong type systems.

If any Haskellers do read this and notice something outrageously dumb that simply cannot stand, please let me know so I can correct it!  I'm going for pared down and simplified for the domain, not dead wrong.

I will say that if you've never programmed before, this might be hard to read and boring, not necessarily in that order.  How'd you even end up here?  Not that it's a bad thing, by all means, learn you some programmin', it's the best, but this random blogpost is probably not where you should start.  I assume familiarity with programming concepts in a general sense like loops and control flow.

I do not remember how this thing works (or much of how Haskell works), so I'm going to write this as I read it and see if we get there in the end.  ¡Vámonos!

## The Program

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

## The Good Stuff

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

...Hah!  Got you, didn't I.  The Haskell solution for this little technicality of actually having to be useful has to do with the types that I raved about.  I read this as "IO Unit".  The first part means it's of type IO, so it does something with IO (Input/Output).  But this is Haskell, and we need to know what *type* this monad returns so that we can use it within our typed functional program (spoiler alert: specifically other monads).  Our compiler is just doing it's happy ol' thing evaluating values as it sees them, we've got to keep the computation rollin'! .  "Doing IO" isn't a type, so Haskell has something called the `IO` Monad.  For this program right now what you need to know is that an `IO` monad like `main` will do something with IO but also evaluates to something.  The `Monad` is a way of encapsulating that idea -  whatever it does will happen inside of it and then you get this second type back. A monad can be thought of as an "action" or "computation".  It isn't the action itself, it's just the concept of carrying out that action.  It's a noun through and through, just a "thing" we can pass around in our program  (NOT a function, though functions can return Monads), but it's a little weird so don't worry if that's not sitting well with you yet.  Monads turn out to be a great way to compose functionality especially where stuff gets messy, like responding to input from the user in a language that only allows pure functions.

Don't worry too much if I lost you here, really.  For our use in this program, the IO Monad is the (slightly confusing) type of "doing IO" and it can carry a type with it.  That type it carries is the second term.  For `main`, we don't have anything, so we return `()`, the empty tuple.  Putting them together, we have our type `IO ()`.  This is akin to `void` in C, or, well, `unit | ()` in a bunch of diferent languages.  Zilch.

For a superior but still blitz-pace Monad (etc.) run-through with pictures, [this blogpost](http://adit.io/posts/2013-04-17-functors,_applicatives,_and_monads_in_pictures.html) will get you up to speed surprisingly quickly, from the author of ["Grokking algorithms"](http://a.co/ba5icnv).

### Back to the Grizz

That wasn't too terrible, right?  Or maybe it was, I don't have a response mechanism from any sort of audience as I'm writing this.  Let me know.  I definitely regret saying "Grizz" already, no need to mention that.  It's not a real word.

Anyway, let's look back up at `main :: IO ()`.  From the type, we know it will perform IO, and give us NOTHING `()` back.  Friggin' `main`, pull a little weight once in a while, huh?

The definition is directly below.  You'll note I haven't called it a function - it isn't one.  It's just an `IO ()`.  A noun, not a verb - a thing that performs some IO to evaluate.  My favorite kind of TicTacToe, luckily!  We can tell because the type doesn't have an arrow `->` in it.  All functions are mappings from a type to another type (or more), like `Int -> Int` or `Int -> ()`.  Main just does our IO and has `()` to show for it.

The name to the left of the defintion, the `=`, and the body to the right.  Haskell is like Python in that its scopes are whitespace delimited.  Anything indented is indside the parent scope.  Main is going to `do` a few things.

#### Gettin Your Sequence On: A Second Digression on `do`

  `do` is actually syntactic sugar for some more monadic jazz, so true to form this is not a full explanation, but rather a taste.  We can use this structure inside any monad (like IO) (if you didn't follow me down the first digression, IO is a monad, we're in one right now, it's going to be OK), and it lets us "fake" an imperative style of programming.   Those same savvy from up above may have noticed `main` doesn't look like what you'd think a functional program should, doing things *and then* other things all imperatively and stuff.  That's not how a functional program works, it's supposed to just compose the results of other functions!  In fact, it's all just a big IO monad defined by one expression chained together with the [`(>>)`/'then'](https://en.wikibooks.org/wiki/Haskell/do_notation) operator.  Pure and strongly typed, like GHC demands.  The `do` notation just helps it look cleaner.
  
If I lost you there, that's ok.  The takeaway is that if you're in a monad like `main :: IO ()`, you can use generally `do` to do some things sequentially and that's A-OK with Haskell.  This is what allows monads to, for instance, respond to input.  Inside the do block, both things I call out to are also `IO` monads.  The total value of `main`, i.e. the result of running the executable, relies on some external input to compute, and it's going to need to respond based on whatever input it receives.

Whew.  Another token, another paragraph of exposition.  So, what is it we're `do`ing?  The first statement is something I finally don't have a whole paragraph about.  With the line `let board = freshBoard` we're creating a binding of the name `board` and assigning it the value `freshBoard`.  What's `freshBoard`, you ask?  Why, lines 27 and 28 of `Main.hs` of course!

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

And there you have it.  The brackets around `Maybe Player` mean that it's a list of `Maybe Player.`.  Obvious, right?  I'm joking, I'll talk about it.  A `Maybe` is a useful type allowing you to encode the concept of nullablillity into the type system, instead of as a `null` value that can get thrown around.  Similar concepts appear in other ML-ish languages (Haskell is ML-ish) like Rust and Swift (and OCaml and Scala and F# and SML and etc, etc - it's not a new or Haskell-specific concept is the point here).  A `Maybe` can either be `Nothing` or a `Just <something>`, in our case a `Player` from the type.  `Maybe Player` is actually also a type - `Maybe` is a *higher-kinded type* meaning it can be (nay, WILL be!  MUST be!) parameterized with a type.  Without a type parameter, `Maybe` is not a complete, usable type at all - every `Maybe` will carry a specific type.  `Maybe` has *kind* `* -> *`, meaning a mapping from something to something, and when `Player` is that `*` something type, it becomes the fully resolved type `Maybe Player`, with *kind* `*` that can be fully evaluated in other functions in our quest for the One True Value.  Remember earlier when I called the compiler magic?  It goes further... `Either`, which is *kind* `* -> * -> *`takes two type-level arguments, can (and does) create curried types by only supplying one parameter!  For example `Either Player` is a partially resolved type that still has kind `* -> *`.  These are *type-level functions*.  It's cool stuff.

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

#### A digression - Typeclasses: types, with class

I know I said I wouldn't go too much into it, but this is fun and quick.  It's got all kinds of typeclass goodness to unwrap.  We already know `Maybe` is a higher-kinded type, specifically of *kind* `* -> *`, which means its one of those fancy type-level functions - those asterisks stand in for any type.  This syntax is used to describe the *kind*s of types.  What we didn't talk about with `Maybe` is that it's a member of several useful typeclasses.  We've already talked about some: `Monad`, you may (or may not, I don't know) hae guessed, `Eq`, `Show`.  These apply to specific types like `Int` or `Maybe` and define what happens to them in certain situations.  For now, you can think of them as not unlike interfaces in an object-oriented settings, but they're really so much more than just interfaces.  The compiler knows how to derive simple ones for us for simple types - for instance, when you want to print a `7` to the screen, you pretty much always want to write that numeral to stdout.  If you ask if that `7` is `==` another `7`, it's reasonable to assume the compiler can tell you it, in fact, is.

### `Show` Me The Money

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

Folds are not an uncommon concept in mainstream languages - if you're already god and comfy with them, feel free to skip this.  If not, though, it will help to know how they work.

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

That's a lot less noisy.  In this example, calling `addEmUp nums` will yield `15 :: Int`.  First, I defined a `[Int]`, that is, a list of `Int`s, called `nums`.  Then I created a function `addEmUp` which is really just an alias for a specific `fold` - notice how it doesn't do anything else, just specifies which arguments to use with the fold.  That's why the type signature for `addEmUp` is a lot simpler - it only takes the `[a]` collection, in this case `nums`.  So our `a` is `Int`.  The first argument, the prosessor, is `(+)` - just the addition operator.  Operators are functions, and this one takes in two values and produces a third.  Let's compare to our expected tpe: `a -> r -> r`.  Well, in this case, `a` is `Int`, and also we want an `Int` at the end, so we can substitute it in for `r` too.  If you add an `Int` to an `Int`, lo and behold, an `Int` will pop out.  So our processor, addition, has type `Int -> Int -> Int`, which fits!  Remember, it's totally fine if `a` and `r` or any two unspecified types are the same, we just note that they don't *have* to be.

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

### The `Show` Must Go On

That digression got a little nuts, but now we're armed to dive in to this bigger, messier fold.  We know its going to do the same basic type of thing as `addEmUp`.  So the first thing to look for is those three elements we know we'll need: the processing function, the starting value to accumulate in to, and the collection to process.  As a reminder, here's the first line of our `show` definition:

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

The first part of the definition is `(++)` is concatenation.  There's a clue to where our other type goes - we're going to have whatever we're doing with `a`, the active cell, on one side, and it's going to get concatenated to the accumulator.  That makes sense - it's kind of like adding an `Int` to the accumulator.  The accumulator will now hold information from both operands.  What on earth are we adding, though?

I've grabbed the `bool` function from `Data.Bool` and it's really just some control flow.

TODO COME BACK WHEN YOU UNDERSTAND YOUR CODE - fmap showCell happens first, then we fold over the *result* of fmap showcell as our init container with spaceEachThird

### Gathering Input

Whew!  That `print board` line turned out to be a little intense.  Luckily, the beauty of programming is that we've only gotta tell it how once, right?  The next order of business is going to be asking the player where they'd like to play.  The next two lines are familiar enough:

```haskell
putStr "Your move: "
hFlush stdout
```

`putStr` is going to simply send its input to stdout, and `hFlush` flushs the stdout buffer (ensuring the full string is printed out) before advancing to the next instruction.

Immediately following, we've got `n <- getLine`.  This just says we should wait for stdin, and when we get a `\n`, store the contents of the line entered to a local binding `n`.  We can do this inside of our `do` block so that the value passed in is available to the rest of the function.  As soon as the user enters a line of text and hits enter, we'll move on.

Now we get to the big `case` statement of `runGame`.  This is where we appropriately dispatch an action based on what the user entered.  This control flow construct is not at all dissimilar to a `switch` in other languages - more similar to a `match`, actually, in terms of expressive power.  We're going to match the value we just received from the user against a few patterns to see how to handle it.

We'll start with the outer layer:
```haskell
case n of
    [c] ->
      -- do some really awesome stuff with our single char
      -- ...
    _   -> putStrLn "Only one digit allowed!"
```

This syntax just checks if our input `n` consists of a single character.  TODO EXPLAIN THIS A LITTLE BETTER.  If it does, we'll keep it and do stuff, and if not we'll lightly admonish the idiot at the keyboard with a `putStrLn` call - this is a `putStr` that includes a trailing newline.  I mean, honestly, don't you know how to play TicTacToe?   This is the last line of our function - but it's all wrapped up in a `forever`, so if that does happen we'll just take it again from the top of `runGame` until the user gives us something we can work with.  Should I have optimized away the extra `gameOver` check in this case?  Most definitely.  Will I?  Highly unlikely.

If, however, the user complied and only passed in a single character, we still have a little work to do:

```haskell
if [c] `elem` map show [(1::Integer)..9]
then do
  -- We've got us a digit!  Do some awesome stuff with it
  -- ...
else putStrLn "1-9 only please"
```

`if` in Haskell works more or less how you might expect, with the caveat that it's an *expression*, not a *statement* - that is, the entire `if` block must reduce to a value  (remember, `IO ()` counts - it's just a value of type "doing some IO" with nothing being passed back into the function).  You cannot have an `if` without an `else`.  Aside from that, though, it's as expected - you pass in a predicate and if that predicate evaluates to `true`, we'll execute the `then` block, and if not, we'll use `else`.  If you have more than two cases, I recommend `case` over `if`.

The first thing to check is whether or not the single character we now know we have is a valid play or not - it must be a digit from 1 to 9, not a letter or a bit of punctuation or anything.  The first line defines this predicate using the `elem` function, which checks if the first operand of type `a` (anything) is an element of the second.  Most functions in haskell are *prefix* in that the function names come first followed by the arguments.  To use a function of two arguments more like an *infix* operator between two operands, you can wrap it in backticks.

This predicate is asking if our char input is a digit from 1 to 9, and employs a handy little trick to do so.  We can't simply ask if `"1" == 1` (lookin' at you, JavaScript) because one is a character and the other is an integer.  So first we need to get a list of valid chars `["1", "2", "3", "4", "5", "6", "7", "8", "9"]` to compare against.  A quick way to build this array is our good friend `show` - if you recall, this is how we convert a type into something we can print out on the screen.  In the case of an Integer, this means turning it into a char representation first to send to stdout.  We can `map` the `show` function over a list `[1..9]` and it will perform that conversion for us for every element.  We're using the range operator `..` to construct our list, and by tagging the first element with a concrete type `1::Integer` we ensure each element we're mapping `show` over is an integer to begin with.  Pretty handy!

So, with the predicate out of the way, we've now determined whether or not the input stored in `n` is a single digit.  Our else statement looks like the previous - print out a quick error telling the user how exactly they were dumb, and that's it - head back on up to the top of `runGame` and hope this chucklehead learned their lesson.  If it was a digit, however, we can move on to one final nested `if`:

```haskell
 then do
          let n' = digitToInt c
          if openCell board n'
          then handleInput board n' >>= compTurn >>= runGame
          else putStrLn "That's taken!"
```

I included the top `then` line to show that we open a new `do` block - `then do` isn't a special syntax, it's just a `do` inside a `then` :)

First, we grab a local binding of the integer version of our input `c` - BEN SHOULD YOU JUST USE `show` HERE FOR CONTINUITY? - and store it as `n'`.  Then we have one final predicate - before we can go thrusting the play's move onto the board, the Laws of TicTacToe state that you can only make a move on a square if it's empty.  No playing on top of each other!  Here's `openCell`:

```haskell
openCell :: Board -> Int -> Bool
openCell (Board b) n = isNothing $ b !! (n - 1)
```

This is a function that takes two arguments, a `Board` and an integer, and returns a boolean like a predicate should.  We're going to pass in the full board and a specific square, and `openCell` will tell us if the space is already occupied.  In some languages it's good style to name predicates like this something that's obviously a predicate, like with a `?` at the end or a `_p` or something.  I have not done so here, sue me.  TODO LOOK UP HASKELL STYLE GUIDE.

Thanks to Haskell's operator love affair, this looks a little more complicated than it is at first glance.  We've seen `$` before - it's function application.  The other funky operator is `!!` - this is just a list subscript.  In a more C-like language, we might have written this exact logic something like `isNothing(b[n - 1])`.  That is, we're asking for the `n - 1`th element of our inner board list `b` (named so via destructuring in the definition: `(Board b)`), and passing it to `isNothing`.  `isNothing` we brought in at the top from `Data.Maybe` and itself is just a predicate which is true if the `Maybe a` passed in is a `Nothing`, as opposed to a `Just a`.

We initialized our board to a list of `Nothing`s, so the first time through this loop, any digit we pass in is going to come up clear.  If there had been a `Just Human` or `Just Computer`, we'd hit the `else` block, yell at the user a little (we JUST printed out the board state, they're called EYES, use 'em), and take it from the top.

HOWEVER!  If `openCell` comes back `true`, we've finally done it - we've ensured the value passed to `n` from stdin is a value we can meaningfully use as the player's next move.  Hot digggity dog!

THe full `then` block reads: `handleInput board n' >>= compTurn >>= runGame`.  This is three separate function calls wrapped up together with `>>=`, which is read `bind`.  `>>=` is going to allow us to pass the result of a monad (that second word in the type) as the input to a subsequent monad in the chain, while still keeping it wrapped up in the proper context, in this case `IO a` or specifically `IO Board`.  We want to do stuff to that `Board` without losing the `IO` wrapping.  I think this is clearest through example, and luckily we're working through an example right now!  The first function call is `handleInput board n'`, so let's unpack that first.

### Making a Play

I bet we can work out the type of `handleInput` from the call.  `board` is easy - it's a `Board`, and `n'` is our newly converted integer from stdin.  So we know this will be a `Board -> Int -> something`.  What, though?

Well, we know we're inside an `IO` monad, and in a series of calls chained together with the monadic `>>=`.  So it's a safe bet this will be another `IO a`, that is, an `IO` monad with some type as a result.  And if we look down the chained call, we end things up with a call to `runGame`.  We've already looked at `runGame` (we're inside of it RIGHT NOW), so we know it's a `Board -> IO ()`.  We're calling it here with no argument, but from the type know it will need a `Board`, and we're passing a monadic result through a chain of functions - so it would follow that the type of each step *must* be `IO Board`.  Lo and behold:

```haskell
handleInput :: Board -> Int -> IO Board
handleInput board n = do
  let b = playCell board n Human
  checkWin b Human
  gameOver b
  return b
```

Just as expected!  You're super good at this.  In the body of the function, we're opening another `do` block, and as our first step creating a new binding `b`.  Time to finally examine `playCell`:

```haskell
playCell :: Board -> Int -> Player -> Board
playCell (Board b) n m = Board $ take (n - 1) b ++ [Just m] ++ drop n b
```

From the function call, we expected a type like that - 3 arguments.  We also now see it will give us back a `Board` to store in `b`. The only type we haven't seen used much yet is `Player` - but we know all about that already from discussing `Board`!  It can be a `Human` or a `Computer` and nothing else, and in this case we're processing the human's input - so we just pass in `Human`.  The fully qualified type is simply `Human :: Player`.

In the argument list, we've destructured the `Board` again to access the list of cells inside and assigned letters to the other two[7].

Now, in a C-style language, you'd probably at first approach this task of adding a play to the board by indexing into the list and changing the value inside.  In Haskell, that's a big nope.  Remember when we discussed purity?  That would involve *changing the state of the world outside of the function* - namely the `Board`.  If we did it this way, this function of have wildly different and unpredicable results based entirely on the state of the `Board` when it was called, which is terrifying.  We cannot definitely look at that function and tell you what *exactly* it will do.  If that's not terrifying to you, it SHOULD be.  But, of course, this would be a dumb game[8] if nothing was ever allowed to change.

The way we get around this restriction in any functional language, not just Haskell, is to not attempt to change anything at all.  Instead, we're just going to construct a *brand new* `Board` based on the previous one.  Haskell is garbage-collected, so the old iteration will be automatically dropped by the runtime, no need to call any sort of destructor or free the memory yourself.  That way the game as a whole can continue in a new state and we haven't broken our purity restriction.

I do this using the super handy `take` and `drop` functions, which return sublists - again, these are brand new lists, leaving the input list untouched.  `take` returns the specified number of elements from the front, and `drop` returns the end of a list beginning at the index specified.  So in `playCell` I just `take` the cells up to but not including the cell specified, and at the end we'll put on the cells after the cell specified.  That only leaves the single cell in question.  Because the `Board` requires each cell to be a `Maybe Player`, we can wrap our `Human :: Player` inside a `Just`.  We then put it in brackets to make a single element list, and use `++` to concatenate all of our sublists together, and wrap the new list up in a new `Board`.  The end result is a `Board` just like the last, except the cell we passed in as an argument has a `Just Human` now instead of a `Nothing`.  Everything else is a direct copy.

This way, for the same inputs we can always guarantee the same outputs.  The current state of the `Board` is passed directly into the function, which allows us to take action on it, and we know exactly what will happen given all the inputs we've got.  This makes reasoning about the flow of logic in Haskell code almost trivially easy in cases that become very convoluted otherwise.

### Winners Only, Please

Now that we've stored our shiny new Board with one cell updated, we've got to see how well we did.  The next line of `handleInput` calls out to `checkWin`:

```haskell
checkWin :: Board -> Player -> IO ()
checkWin board@(Board b) m =
  let
    bi = withIndicesFrom 0 b
    plays = map fst.filter ((Just m==) . snd) $ bi
  in
   when (foldr ((||) . flip isSubsequenceOf plays) False winStates) $ do
     -- End the game!
```

Ok, this is a little bigger.  It's a function of two arguments returning an IO monad, which (I really hope) makes sense by now.  This monad isn't returning anything (note we havent stored this function callt o a binding, we just called it), so `IO ()` is appropriate.  This will just do some IO, and will be responsible for terminating the process if we find a win.

The `let...in` syntax is a way of creating function-local bindings, not unlike `where`.  In fact, they can often be used interchangeably, and the difference is subtle: `let...in` is an expression, which can be used anywhere at all that expects an expression (kinda like `if...then...else`), whereas `where` is a syntactic construct that only come after a function body.  I'm not going to get into the subtlies, see the [Haskell Wiki](https://wiki.haskell.org/Let_vs._Where) for a more thorough discussion.

Anyway, before diving into the endgame checking, we're going to set up some computed local bindings to make our life a little easier.  The first one calls a helper function:

```haskell
bi = withIndicesFrom 0 b

withIndicesFrom :: Int -> [a] -> [(Int, a)]
withIndicesFrom n = zip [n..]
```

This is really just a handly alias to attach a more domain-specific semantic name to a general function `zip`.  Given two collections, `[a]` and `[b]`, `zip` give you back a single collection `[(a, b)]`.

This alias just defines the first term of the zip.  You might notice the argument list doesn't match up with our type declaration - we're expecting two arguments, an `Int` and some list, but only have one below.  This is an example of the "eta-reduction" I mentioned earlier - the second argument, namely the list to zip with, appears last in the argument list and the function body, so we drop it from both.  The fully specified version would read:

```haskell
bi = withIndicesFrom 0 b

withIndicesFrom :: Int -> [a] -> [(Int, a)]
withIndicesFrom n cs = zip [n..] cs
```

We're using the argument to define the beginning of a range `[n..]`.  to zip with, which will have the effect of attaching an index to each element in the list.  That's all.

#### A Brief Digression on Laziness

This function brushed up on another super-cool property of Haskell that I haven't made much use of in this program, but is too neat to just blow by.

You may notice that the seemingly-innocuous expression `[n..]` doesn't specify a top value.  What we've done, then, is defined an *infinite list*, starting at `n` and just going and going.

In most programming languages, this is quite obviously not ok.  The process would drop everything else and build this infinite list until it blows the stack and crashes, resulting in a pretty shit game[10].  Haskell, on the other hand, employs *lazy* evaluation semantics.  When the compiler passes through, it's perfectly content to leave that `[n..]` alone until it needs to begin the expansion - and even then, it only expands *as-needed*.  In the case of `withIndicesFrom`, the argument we pass it will be finite, which if you need a refresher, is not as big as infinite.  When we hit the last value of that collection to pass into `zip`, then we're good to go - no need to keep drilling our way through `[n..]` for indices we won't use.  Haskell just leaves it wherever we are and moves on.

This is a pretty incredible property that allows for all kinds of patterns not possible in strict-evaluation languages, but does have the side effect of making some perfomrance characterists difficult to reason about.  It's a good thing to keep in mind when writing Haskell.

### Surveying the damage

So, now we've saved as `bi` a version of the `Board` we're working with zipped up with indices - instead of, e.g., `[Nothing, Just Human, Nothing...]` we have `[(0, Nothing), (1, Just Human), (2, Nothing)...]`.  We're going to use this in our next `let` binding `plays = map fst.filter ((Just m==) . snd) $ bi`.

This line is a little token-soupy, but we're intrepid as heck.  It's a call to `map`, and the collection we're mapping over is the newly defined `bi`, so all that junk in the middle must be our mapping function.  Let's see if we can untangle it.

This function has opted for concision via the `.` composition operator we saw up in our `Show Board` instance, at the cost of readability.  This one actually has a composed function inside a larger composed function, for extra goodness.  These are easiest to read inside-out (Lisp-ers know what's up).

The first action that happens to `bi`, our indexed `Board`, is `filter ((Just m==) . snd)`.  The filter function first calls `snd` on each element, returning just the second element of the tuple:

```haskell
snd (1, Just Human) == Just Human
```

Then, we compare it to the value passed in as `m` - remember when we called the function, it looked like `checkWin b Human`.  We're specifically checking if the Human player won the game with their latest play.  This is why we derived the `Eq` typeclass up in the `Player` declaration - this check wouldn't compile otherwise.  So `((Just m==) . snd)` will return true on a `(Int, Maybe Player)` if the second value is `Just Human`, and false otherwise.

Now that we've pared down `bi` to only the cells that have been played, we pass that whole result into `fst` - that is, grab the first value of each tuple.  These are our indices.

The end result that's stored in `plays` is a list of the indices from 0 of all of the places the Human has played.  For example,  `[(0, Nothing), (1, Just Human), (2, Just Computer), (3, Nothing), (4, Just Human)]` will come back with `[1, 4]`.  Neat.

Now that we've got our packed-up Human plays, we can check to see if that constitutes a win.  The main body of the function, following the `in`, is another `when ... do` shindig like we saw back in `gameOver`.  This monad will execute its body under this condition, and otherwise its a no-op.

How about that condition, then?  Let's see: `foldr ((||) . flip isSubsequenceOf plays) False winStates`.  Aha, it's our good old friend `foldr`.  I unabashedly love this function.

True to form, we've got three arguments: a transforming function, an initializer, and a collection.  We've looked at two folds before - the trivial example used an `Int` as an initializer that we added numbers to, and the the code from the game used a collection (that we pre-built).  This time around it's simple a `Bool` - `False`.  That's is a-ok too as long as your transforming function returns a `Bool`!  It can be any type at all.  That means this whole fold will return a `Bool` - by definition, the fold always returns the same type as the initializer: `(a -> r -> **r**) -> **r** -> [a] -> **r**`.  And that's what we want, because `when` expects a predicate.

Before picking apart the transformer, let's look at `winStates` - the collection we're folding over.

```haskell
winStates :: [[Int]]
winStates = [[0, 1, 2], [3, 4, 5], [6, 7, 8], [0, 3, 6], [1, 4, 7], [2, 5, 8], [0, 4, 8], [2, 4, 6]]
```

This is pretty simple - it's just a list of lists.  This is admittedly not an elegant way to handle this problem, but TicTacToe is simple enough that it's feasible to simply hardcode all the possible winning configurations.  This is a list of lists of `Int`s (`[[Int]]`) just containing all the indexes that are in a row.

Finally, the transformer: `(||) . flip isSubsequenceOf plays`.  We know this function will be of type `(a -> r -> r)` - filling in the concrete types this becomes `([Int] -> Bool -> Bool)` - out initial collection is a `[[Int]]`, a list of lists of `Int`s, so each time through we're checking just one of these sublists and returning true or false.

The workhorse function I chose is the aptly-named `isSubsequenceOf`, imported with care from `Data.List`.  It returns true if the elements of the first list appear in order (but not necessarily consecutively) in the second list.  The docs helpfully note that this function is equivalent to calling `elem x (subsequences y)` - true to form, the standard library is building useful abstractions by composing smaller abstractions!  I actually came across this library function in the course of googling a problem I come up against trying to implement it myself.  I don't remember the specific nature of the problem, but Haskell's standard library is as incredible as the language itself - so don't forget to look through it for functionality you need before falling down the wrong rabbit hole.

#### Typclass constraints - a digression

According to [Hackage](https://hackage.haskell.org/package/base-4.11.1.0/docs/Data-List.html#v:isSubsequenceOf),  this function has type `Eq a => [a] -> [a] -> Bool`.  This signture has one syntactic element I haven't touched upon yet - that first part, `Eq a =>`, is a *typeclass constraint* on `a`.  I've been using `a` as a stand-in for "any type" over the course of this article.  This syntax lets you more precisely define what sorts of types are ok - unlike a fold, `isSubSequenceOf` only makes sense to call on lists with elements that can be compared to each other.  This stands to reason - it's going to have to check each element in one list against the other.  This is Haskells system for *ad-hoc polymorphism*.  If the types involved do not have instances of the typeclasses specified, either derived or hand-implemented, this won't compile.

### `flip`ing out

The last unfamiliar part of this function composition is the word "flip".  This is a simple but useful function that just switches the order in which the arguments are expected.  The way we're calling it in our transformer function, `isSubsequenceOf` receives our `plays` list first, and then the element of `winStates` the fold is currently processing.  However, we want it the other way around - to tell if we've won, we want to check if the winState is a subsequence of all the plays this player has made.  You can win with other non-lined-up plays on the board, they're just irrelevant.  "flip" just swaps the positions of the arguments so we get the logic we want!

Finally, we compose that result with the simple operator `(||)`.  This is usually used infix, e.g. `true || false`, but we can use it as a normal prefix function as well bywrapping it in parens.  One value it receives will be the result of our `flip isSubsequenceOf` call, and the other?  Why, that's our initialized `Bool`!  By chaining together all these calls with a big 'ol `OR`/`||`, this transformer will return `True` for the whole collection if any one of these iterations comes back `True` (meaning `plays` contains one of our `winStates`), or remain `False` as we initialized it.

If it was `False`, we didn't win - `checkWin` has nothing else to do.  The code inside the block doesn't execute, we have `()` to return, and control passes back to the caller.  If we *did* win:

```haskell
print board
putStrLn $ show m ++ " won!"
exitSuccess
```

Now we can finall hop back in and finish up `handleInput`:

```haskell
gameOver b
return b
```

If we've gotten here, it means `checkWin` didn't find a winning board configuration, so before we move on we call `gameOver` again to see if this play resulted in a draw, and if not, we `return b`.  `return` is a little different than you're used to - it specifically means to pass back the value given wrapped up in the `IO` monad - this monad is of type `IO Board`.  This is how we pass the result back to the main `runGame` loop, having determined that this play didn't end the game in either a win or a draw.

### RNG Rover

We're nearing the end of the road, here - if you're still with me, I'm seriously impressed!  We've just got one last part to pull this together - what's a game of TicTacToe without a steely-eyed, calculating oponent, ready to squelch your every plan?

Well, we're not going to find out here because my computer player is real dumb and plays by dice roll.  It could be fun to try to make a smarter one - I'm leaving that as an exercise to the reader (read: too lazy to do it myself).

Rewinding a little, we entered `handleInput` inside this larger clause: `handleInput board n' >>= compTurn >>= runGame`.  So far, we've updated the world state according to human input, made sure there's still a game going on, and received the new `Board` to work with.  No we're going to pass that brand new world state into `compTurn` via `>>=`, which as we discussed will allow the `Board` to be passed without losing the `IO a` context it started with.  This means we should expect `compTurn` to take a `Board` as input and, because we're in the middle of a `>>=`/`bind` chain, return an `IO Board`:

```haskell
compTurn :: Board -> IO Board
compTurn board@(Board b) = do
  let options = filter (isNothing.snd).withIndicesFrom 1 $ b
  r <- randomRIO (0, length options - 1)
  let b2 = playCell board (fst $ options !! r) Computer
  checkWin b2 Computer
  return b2
```

Ok.  So, this function is mostly familiar by now.  We see our `IO Board` return type, we're destructuring the argument to get at the list of cells as `b`, we've got our old friend the `do` block - nothing too surprising.

The first line creates local binding `options`, which is going to be the result of `filter`ing our list of cells.  Filter is like `map`, except it returns only the elemnts of the input collection for which the predicate is true.  Again, aptly named.  Let's take a look at the predicate:

```haskell
(isNothing.snd).withIndicesFrom 1
```

This function is composed from parts we've seen before.  First, we're going to zip up our cells with indices starting from 1 (spoiler alert, because that's what `playCell` wants as input).  Then, we're going to pass that to the composition `.` of `snd` and `isNothing`.  Hopefully this starts to feel a little more readable by now - in English, this `filter` will have the effect of storing to `options` a list of 1-indexed cells that contain a `Nothing` - anything that's a `Just Human` or `Just Computer` will be omitted.  These comprise the possible cells the computer can choose.

In the next line, we introduce the randomness.  This ends up looking similar to how you'd do this in the language of your choice - `randomRIO` from `System.Random` takes a range and will give you a pseudo-random number in that range.  We're using the length of our `options` list, and storing the result to `r`.

Now, we've got to actually make the change.  This is done with `playCell` again - the differences being that instead of user input, we're using `!!` again to index into `options` with the random number we just grabbed, and we're passing in `Computer` instead of `Human`.  Now, `b2` holds our new `Board` with the random play applied.  With that taken care of, we can see if the computer managed to win the thing with `checkWin`.  If it did, `checkWin` will handle ending the game for us, and if not, we `return` again.  No need to call `gameOver` again here, because `runGame` does so first - and our pipeline `handleInput >>= compTurn >>= runGame` is sending us right back up there.

### The Thrilling Conclusion

We did it!  I'm all out of code to unpack.  `runGame` has everything it needs to alternate human turns and computer turns until somebody wins or we run out of spaces.  Haskell ain't no thang :)

Th-th-th-that's all, folks![9]

### Footnotes

TODO check yer numbers, foo'

[1] Kinda nuts that I had like 12,000 words to say about 94 lines of code

[1] I hesitated to say [Great Good](http://learnyouahaskell.com/) because that's pretty wishful thinking in my case - Hopefully Not For Nothing is more accurate.  This is a great book nonetheless if you're not ready to shell out $60 for the First Principles book.

[3] I do not mean this figuratively

[4] I had to fight down the urge to start this next sentence "The IO Monad can be thought of as a..."

[5] Haskell [Prelude](https://hackage.haskell.org/package/base-4.11.1.0/docs/Prelude.html#v:-36-)

[6] We know it's an `IO ()` because we're inside a `do` block in an `IO ()`, it performs IO of its own, and it doesn't have any value coming back up.  It just exists to print the value to the console.  So when the compiler comes hungrily munching through `runGame`, `print` just evaluates to `()`.

[7] This was actually one of my bigger beefs with Haskell as a beginner.  In other languages, I've gotten used to choosing descriptive (but still short) names for any bindings I create.  It seems, though, that Good Haskell Style involves lots and lots of single-letter stand-ins, which goes against every instinct I have.  I feel this inhibits readability for little gain - Haskell is terse enough as it is.  I'd be interested to hear thoughts about this for more experienced Haskellers.

[8] Well, more dumb - TicTacToe isn't exactly a groundbreaking paragon of high strategy to begin with

[9] If this is a copyrighted phrase a) I'm sorry and b) come at me, bruh

[10] Yeah, yeah, it already is, save it
