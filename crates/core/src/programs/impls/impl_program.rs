/*
    Appellation: impl_program <module>
    Created At: 2025.09.04:19:04:56
    Contrib: @FL03
*/
#![cfg(feature = "alloc")]
use crate::programs::ProgramO;

use crate::{Head, Rule, RuleVec, Tail};
use alloc::vec::Vec;
use rstm_state::{RawState, State};

impl<Q, A> ProgramO<Q, A>
where
    Q: RawState,
{
    /// initializes a new, empty instance of the [`Program`]
    pub const fn new(initial_state: Q) -> Self {
        Self {
            initial_state: State(initial_state),
            rules: Vec::new(),
        }
    }
    /// returns a new instance of the [`Program`] using the given rules
    pub fn from_rules<I>(iter: I) -> Self
    where
        Q: Default,
        I: IntoIterator<Item = Rule<Q, A>>,
    {
        Self {
            initial_state: State::default(),
            rules: Vec::from_iter(iter),
        }
    }
    /// Create a new instance of the [Program] using the given initial state.
    pub fn from_state(initial_state: Q) -> Self {
        Self {
            initial_state: State(initial_state),
            rules: Vec::new(),
        }
    }
    #[cfg(all(feature = "json", feature = "std"))]
    /// load a program from a `.json` file at the given path
    pub fn load_from_json<P: AsRef<std::path::Path>>(path: P) -> crate::Result<Self>
    where
        ProgramO<Q, A>: serde::de::DeserializeOwned,
    {
        // open the file
        let file = std::fs::File::open(path)?;
        // create a buffered reader
        let reader = std::io::BufReader::new(file);
        // deserialize the program
        let p = serde_json::from_reader(reader)?;
        Ok(p)
    }
    /// Returns an owned reference to the initial state of the program.
    pub fn initial_state(&self) -> State<&Q> {
        self.initial_state.view()
    }
    /// Returns a reference to the instructions.
    pub const fn rules(&self) -> &RuleVec<Q, A> {
        &self.rules
    }
    /// Returns a mutable reference to the instructions.
    pub const fn rules_mut(&mut self) -> &mut RuleVec<Q, A> {
        &mut self.rules
    }
    /// consumes the current instance to create another with the given default state
    pub fn with_default_state(self, state: Q) -> Self {
        Self {
            initial_state: State(state),
            ..self
        }
    }
    /// consumes the current instance to create another with the given rules
    pub fn with_rules<I>(self, rules: I) -> Self
    where
        I: IntoIterator<Item = Rule<Q, A>>,
    {
        Self {
            rules: Vec::from_iter(rules),
            ..self
        }
    }
    /// Returns an iterator over the elements.
    pub fn iter(&self) -> core::slice::Iter<'_, Rule<Q, A>> {
        self.rules().iter()
    }
    /// Returns a mutable iterator over the elements.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, Rule<Q, A>> {
        self.rules_mut().iter_mut()
    }
    /// returns an immutable reference to the tail for a given head; returns [`None`](Option::None)
    /// if no match is found.
    pub fn get<K>(&self, head: &K) -> Option<&Tail<Q, A>>
    where
        Q: PartialEq,
        A: PartialEq,
        K: core::borrow::Borrow<Head<Q, A>>,
    {
        self.iter().find_map(|i| {
            if i.head() == head.borrow() {
                Some(i.tail())
            } else {
                None
            }
        })
    }
    /// returns a mutable reference to the tail for a given head; returns [`None`](Option::None)
    /// if no match is found.
    pub fn get_mut<K>(&mut self, head: &K) -> Option<&mut Tail<Q, A>>
    where
        Q: PartialEq,
        A: PartialEq,
        K: core::borrow::Borrow<Head<Q, A>>,
    {
        self.iter_mut().find_map(|i| {
            if i.head() == head.borrow() {
                Some(i.tail_mut())
            } else {
                None
            }
        })
    }
    /// returns a reference to the tail of a rule for a given state and symbol; returns
    /// [`None`](Option::None) if no match is found.
    pub fn find_tail(&self, state: State<&Q>, symbol: &A) -> Option<&Tail<Q, A>>
    where
        Q: PartialEq,
        A: PartialEq,
    {
        self.iter().find_map(|i| {
            if i.head().view() == (Head { state, symbol }) {
                Some(i.tail())
            } else {
                None
            }
        })
    }
    /// returns a mutable reference to the tail for a given head; returns [`None`](Option::None)
    /// if no match is found.
    pub fn find_mut_tail(&mut self, state: State<&Q>, symbol: &A) -> Option<&mut Tail<Q, A>>
    where
        Q: PartialEq,
        A: PartialEq,
    {
        self.iter_mut().find_map(|i| {
            if i.head().view() == (Head { state, symbol }) {
                Some(i.tail_mut())
            } else {
                None
            }
        })
    }
    /// Returns a collection of rules whose head contains a match for the given state.
    pub fn filter_by_state(&self, state: State<&Q>) -> Vec<&Rule<Q, A>>
    where
        Q: PartialEq,
    {
        self.iter().filter(|i| *i.head() == state).collect()
    }
    #[cfg(all(feature = "json", feature = "std"))]
    /// export the program to a `.json` file at the given path
    ///
    /// **note**: there are no checks to see if the file already exists; it will automatically
    /// be overwritten.
    pub fn export_json<P>(&self, path: P) -> std::io::Result<()>
    where
        P: AsRef<std::path::Path>,
        Q: serde::Serialize,
        A: serde::Serialize,
    {
        let path = path.as_ref();
        // ensure the filename ends with `.json`
        if path.extension().map(|os| os.to_str()).flatten() != Some("json") {
            #[cfg(feature = "tracing")]
            tracing::error!(
                "the provided path does not end with `.json`; consider changing the file extension"
            );
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "the provided path does not end with `.json`",
            ));
        }
        let serialized = serde_json::to_string_pretty(self).unwrap();
        std::fs::write(path, serialized)?;
        #[cfg(feature = "tracing")]
        tracing::info!("Program exported as JSON");
        Ok(())
    }
}
