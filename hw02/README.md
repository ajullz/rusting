Implementation and testing based on the exercies from https://github.com/dhole/rust-homework/tree/master/hw02

**Some Tips from the Slides:**

* You can't keep borrowing something after it stops existing.
* One object may have many immutable references to it (&T).
* OR exactly one mutable reference (&mut T) (not both).