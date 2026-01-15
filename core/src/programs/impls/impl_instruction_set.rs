/*
    Appellation: impl_program <module>
    Created At: 2026.01.11:12:33:32
    Contrib: @FL03
*/
use crate::programs::{InstructionSet, RuleSet};
use crate::rules::Rule;
use rstm_state::{IntoState, RawState, State};

impl<R, Q, A> InstructionSet<R, Q, A>
where
    Q: RawState,
    R: RuleSet<Q, A>,
{
    /// initialize a new program from the given rule set
    pub const fn from_rules(rules: R) -> Self {
        Self {
            rules,
            initial_state: None,
            _marker: core::marker::PhantomData::<Rule<Q, A>>,
        }
    }
    #[cfg(all(feature = "json", feature = "std"))]
    /// load a program from a `.json` file at the given path
    pub fn load_from_json<P: AsRef<std::path::Path>>(path: P) -> crate::Result<Self>
    where
        Self: serde::de::DeserializeOwned,
    {
        // open the file
        let file = std::fs::File::open(path)?;
        // create a buffered reader
        let reader = std::io::BufReader::new(file);
        // deserialize the program
        let p = serde_json::from_reader(reader)?;
        Ok(p)
    }
    /// returns a reference to the ruleset
    pub const fn rules(&self) -> &R {
        &self.rules
    }
    #[allow(dead_code)]
    /// returns a mutable reference to the ruleset
    pub(crate) const fn rules_mut(&mut self) -> &mut R {
        &mut self.rules
    }
    /// returns reference to the (optional) initial state
    pub fn initial_state(&self) -> Option<&State<Q>> {
        self.initial_state.as_ref()
    }
    /// configure the initial state
    pub fn set_initial_state(&mut self, initial_state: Q) {
        self.initial_state = Some(State(initial_state));
    }
    #[inline]
    /// consumes the instance to create another with the given initial state
    pub fn with_initial_state<U>(self, initial_state: U) -> Self
    where
        U: IntoState<Q>,
    {
        Self {
            initial_state: Some(initial_state.into_state()),
            ..self
        }
    }
}
