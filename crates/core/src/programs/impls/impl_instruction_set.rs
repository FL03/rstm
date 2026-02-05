/*
    Appellation: impl_program <module>
    Created At: 2026.01.11:12:33:32
    Contrib: @FL03
*/
use crate::programs::{ProgramBase, RuleSet};
use crate::rules::{Head, Rule, Tail};
use rstm_state::{IntoState, RawState, State};

impl<R, Q, A> ProgramBase<R, Q, A>
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
    #[cfg(feature = "serde_json")]
    /// serializes the current instance into a JSON string
    pub fn to_json(&self) -> serde_json::Value
    where
        Self: serde::Serialize,
    {
        serde_json::to_value(self).expect("Failed to serialize the Program instance")
    }
    #[cfg(all(feature = "json", feature = "std"))]
    /// export the program to a `.json` file at the given path
    ///
    /// **note**: there are no checks to see if the file already exists; it will automatically
    /// be overwritten.
    pub fn export_json<P>(&self, path: P) -> std::io::Result<()>
    where
        P: AsRef<std::path::Path>,
        Self: serde::Serialize,
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

    pub fn get_head(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
        self.rules().get(head)
    }
}
