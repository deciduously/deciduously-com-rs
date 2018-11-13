# A ClojureScript Exercise Template

Clojure is a fantastic language.  It's quite minimal - there's very little syntax to learn, and it supports absolutely any paradigm of your choosing while still offering seamless (really, seamless) interop with both the massive Java and JavaScript ecosystems.

Unfortunately, though, getting going with it can be tricky and a little on the heavy side for quick experimentation.  You don't want to spin up a JVM and bootstrap the language just to run a quick function.

ClojureScript has a name that makes it sound like a whole separate thing but it's mostly the same exact language.  It's just hosted by JavaScript instead of Java.  One consequence of this is it becomes feasible for scripting.  You *can* do it with JVM Clojure but there's still more spin-up time than there should be.  My favorite option is called [planck](http://planck-repl.org/).

It starts almost instantly.  I've taken advantage of this to use it for language-agnostic code exercises - Advent of Code is just around the corner!  The [planck guide](http://planck-repl.org/guide-all.html) should help answer any questions, and also lists a number of very useful builtins for interacting with the shell and making network requests - you could use this instead of Python or Ruby for scripting tasks.

Here's my blank template:

```clojure
(ns exercise.core
  (:require [cljs.test :refer-macros [deftest is run-tests]]
            ;[clojure.string :refer [split]] ; you can import clojure fns!
            [planck.core :refer [slurp]]))

;; IMPLEMENTATION

(defn part1
  "Write your first function here!"
  [word]
  (str "Hello " word)

;; TESTS

(deftest test-part1
  (is (= (part1 "CLJS") "Hello, World")))

;; RUN
(defn -main []
  (let [puzzle (slurp "realinput.txt")]
    (run-tests)
    (println (str "Part 1 output: " (part1 puzzle)))

(set! *main-cli-fn* -main)

```

This setup will run your tests, and then use the builtin `slurp` to read the contents of the file you give it into a string.

Then you just run it!  Run `planck exercise.cljs` and watch your tests fail :)

Here's an example completed exercise:

```clojure
(ns d4.core
  (:require [cljs.test :refer-macros [deftest is run-tests]]
            [clojure.string :refer [split]]
            [planck.core :refer [slurp]]))

;; IMPLEMENTATION

(defn part1
  "Get how many times w occurs in ws"
  [w ws]
  (->> ws (filter #{w}) count))

(defn part2
  "Get how many anagrams of w occur in ws"
  [w ws]
  (->> ws (filter #(= (sort w) (sort %))) count))

(defn valid?
  "Check s for validity according to f"
  [f s]
  (let [words (split s " ")]
    (->> (map #(f % words) words)
         (filter #(> % 1))
         (empty?))))

(defn count-valid
  "Given string, returns how many lines are valid"
  [f s]
  (->> (split s "\n")
       (filter #(valid? f %))
       (count)))

;; TESTS

(deftest sample1
  (is (valid? part1 "aa bb cc dd ee"))
  (is (not (valid? part1 "aa bb cc dd aa")))
  (is (valid? part1 "aa bb cc dd aaa")))

(deftest sample2
  (is (valid? part2 "abcde fghij"))
  (is (not (valid? part2 "abcde xyz ecdab")))
  (is (valid? part2 "a ab abc abd abf abj"))
  (is (valid? part2 "iiii oiii ooii oooi oooo"))
  (is (not (valid? part2 "oiii ioii iioi iiio"))))

;; RUN

(defn -main []
  (let [puzzle (slurp "d4.txt")]
    (run-tests)
    (println (str "Part 1 output: " (count-valid part1 puzzle)))
    (println (str "Part 2 output: " (count-valid part2 puzzle)))))

(set! *main-cli-fn* -main)
```

Happy coding!