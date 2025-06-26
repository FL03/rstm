use crate::state::{Halt, RawState, State};

#[cfg(feature = "std")]
use std::sync::Arc;

impl<Q> Halt<Q>
where
    Q: RawState,
{
    /// a constructor method for the [`Halt`] type.
    pub const fn new(q: Q) -> Self {
        Self(q)
    }
    /// initializes a new instance of Halt using the given initializer function.
    pub fn create<F>(f: F) -> Self
    where
        F: FnOnce() -> Q,
    {
        Halt(f())
    }
    /// returns a new Halt with a value of one.
    pub fn one() -> Self
    where
        Q: num_traits::One,
    {
        Halt::create(Q::one)
    }
    /// returns a new Halt with a value of zero.
    pub fn zero() -> Self
    where
        Q: num_traits::Zero,
    {
        Halt::create(Q::zero)
    }
    /// returns a new instance of Halt with a raw pointer to the inner value.
    pub const fn as_ptr(&self) -> *const Q {
        core::ptr::addr_of!(self.0)
    }
    /// returns a new instance of Halt with a mutable raw pointer to the inner value.
    pub const fn as_mut_ptr(&mut self) -> *mut Q {
        core::ptr::addr_of_mut!(self.0)
    }
    #[allow(clippy::missing_safety_doc)]
    /// Casts the Halt to a new type, returning a new instance of [Halt].
    ///
    /// # Saftey
    ///
    /// This method is unsafe because it is up to the caller to ensure that the cast is valid.
    pub unsafe fn cast<R>(self) -> Halt<R> {
        unsafe { Halt(core::ptr::read(&self.0 as *const Q as *const R)) }
    }
    /// returns an immutable reference to the inner value of the Halt.
    pub const fn get(&self) -> &Q {
        &self.0
    }
    /// returns a mutable reference to the inner value of the Halt.
    pub const fn get_mut(&mut self) -> &mut Q {
        &mut self.0
    }
    /// consumes and returns the inner value of the Halt.
    #[inline]
    pub fn value(self) -> Q {
        self.0
    }
    /// [Halt::map] applies the given function onto the inner value of the Halt, returning a
    /// new Halt with the result.
    pub fn map<R, F>(self, f: F) -> Halt<R>
    where
        F: FnOnce(Q) -> R,
    {
        Halt(f(self.value()))
    }
    /// [`replace`](core::mem::replace) the inner value of the Halt with the given Halt,
    pub const fn replace(&mut self, value: Q) -> Q {
        core::mem::replace(self.get_mut(), value)
    }
    /// Clears the Halt, setting it to its default value.
    #[inline]
    pub fn reset(&mut self) -> &mut Self
    where
        Q: Default,
    {
        self.set(Default::default());
        self
    }
    /// Sets the Halt to a new value.
    pub fn set(&mut self, value: Q) -> &mut Self {
        self.0 = value;
        self
    }
    /// [`swap`](core::mem::swap) the inner value of the Halt with that of the given Halt.
    pub const fn swap(&mut self, other: &mut Halt<Q>) {
        core::mem::swap(self.get_mut(), other.get_mut());
    }
    /// [`take`](core::mem::take) the inner value of the Halt, leaving the logical default in
    /// its place
    pub fn take(&mut self) -> Q
    where
        Q: Default,
    {
        core::mem::take(self.get_mut())
    }
    pub fn stated(self) -> State<Halt<Q>> {
        State::new(self)
    }
    /// returns a new Halt with a boxed inner value.
    pub fn boxed(self) -> Halt<Box<Q>> {
        self.map(Box::new)
    }
    /// Converts the inner type into a boxed "any" Halt, returning a new instance of Halt
    pub fn as_any(&self) -> Halt<Box<dyn std::any::Any>>
    where
        Q: Clone + 'static,
    {
        Halt(Box::new(self.get().clone()))
    }
    /// Converts the inner type into a boxed "any" Halt, returning a new instance of Halt
    pub fn into_any(self) -> Halt<Box<dyn std::any::Any>>
    where
        Q: 'static,
    {
        Halt(Box::new(self.value()))
    }

    #[cfg(feature = "std")]
    /// Wraps the inner value of the Halt with an [`Arc`] and returns a new instance of [Halt]
    pub fn shared(self) -> Halt<Arc<Q>> {
        self.map(Arc::new)
    }
    #[cfg(feature = "std")]
    /// returns a shared reference to the Halt.
    pub fn to_shared(&self) -> Halt<Arc<Q>>
    where
        Q: Clone,
    {
        self.clone().shared()
    }
    /// returns a Halt with an owned inner value.
    pub const fn view(&self) -> Halt<&Q> {
        Halt(self.get())
    }
    /// returns a Halt with a mutable reference to the inner value.
    pub const fn view_mut(&mut self) -> Halt<&mut Q> {
        Halt(self.get_mut())
    }
    /// returns the `name` of the generic inner type, `Q`.
    pub fn get_inner_type_name(&self) -> &'static str {
        core::any::type_name::<Q>()
    }
    /// returns the `type id` of the generic inner type, `Q`.
    pub fn get_inner_type_id(&self) -> core::any::TypeId
    where
        Q: 'static,
    {
        core::any::TypeId::of::<Q>()
    }
}
