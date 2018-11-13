
# Reactive Canvas in TypeSript and Vue
## Or How I Learned to Stop Worrying and Love Custom Directives

Another in my "stuff I got stuck on" series!  The solution to this particular problem ended up being rather straightforward, perhaps to the point of obvious, but arriving at it was a roundabout process for me so here's hoping this is useful for someone anyway.

[Vue](https://vuejs.org/) provides [directives](https://vuejs.org/v2/api/#Directives) to hook your templates to your scripts.  For most cases these are sufficient, but controlling a `canvas` element requires lower-level DOM access.  `<canvas>` does not support `v-model`, so we need some other way to pass data into the element for rendering in such a way that it can keep itself in sync with our ViewModel.

As luck would have it, they'd *thought of that*.  With [custom directives](https://vuejs.org/v2/guide/custom-directive.html) we can make our own `v-something` for our template for which we can define our own behavior.

This code is written to fit in a project created by the Vue CLI 3.0 with the "TypeScript" option selected and class-style component syntax.  It should be simple to use with other configurations - the meat here is the directive itself.  See the doc links for the full syntax.

We'll work with a bare minimum Single-File Class-Based Component:

```vue
<template>
  <div class="rxcanvas">
    <span>{{ size }}</span>
    <input type="range" min="1" max="100" step="5" id="size" v-model="size">
    <label for="size">- Size</label>
    <p><canvas></canvas></p>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import Dot from "@/dot"; // defined below

@Component
export default class RxCanvas extends Vue {
  private data() {
    return {
      size: 10
    };
  }

  // computed property
  get dot(): Dot {
    return new Dot(this.$data.size);
  }
}
</script>

<style scoped>
</style>

```

Our Dot class just knows to draw itself given a Canvas element for a target:

```typescript
// dot.ts
export default class Dot {
    private readonly color: string = "#000";
    constructor(private radius: number) { }
    public draw(canvas: HTMLCanvasElement): void {
        // resize canvas to dot size
        const canvasDim = this.radius * 2;
        canvas.width = canvasDim;
        canvas.height = canvasDim;

        // get context for drawing
        const ctx = canvas.getContext('2d')!;

        // start with a blank slate
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        // find the centerpoint
        const centerX = canvas.width / 2;
        const centerY = canvas.height / 2;

        // create the shape
        ctx.beginPath();
        ctx.arc(centerX, centerY, this.radius, 0, 2 * Math.PI, false);
        ctx.fillStyle = this.color;
        ctx.fill();
        ctx.stroke();
    }
}
```

To get the behavior we want, i.e. a properly sized and drawn-to canvas in sync with our slider input, there's a little more logic that we want to fire on each change than simply bumping a number.  We've hidden all that logic inside our `Dot` class - `Dot.draw(el)` knows how to do everything it needs.  We just need this method to automatically fire whenever there's a change.

For starters, we can throw the directive right on to the canvas element in our template - we already know what data it's concerned with:

```html
<canvas v-draw="dot"></canvas>
```

In this example, our custom directive is called `draw`.  You could name it anything you like.  All directives are prefixed `v-`.  We're passing in `"dot"`, which is the computed property defined on our `RxCanvas` class.  This way whenever `size` changes, this computed property will create a new Dot with the correct size.

Custom directives are defined on the Vue component.  When using `vue-property-decorator`, you can place it in the decorator options:

```typescript
@Component({
  directives: {
    "draw": function(canvasElement, binding) {
    // casting because custom directives accept an `Element` as the first parameter
      binding.value.draw(canvasElement as HTMLCanvasElement);
    }
  }
})
export default class RxCanvas extends Vue {
    // data(), dot(), etc
}
```

...and that's it!  `binding.value` contains the actual `Dot` we get from our computed property.  This syntax takes advantage of a shorthand available for directives allowing us to condense the definition and not spell out each hook we use.  Acknowledging that in most cases users of this feature will want the same logic to happen on `bind` and `update`, we just define a function with our logic for the directive instead of an object containing hook functions and it gets that behavior by default.  Without using the shorthand, you'd define this logic as following:

```typescript
directives: {
    draw: {
      bind: function(canvasElement: Element, binding: VNodeDirective) {
        binding.value.draw(canvasElement as HTMLCanvasElement);
      },
      update: function(canvasElement, binding) {
        binding.value.draw(canvasElement as HTMLCanvasElement);
      }
    }
}
```

The `bind` rule is fired exactly once on component creation, and the `update` rule will happen any time there is a change to the `VNode` instance created from the `RxCanvas` class - which includes changes to its `data`.  Spelling it out like this is verbose and repetitive - prefer the shorthand where possible.

This custom directive will only be available on your `RxCanvas` component.  If you'd like to use it on multiple components, define it globally:

```typescript
// index.ts
Vue.directive('draw': function(canvasElement, binding) {
      binding.value.draw(canvasElement as HTMLCanvasElement);
});
```

Huzzah!