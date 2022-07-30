Implementation and testing based on the exercies from https://github.com/dhole/rust-homework/tree/master/hw02 and https://github.com/dhole/rust-homework/tree/master/hw03

**Some Tips from the Slides:**

* You can't keep borrowing something after it stops existing.
* One object may have many immutable references to it (&T).
* OR exactly one mutable reference (&mut T) (not both).

** Part 1: **
* Simple BST with insert and search methods

** Part 2: **
* BST from part 1 but with generics and iterators (into_iter, iter and mut iter)