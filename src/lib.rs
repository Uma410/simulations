#[deny(clippy::all)]

pub trait Simulation {
    type State;
    type Input;

    fn step(&mut self, input: Self::Input);

    fn state(&self) -> Self::State;

    fn iter<F>(&mut self, input: F) -> Iter<Self, F>
    where
        Self: Sized,
        F: FnMut() -> Self::Input,
    {
        Iter::new(self, input)
    }

    fn into_iter<F>(self, input: F) -> IntoIter<Self, F>
    where
        Self: Sized,
        F: FnMut() -> Self::Input,
    {
        IntoIter::new(self, input)
    }
}

impl<T> Simulation for Box<T>
where
    T: Simulation,
{
    type State = T::State;
    type Input = T::Input;

    fn step(&mut self, input: Self::Input) {
        (**self).step(input)
    }

    fn state(&self) -> Self::State {
        (**self).state()
    }
}

pub struct IntoIter<T, F>
where
    T: Simulation,
    F: FnMut() -> T::Input,
{
    sim: T,
    input: F,
}

impl<T, F> IntoIter<T, F>
where
    T: Simulation,
    F: FnMut() -> T::Input,
{
    fn new(sim: T, input: F) -> Self {
        Self { sim, input }
    }
}

pub struct Iter<'a, T, F>
where
    T: Simulation + ?Sized,
    F: FnMut() -> T::Input,
{
    sim: &'a mut T,
    input: F,
}

impl<'a, T, F> Iter<'a, T, F>
where
    T: Simulation + ?Sized,
    F: FnMut() -> T::Input,
{
    fn new(sim: &'a mut T, input: F) -> Self {
        Self { sim, input }
    }
}

impl<T, F> Iterator for IntoIter<T, F>
where
    T: Simulation,
    F: FnMut() -> T::Input,
{
    type Item = T::State;

    fn next(&mut self) -> Option<Self::Item> {
        self.sim.step((self.input)());
        Some(self.sim.state())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }
}

impl<'a, T, F> Iterator for Iter<'a, T, F>
where
    T: Simulation,
    F: FnMut() -> T::Input,
{
    type Item = T::State;

    fn next(&mut self) -> Option<Self::Item> {
        self.sim.step((self.input)());
        Some(self.sim.state())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }
}
