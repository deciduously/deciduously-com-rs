# Higher-Order Functions in Rust

Rust is an imperative language but it provides many tools in the standard library which adhere to a more functional style, like the [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) trait and its methods like `map`, `for_each`, and `filter`.  This is a quick run-down of how to define your own higher-order functions in Rust which can both take closures as parameters and return closures in such a way that you can use the two together.

*edit* @theodesp rightly pointed out that these examples do not follow a purely functional style - I'm mutating the input in place.  I wrote it this way because the pure solutions are well covered in the standard library and this use case is less clear, but I've updated the [Rust Playground](https://play.integer32.com/?version=stable&mode=debug&edition=2015&gist=8639706958a3b51389474b328331d9d8) with some hand-rolled pure implementations as well for comparison.

To demonstrate these we're going to work with a 2D grid of numbers:

```rust
type Grid = Vec<Vec<i32>>;

fn prepare_grid(rows: i32, columns: i32) -> Grid {
    let mut ret = Vec::new();
    for _ in 0..rows {
        let mut row = Vec::new();
        for _ in 0..columns {
            row.push(0i32)
        }
        ret.push(row);
    }
    ret
}
```

This function will initialize the grid to all `0`s:

```rust
fn main() {
    let mut grid = prepare_grid(5, 5);
    println!("{:?}", grid);
}
// [[0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0]]
```

For whatever nefarious (?) purposes we might have in mind for our number grid it may be useful to act on a row at a time.  We can accept a closure as a parameter using a generic data type implementing the [`Fn`](https://doc.rust-lang.org/std/ops/trait.Fn.html) trait:

```rust
fn map_rows<F>(grid: &mut Grid, func: F)
where
    F: Fn(&mut Vec<i32>)
{
    for row in grid {
        func(row)
    }
}
```

Now we can, for example, increment the first element of each row:

```rust
map_rows(&mut grid, |row: &mut Vec<i32>| row[0] += 1);
    
println!("{:?}", grid);
// [[1, 0, 0, 0, 0], [1, 0, 0, 0, 0], [1, 0, 0, 0, 0], [1, 0, 0, 0, 0], [1, 0, 0, 0, 0]]
```

Returning a function is a tad trickier because of how Rust manages lifetimes.  What say you wanted to decide how much to increment that first value by.  You need to return a `Fn(&mut Vec<i32>)`.  Thing is, Rust needs to calculate the lifetime of that return function at compile time.  We have to explicitly tell the compiler that this function will live as long as the input parameter lives by using a reference and assigning it the lifetime `'a`:

```rust
fn make_incrementer<'a>(amount:&'a i32) -> Box<Fn(&mut Vec<i32>) + 'a> {
    Box::new(move |row: &mut Vec<i32>| row[0] += amount)
}
```

We're using the [`Box`](https://doc.rust-lang.org/std/boxed/index.html) pointer type so that `make_incrementer`'s return type's size is known at compile time, and using a `move` closure to ensure a new stack frame is allocated for the closure and we copy `amount` into it - thus escaping `make_incrementer`'s stack frame.

Using it with `map_rows` requires some changes:

```rust
fn map_rows2<F>(grid: &mut Grid, func: Box<F>)
where
    F: for<'a> Fn(&'a mut Vec<i32>) + ?Sized
{
    for row in grid {
        (*func)(row)
    }
}
``` 
We now have an explicit lifetime to deal with but generally it would need to be at least as long as the whole function.  In this case the compiler will complain because it only comes into scope when the closure is called inside the function, way down in our `for` loop - clearly shorter than the whole function.  The `for<...>` syntax is a feature called a Higher-Ranked Trait Bound, which tells the compiler to explicitly calculate the *minimum* lifetime to invoke our closure instead of defining it for the whole function with `map_rows2<'a, F>(...)`, circumventing this problem.

We also need to pass the whole `Box` in as our argument because all local variables *must* be `Sized` (though apparently unsized locals are in the works).  You don't need to take ownership - `&Box<F>` is fine.  This, though, will cause Rust to freak out because now we *don't* have a `Sized` value for `F` like before.  We needed to create one in order to get `make_incrementer` to compile but we've gone and undone it all by unwrapping it.  Luckily there's an escape hatch - by adding `?Sized` we can relax that requirement.  The only other change is getting at the actual closure with `(*func)` inside our for loop.

Now we can go *wild*:

```rust
map_rows2(&mut grid, make_incrementer(&2));
    
println!("{:?}", grid);
// [[3, 0, 0, 0, 0], [3, 0, 0, 0, 0], [3, 0, 0, 0, 0], [3, 0, 0, 0, 0], [3, 0, 0, 0, 0]]
// we already added 1!
```

Note: This example would work fine without the reference/lifetimes because our `amount` has type `i32`, which implements `Copy` - the move closure will just copy it for you.  This means you can omit them in the `map_rows2` too.  The reference is only strictly necessary if your parameter to your factory function is not `Copy` - say, a `String`.


All in all higher-order functions are definitely more cumbersome to use in Rust than in a garbage-collected functional language, but absolutely not impossible.  You just need to massage the borrow checker a little and make sure you know what exactly you're asking for!

Here's the full sample on the [Rust Playground](https://play.integer32.com/?version=stable&mode=debug&edition=2015&gist=8639706958a3b51389474b328331d9d8) to tinker with.