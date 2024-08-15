/*
 mod.rs file serves as the module root for a directory and
 is used to organize and manage the submodules within that directpry.
 Hwne you have health_check.rs and subscription.rs in routes directory
 , you need to declare them in mod.rs to make them part of routes module.

 */

mod health_check;
mod subscriptions;


/*
re-exports all public items from health_check modules so that they
can be accessed directly via routes module
 */
pub use health_check::*;
pub use subscriptions::*;