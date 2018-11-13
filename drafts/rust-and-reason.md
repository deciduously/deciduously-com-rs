# Rust + ReasonML: A Beginner's Love Story
## Baby's First Full-Stack App

I've done it - I've made a Thing.  I identified a problem I was having, designed a solution, and wrote a bunch of code that does the thing I wanted well enough for other people to use it.  I've got, like, *six* whole users now.

I know that's pretty much the name of the game with this craft and all y'all do that on the daily but it's a bit of a big deal for me.  The chasm between being able to complete exercises, tutorials, and little toy terminal apps and a full-fledged application like this is a large one even if the final product is very simple in scope.

Generally, the advice from the wise especially when learning is to gravitate towards tools which are battle-tested and widely used.

I'd like to make a counter-argument for trying the weird stuff anyway - I believe choosing Rust for my backend and ReasonML for the frontend allowed me to spend more time on the problem than the tooling, gently guided me towards better practices, and increased my understanding of the some of the concepts at play all while setting me up well to transition to more mainstream tools without much lead time.

I'm not going to get into too much detail but just want to mention some parts of this project that I believe were easier because of these choices.

The project is hosted on GitHub - it's called [mifkad](https://github.com/deciduously/mifkad).  It's designed to replace a handwritten process for tracking the attendance of a small school and generating rosters for the "extra hours" portion of the day based on that attendance.

## The Backend

I could not be happier with [actix-web](https://actix.rs).  I had already been playing around with Rust for a bit when I stumbled upon it, and had a few endpoints working as expected within minutes from just reading the website.  Easy to configure, flexible as heck, runs blazingly fast, and because it's Rust I get a small portable binary with no runtime dependencies - I'm sold.  However, while Rust and actix are great, what really struck me about building this backend was how the *rust compiler* taught me how to write asynchronous code.

The trickiest part of building this was ensuring it would work between sessions - there would need to be multiple users throughout the day, from different workstations.  I decided to just persist the whole state to a shared network storage on each click, but needed to ensure writes don't conflict.

Not long ago that sort of problem would have flown way over my head.  Thread-safe code is for smart, experienced people who know what they're doing!

Turns out, the Rust compiler can basically do it for you.  I'd used Reference Counting types so I vaguely knew something called an [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html) existed.  I knew I'd need some sort of mutex, so I cracked open the standard library docs and found [`RwLock`](https://doc.rust-lang.org/std/sync/struct.RwLock.html), which sounded about right.

I then...guessed:

```rust
pub struct AppState {
    pub school: Arc<RwLock<School>>,
}
```

I replaced my regular old School type with the above, just to see if I was on the right track.

Not only was I on the right track - that was pretty much *it*.  I rewrote my handlers to grab the right locks before reading and writing to and from the app state, and followed the actix docs to switch all my handlers to return Futures (which is a built-in feature of the framework - this took about ten minutes), and then fixed all the `rustc` errors.

It just friggin' *worked*.  That's *nuts*.  And now I'm no longer scared of using concurrency primitives in my Rust code.

## The Frontend

React.js in 2018 is a beast.  I mean that lovingly - the ecosystem is huge and powerful and has something for every need that you can pick and choose from.

Starting a brand new React project, though, having never tried any of it before?  Daunting, to say the least.  No matter what choices you make there's a nagging feeling there's something better on the next search, even though in most cases whatever you're using is just fine.  There's something about an overwhelming amount of choice available that can cause you to just freeze, or build the same beginning of a project over and over again with slightly different libraries and never finish.

Choosing ReasonML allowed me to completely skip that entire process without forgoing control.  While I know and appreciate tools like `create-react-app`, trying it myself left me with a bad taste in my mouth.  It's very magical, hiding most of what it's doing to provide so much power from the user until you run `eject`.  After ejecting, though, you're left with a *lot* to sift through - too much to learn right off the bat.  I prefer to build up this sort of thing myself, ensuring I actually understand each component, so that tool didn't work for me.

The basic app you get via `bsb -init` was very no-frills.  I fully understood everything I looked at in my brand new folder and was able to start iterating immediately.  Not knowing Reason or OCaml that well didn't end up being an issue - ReasonReact maps pretty closely to regular old React!  After perhaps a day of slow progress I wasn't running into syntax errors anymore and knew where every new line of code should fit.

Not to mention the fact that I didn't need to attach a state management library - it comes with `reducerComponent` built-in so you can have a Redux-ish action dispatcher instead of setState but don't have the boilerplate associated with an app-wide Redux store.  You just set up a sum type for your actions and then use them from a `reducer` method available on your `reducerComponent` like any other React lifecycle method and it's just all so easy to use.

The biggest benefit, though, was the compiler.  It's *fast* - you'll never beat it.  TypeScript and Flow aren't even close.  It's built around an industry giant of type inference so you get amazingly helpful error messages pointing you towards exactly what you're doing wrong.  What a phenomenal way to learn a new tool, with training wheels attached - almost everything I learned while building this I'll be able to carry over to a "real" React app, but it's like having a dedicated mentor over your shoulder calmly pointing out every stupid thing you do as you do it.  Eventually, you stop doing those things!

I truly felt like ReasonReact got out of my way and let me just write the application.  It made sense to me to set up types to mirror those used on the backend.  It was trivial to deserialize the json responses into ReasonML data structures.  I loved how all of my props were fully typed by default.  The generated output is just plain old JavaScript that appears alongside your code, so it's not complicated to figure out how to bundle your final app.  There was just no friction anywhere, and at no point did I run into a question that wasn't answered clearly on the Reason or ReasonReact websites.

## Your turn!

Granted, I wasn't trying to do anything fancy here - but I did brush up on the interop story and even that was easy.  At no point during this project did I feel I was fighting my tools - it was always the opposite.  I do not have that experience using the mainstream tools I "should" be using, by a long shot.  I think I got this done faster and better than I would have with plain JS throughout, and learned more to boot.

Does anyone else have an experience like that with a tool they've used?  Any recommendations for some neat things off the beaten path that improved your quality of life?  I'd love to hear about it!