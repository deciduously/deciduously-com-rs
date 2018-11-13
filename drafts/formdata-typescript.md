# FormData in TypeScript

I'm trying to get in the habit of writing a post about anything that takes me more than, say, 15 minutes of trial and error and Google to get right.  Chances are I'm not the first nor the last to hit any particular roadblock.

This is both a tutorial and a request for help!

*edit*: @foresthoffman provided the help I needed!  I've updated the post to include the fix.

I'm working on a small toy app that needs a few controls to specify how to draw something to the canvas.  A `<form>` seems like the natural choice for this, but using the data from TypeScript proved a little bit tricky.

The MDN page on [`FormData`](https://developer.mozilla.org/en-US/docs/Web/API/FormData) suggests using syntax like the following:

```javascript
const form = document.querySelector('form')
const data = new FormData(form);

for (const pair of data) {
    // cool stuff
}
// OR
for (const pair of data.entries()) {
    // cool stuff
}
```

I had no luck with these.  Using `for..of` causes TypeScript to complain that `data` is not an iterator (I think [it is](https://developer.mozilla.org/en-US/docs/Web/API/FormData/entries), or at least in JS it can be used like one with `for...of`), or that `data` has no such property `entries`.  This makes a little more sense - it doesn't yet in all environments.  I tried tweaking my `tsconfig.json` to target `esnext` but that didn't do it, and I'd rather keep that set to `es5` anyway.  Switching to use `for..in` on `data` does what you'd expect, really - it enumerates all of the methods available on `data`:

```javascript
const data = new FormData(form);
for (const entry in data) {
  console.log(entry);
}
/*
get
getAll
has
set
entries
keys 
values
forEach
*/
```

Neat, I guess, but not what I'm looking for!  Frustratingly, `entries` appears.  Why can I not use it?

It turns out the fix for this is subtle - you need to specifically tell TypeScript you're going to be using this method by adding `dom.iterable` to your `tsconfig.json` - it's not automatically brought in with "dom":

```json
"lib": [
  "dom",
  "dom.iterable",
  "esnext"
],
```

Now you can `for (let entry of data.entries())` to your heart's content!  That's still not as concise as it could be, though - in JavaScript you can just write `(let entry of data)`.  We can allow this pattern in TypeScript by adding one more line to `tsconfig.json`:

```json
"downlevelIteration": true,    
```

This compiler option "provide[s] full support for iterables in 'for-of', spread, and destructuring when targeting 'ES5' or 'ES3'."  Now our code can match the JS exactly!

I'm leaving my workaround for posterity, because in some simple cases I'd prefer to skip the iterator and do it this way anyway.  It simply doesn't iterate at all, it looks for what it needs.  As an example, here's part of the form in question:

```html
<form>
        <fieldset>
            <legend>Choices</legend>

            <input type="radio" name="choice" id="choice1" value="choice1" checked>
            <label for="choice1">Choice 1</label>

            <input type="radio" name="choice" id="choice2" value="choice2">
            <label for="choice2">Choice 2</label>
        </fieldset>
        <button type="submit">Do The Thing!</button>
</form>
```

To get at the user's choice, I'm using code like this:

```typescript
const form = document.querySelector('form')!;

form.onsubmit = (_) => {
  const data = new FormData(form);
  const choice = data.get('choice') as string;
  doCoolThing(choice);
  return false; // prevent reload
};
```

A few TypeScript-y things to note - I'm using the `!` operator at the end of the `querySelector` call.  This is the non-null assertion operator - `querySelector` returns a value of type `Element | null`.  I prefer to keep `strictNullChecks` on, so TS doesn't enjoy me trying to operate on `form` as if it were an element - this operator tells the compiler that *I* promise there will be a form to select and it won't return null.

Also, `FormData.get()` returns a value of type `string | File | null`.  This is another case where I've quite literally just written the form myself - I *know* it's gonna be a string.  I'm using `as` to cast to the proper type.

Finally, I return false to prevent the page from actually reloading - the re-draw to the canvas happens inside `doCoolThing`, and if the page reloads it'll disappear along with the form data!  I'm not sending anything to a server, just using the user input locally on the page.

This does do the trick - I can just grab the the form data I want one at a time without using the iterator to configure the output.