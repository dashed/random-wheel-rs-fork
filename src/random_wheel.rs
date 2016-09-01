extern crate rand;
extern crate num;

use std::fmt::Display;
use std::iter::repeat;
use std::collections::VecDeque;
use std::collections::vec_deque::{ Iter, IterMut };
use self::rand::Rng;
use self::rand::distributions::range::SampleRange;
use self::num::{Float};

/// a little implementation of a random-wheel.
pub struct RandomWheel<P: SampleRange + Float, T> {
    /// the sum of all probabilities in this wheel.
    proba_sum: P,
    /// all the (probability, data) in a linked-list to pop easily.
    cards: VecDeque<(P, T)>
}

impl<P: SampleRange + Float, T: Clone> Clone for RandomWheel<P, T> {
    fn clone(&self) -> RandomWheel<P, T> {
        RandomWheel{
            proba_sum: self.proba_sum,
            cards: self.cards.clone()
        }
    }
}

impl<P: SampleRange + Float + Display, T> Iterator for RandomWheel<P, T> {

    type Item = (P, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

impl<P: SampleRange + Float + Display, T> RandomWheel<P, T> {
    /// create a new random-wheel from vector.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let numbers: Vec<_> = (0..20).collect();
    ///
    /// // default probability is set to 1.0 for each element
    /// let rw: RandomWheel<u8> = RandomWheel::from_vec(numbers);
    /// ```
    pub fn from_vec(vector: Vec<T>) -> RandomWheel<P, T> {

        RandomWheel {

            proba_sum: P::from(vector.len()).unwrap(),
            cards: repeat(P::one()).into_iter().zip(vector).collect()
        }
    }

    /// create a new empty random-wheel.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let rw: RandomWheel<u8> = RandomWheel::new();
    /// ```
    pub fn new() -> RandomWheel<P, T> {
        RandomWheel {
            proba_sum: P::zero(),
            cards: VecDeque::new()
        }
    }

    /// Creates an empty RandomWheel with space for at least n elements.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let numbers: Vec<_> = (0..20).collect();
    /// let mut rw: RandomWheel<u8> = RandomWheel::with_capacity(numbers.len());
    ///
    /// assert_eq!(rw.len(), 0);
    /// ```
    pub fn with_capacity(n: usize) -> RandomWheel<P, T> {
        RandomWheel {
            proba_sum: P::zero(),
            cards: VecDeque::with_capacity(n)
        }
    }

    /// Reserves capacity for at least `additional` more elements to be inserted
    /// in the given `Ringbuf`.
    /// The collection may reserve more space to avoid frequent reallocations.
    /// # Example
    ///
    /// ```norun
    /// use random_wheel::RandomWheel;
    ///
    /// let mut rw: RandomWheel<u8> = RandomWheel::new();
    /// rw.reserve(20);
    ///
    /// assert_eq!(rw.len(), 0);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.cards.reserve(additional);
    }

    /// Returns the number of elements the RandomWheel can hold without
    /// reallocating.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let rw: RandomWheel<u8> = RandomWheel::new();
    ///
    /// println!("actual capacity: {}", rw.capacity());
    /// ```
    pub fn capacity(&self) -> usize {
        self.cards.capacity()
    }

    /// returns the number of elements in the wheel.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let mut rw = RandomWheel::new();
    ///
    /// assert_eq!(rw.len(), 0);
    ///
    /// rw.push(1., 'r');
    /// rw.push(1., 'c');
    /// rw.push(1., 'a');
    ///
    /// assert_eq!(rw.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// remove all elements in this wheel.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let mut rw = RandomWheel::new();
    ///
    /// rw.push(1., 'r');
    /// rw.push(1., 'c');
    /// rw.push(1., 'a');
    ///
    /// assert_eq!(rw.len(), 3);
    ///
    /// rw.clear();
    ///
    /// assert_eq!(rw.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.cards.clear()
    }

    /// returns `true` if this wheel is empty else return `false`.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let mut rw = RandomWheel::new();
    ///
    /// assert_eq!(rw.is_empty(), true);
    ///
    /// rw.push(1., 'r');
    /// rw.push(1., 'c');
    /// rw.push(1., 'a');
    ///
    /// assert_eq!(rw.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Returns an iterator over the slice.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let mut rw = RandomWheel::new();
    ///
    /// rw.push(1., 'r');
    /// rw.push(1., 'c');
    /// rw.push(1., 'a');
    ///
    /// let mut iter = rw.iter();
    ///
    /// assert_eq!(iter.next(), Some(&(1.0, 'r')));
    /// assert_eq!(iter.next(), Some(&(1.0, 'c')));
    /// assert_eq!(iter.next(), Some(&(1.0, 'a')));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<(P, T)> {
        self.cards.iter()
    }

    /// Returns an iterator that allows modifying each value.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let mut rw = RandomWheel::new();
    ///
    /// rw.push(1., 'r');
    /// rw.push(1., 'c');
    /// rw.push(1., 'a');
    ///
    /// for a in &mut rw.iter_mut() {
    ///     a.1 = 'm';
    /// }
    ///
    /// assert_eq!(rw.peek(), Some((1., &'m')));
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<(P, T)> {
        self.cards.iter_mut()
    }

    /// add an element associated with a probability.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let mut rw = RandomWheel::new();
    ///
    /// rw.push(1., 'r');
    /// rw.push(1., 'c');
    /// rw.push(1., 'a');
    ///
    /// assert_eq!(rw.len(), 3);
    /// ```
    pub fn push(&mut self, proba: P, data: T) {

        assert!(proba > P::zero(), "proba {} is lower or equal to zero!", proba);
        self.cards.push_back((proba, data));
        self.proba_sum = self.proba_sum + proba;
        if self.proba_sum.is_infinite() {
            panic!("Probability sum reached an Inf value!");
        }
    }

    /// Will recompute the probabilities sum
    /// use it when you iterate through this vector and change proba values
    pub fn compute_proba_sum(&mut self) {

        let mut sum = P::zero();
        for &(proba, _) in self.cards.iter() {

            assert!(proba > P::zero(), "proba {} is lower or equal to zero!", proba);
            sum = sum + proba;
        }
        self.proba_sum = sum;
        if self.proba_sum.is_infinite() {
            panic!("Probability sum reached an Inf value!");
        }
    }

    /// returns total of luck you pushed.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let mut rw = RandomWheel::new();
    ///
    /// rw.push(1.5, 'r');
    /// rw.push(2., 'c');
    /// rw.push(3., 'a');
    ///
    /// assert_eq!(rw.proba_sum(), 6.5);
    /// ```
    pub fn proba_sum(&self) -> P {
        self.proba_sum
    }

    /// returns a random distance to browser between 0 and the probabilities sum.
    fn gen_random_dist(&self) -> P {

        match self.proba_sum {

            sum if sum > P::zero() => rand::thread_rng().gen_range(P::zero(), sum),
            _               => P::zero()
        }
    }

    /// returns a random index in self.cards.
    fn get_random_index(&self) -> Option<usize> {

        if self.is_empty() {
            return None;
        }

        if self.len() <= 1 {
            // NOTE: fast path
            return Some(0);
        }

        let zero = P::zero();

        let mut dist = self.gen_random_dist();
        for (id, &(ref proba, _)) in self.cards.iter().enumerate() {

            dist = dist - *proba;
            if dist <= zero {
                return Some(id);
            }
        }

        // NOTE: this is unreachable

        return None;
    }

    /// returns a ref to the randomly peeked element with
    /// it's probality to be peeked.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let mut rw = RandomWheel::new();
    ///
    /// rw.push(1., 'r');
    ///
    /// assert_eq!(rw.peek(), Some((1.0, &'r')));
    /// assert_eq!(rw.peek(), Some((1.0, &'r')));
    /// ```
    pub fn peek(&self) -> Option<(P, &T)> {

        if let Some(index) = self.get_random_index() {

            if let Some(&(proba, ref data)) = self.cards.get(index) {
                Some((proba, data))
            }
            else { None }
        }
        else { None }
    }

    /// returns a mutable ref to the randomly peeked element with
    /// it's probality to be peeked.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let mut rw = RandomWheel::new();
    ///
    /// rw.push(1., 'r');
    ///
    /// match rw.peek_mut() {
    ///     Some((_, val)) => *val = 'b',
    ///     None => {}
    /// }
    ///
    /// assert_eq!(rw.peek(), Some((1.0, &'b')));
    /// ```
    pub fn peek_mut(&mut self) -> Option<(P, &mut T)> {

        if let Some(index) = self.get_random_index() {

            if let Some(&mut (proba, ref mut data)) = self.cards.get_mut(index) {
                Some((proba, data))
            }
            else { None }
        }
        else { None }
    }

    /// removes a randomly peeked element and return it with
    /// it's probality to be peeked.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let mut rw = RandomWheel::new();
    ///
    /// rw.push(1., 'r');
    ///
    /// assert_eq!(rw.pop(), Some((1.0, 'r')));
    ///
    /// // once you pop the value, it doesn't exist anymore
    /// assert_eq!(rw.peek(), None);
    /// assert_eq!(rw.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<(P, T)> {

        if let Some(index) = self.get_random_index() {

            if let Some((proba, data)) = self.cards.remove(index) {

                self.proba_sum = self.proba_sum - proba;
                Some((proba, data))
            }
            else { None }
        }
        else { None }
    }
}
