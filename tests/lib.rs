/*#![feature(test)]
extern crate test;*/
extern crate random_wheel;
extern crate rand;

#[cfg(test)]
mod tests {

    use random_wheel::RandomWheel;
    use rand;

    // need nightly
    /*use test;
    #[bench]
    fn bench_create(b: &mut test::Bencher) {

        b.iter(|| RandomWheel::<i32>::new());
    }*/

    #[test]
    fn test_peek_and_pop_to_empty() {

        let value = 'r';
        let mut wheel = RandomWheel::new(rand::thread_rng());

        wheel.push(1., value);
        assert_eq!(wheel.peek(), Some((1., &value)));
        assert_eq!(wheel.pop(), Some((1., value)));
        assert_eq!(wheel.pop(), None);
    }

    #[test]
    fn test_iterable() {

        let value = 'r';
        let mut wheel = RandomWheel::new(rand::thread_rng());

        wheel.push(1., value);

        for (prob, actual_value) in wheel.by_ref() {
            assert_eq!(prob, 1.);
            assert_eq!(value, actual_value);
        }

        assert_eq!(wheel.len(), 0);
    }

    #[test]
    fn test_clear() {

        let mut wheel = RandomWheel::new(rand::thread_rng());

        wheel.push(1., 'a');
        wheel.push(1., 'c');
        wheel.push(1., 'f');
        wheel.push(1., 'r');
        wheel.push(1., 'z');
        assert_eq!(wheel.len(), 5);

        wheel.clear();
        assert_eq!(wheel.len(), 0);
        assert_eq!(wheel.peek(), None);
        assert_eq!(wheel.pop(), None);
    }

    #[test]
    fn test_proba_sum() {

        let mut wheel = RandomWheel::new(rand::thread_rng());

        wheel.push(1., 'a');
        wheel.push(0.5, 'c');
        wheel.push(1.5, 'f');
        wheel.push(3., 'r');
        wheel.push(5.5, 'z');
        assert_eq!(wheel.proba_sum(), 11.5);
    }

    /*
    // no clone struct
    #[derive(PartialEq, Eq, Debug)] // for assert!
    struct NoClone {
        field: i32
    }

    // not properly working without clone
    #[test]
    fn test_no_clone_implement() {
        let mut wheel = RandomWheel::new(rand::thread_rng());

        let noclone = NoClone{ field: 3 };
        wheel.push(1., noclone);
        assert_eq!(wheel.peek(), Some(&noclone));
        assert_eq!(wheel.pop(), Some(noclone));
        assert_eq!(wheel.pop(), None);
    }
    */
}
