/*
    Appellation: actor <module>
    Created At: 2025.08.31:00:17:44
    Contrib: @FL03
*/

/// The [`RawActor`] is the basis for all compatible actors within the system.
pub trait RawActor {
    type Store;

    private! {}
}

/// Here, an [`Actor`] defines an entity capable of performing actions within an environment,
/// typically in response to a set of rules or stimuli. The interface works to generalize the
/// core components of the actual machines, isoating the essential behaviors and interactions
/// that define an actor's role within a system.
///
/// If Turing machines were divided into two focuses we could define them as the rulespace and
/// the actor. Understanding this distinction provides crucial insights into the design and
/// functionality of the machines. Most importantly, it clarifies the role of the actor as the
/// an entity without any inherent context or logic, rather, it is simply an _actionable_
/// system that may be orchestrated according to a set of rules. Any alterations to the
/// rulespace have an immediate effect on the overall behavior of the actor, as it is the
/// rules themselves that dictate the _response_ of the system depending on the current state
/// and symbol being read.
///
/// In line with robotics, an actor requires the introduction of a so-called _world space_ in
/// order to make sense of the "world" (i.e. inputs) it interacts with.

pub trait Actor: RawActor {}
