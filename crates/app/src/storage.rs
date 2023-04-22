pub mod postgre;
pub(self) mod schema;

pub trait State {}

#[derive(Clone)]
pub struct Storage {}

impl State for Storage {}

/*
 *
 * ## SuperTrait Referencing
 * - Consider situation where we have a trait Alpha.
 *  ```rust
 *  trait Alpha<T>: Nexter<NextTarget = T> {}
 *  trait Nexter {
 *      type NextTarget;
 *  }
 *  ```
 *
 *  - In this scenerio we wish that the type `NextTarget` should know about the baby-trait that
 *  `Nexter` is used as a super-trait.
 *
 *  ```rust
 *  trait Nexter<T> {
 *      type NextTarget: Alpha<T> // This Alpha<T> should be auto referenced from the baby-trait
 *  }
 *  ```
 *
 *
 */

trait UserInterface {}

trait Tower {
    type NextTarget;
}
