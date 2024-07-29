/*
    Appellation: slider <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(dead_code)]
use crate::{Head, State};

///
///
/// In the paper, ["On the topological dynamics of Turing Machines"](https://pdf.sciencedirectassets.com/271538/1-s2.0-S0304397500X00527/1-s2.0-S0304397596000254/main.pdf?X-Amz-Security-Token=IQoJb3JpZ2luX2VjEC4aCXVzLWVhc3QtMSJHMEUCIAs%2FIGbrXGO61g5aPm39Ib9mhu0rk0voDj%2FUKAD%2B0EmVAiEAtagmY%2FVIBX8CvMgEmXsSzaOck0zIZmmoLxZ5eicB7nwqsgUIFxAFGgwwNTkwMDM1NDY4NjUiDDYtSMTlhppTdI2XOiqPBQ0lXDEHbefl0B8yi5yFePwQGNd18aDmyxP8Gglg%2FY5BIAPdXUNRIoe50cEbiKFm8HdtxVoszJGOBqN71165Tulos2iAdYmhGkzcqMJMikG%2FpAfZu1x3uo3frEbNtIW2J%2FSYo0vrK2OXh3LXb4VQkoZpceVDrG26ZgZWIuUXuSomzBkxy%2BfqvaOeqNrKqN22mxQe59mJYQCgaDP0ev1zltb8ULFHKHv35%2FX98bXTIXXBp7IXMjHdXddO9jhHONGa6HbGazEBhpxy1xs3mADH1ErJ0JlWG0GZwrkz9VM9ap%2Bwnq9niYgYd1adWqeuep6POgA0SZtgAFFFGLtGeHTRx%2B8I%2BSSzzYXK4rrNjnsdJXMkK9fXPOsTZPbtafM5IVBCtCFIClrRx9AKe%2FpDGNGJ9um2Teh%2FgZdamXl0dLCHBmUxxXXf5cH7QBPV1YzOO%2FdsFxhvrST%2FW7BWDEeNV89UVAjJjRXD4gT64B1aujgX55UCgYRJKpi8r8Z7scMdywDUA8lTFxG8ckFy8VSJXP4XIUo76TdmRao2MiDo6a0S3QUvlXp9j01mB%2FLRF0%2Bj7HPOH6PI23YdvMI1U%2BnwYu9FgyfFt8gDO6a3JYDUYmco5K13YXn0BiJtO4bd8D9q2WyCYoHzog4fDPKLArmrEqbBXqcJ5E6SEz7OkQuVIm3eFmSMsU%2BmbXx8di%2FRWaCVpwht3okeVnsci%2F2IMBwCiZnoByJ9a3KD9xavV81n6h%2BAL8YD2tyR%2BqaZdgi%2Fx8eMS8H14cLJB6o8ZpiGmthUXT66emKAhZfhnTweLEuEavsZcMIMOuI%2B9FEC2dfeUWjdtm0VnrufqEID2%2BowOXodj65RLAiD2wKTLYyuVVApBRW%2FFw0wrp%2BZtQY6sQEvfJW%2BnFXtKiHbJusaao0u5Y3T6RDPxNbcZMt1A%2FkN6icm1%2BP32LwAQ2DLIxmT6SFpc1SxogGCfvggZtF%2BxERxIr9B3HpMNnfvvEh9NvD6NQQAqbINxzEoKIMXksRuc5wrnry1KmA51CtmfJ2TbJOlZ1KWI%2FIJW1gcoU0omdhntl3162JQQZFd9jIKQ07K2ojI8FXtcs3tAJQho5g4eM%2F7bCqvDb1Sgn0B77vPL8LEdKI%3D&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Date=20240728T144819Z&X-Amz-SignedHeaders=host&X-Amz-Expires=300&X-Amz-Credential=ASIAQ3PHCVTY2B4ENHUE%2F20240728%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Signature=35f40e1e6e1411dc3690307138e19043edef6f667f5ac5bbf191e0178217abc5&hash=0864d2d44896ab80e9b8766f9dd8b8d7516f124c8f19c29db323f504115bb2ba&host=68042c943591013ac2b2430a89b270f6af2c76d8dfd086a07176afe7c76c2c61&pii=S0304397596000254&tid=spdf-68cb7082-4608-4105-848d-6c23a206e008&sid=920ea93d24f32844d618d475ca675f16e0cbgxrqa&type=client&tsoh=d3d3LnNjaWVuY2VkaXJlY3QuY29t&ua=17155c0606530e0553&rr=8aa5ab6279d016b8&cc=us)
/// the authors define the state of a Turing machine to be its configuration. This
/// configuration consists of the inner state of the head, the symbol it is reading,
/// and the contents of the tape.
pub struct Slider<Q, S> {
    pub(crate) head: Head<Q, usize>,  // head of the tape
    /// state of the machine
    pub(crate) tape: Vec<S>,
}

impl<Q, S> Slider<Q, S> {
    pub fn new(State(state): State<Q>, tape: impl IntoIterator<Item = S>) -> Self {
        Self {
            head: Head::new(State(state), 0),
            tape: tape.into_iter().collect(),
        }
    }
    /// Returns an immutable reference to the head of the machine.
    pub fn head(&self) -> &Head<Q, usize> {
        &self.head
    }
    /// Returns the current state of the machine.
    pub fn state(&self) -> State<&Q> {
        self.head.state.to_ref()
    }
    /// Returns the current position of the head on the tape.
    pub fn pos(&self) -> usize {
        self.head.symbol
    }
    /// Returns the current symbol being read by the head.
    pub fn read(&self) -> &S {
        &self.tape[self.pos()]
    }
    /// Writes a symbol to the tape at the current position of the head.
    pub fn write(&mut self, symbol: S) {
        self.tape[self.head.symbol] = symbol;
    }
}
