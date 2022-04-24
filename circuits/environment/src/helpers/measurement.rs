// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use core::{fmt::Debug, ops::Add};

/// A `Measurement` is a quantity that can be measured.
/// The variants of the `Measurement` defines a condition associated with the measurable quantity.
#[derive(Clone, Debug)]
pub enum Measurement<V: Ord + Add<Output = V>> {
    Exact(V),
    Range(V, V),
    UpperBound(V),
}

impl<V: Ord + Add<Output = V> + Copy + Debug> Measurement<V> {
    /// Returns `true` if the value matches the metric.
    /// For an `Exact` metric, `value` must be equal to the exact value defined by the metric.
    /// For a `Range` metric, `value` must be satisfy lower bound and the upper bound.
    /// For an `UpperBound` metric, `value` must be satisfy the upper bound.
    pub fn matches(&self, candidate: V) -> bool {
        let outcome = match self {
            Measurement::Exact(expected) => *expected == candidate,
            Measurement::Range(lower_bound, upper_bound) => candidate > *lower_bound && candidate < *upper_bound,
            Measurement::UpperBound(bound) => candidate < *bound,
        };

        if !outcome {
            eprintln!("{:?} does not match {:?}", candidate, self);
        }

        outcome
    }

    /// Composes two variants of `Measurement` and returns the resulting `Measurement`.
    /// The composition is defined such that if a value `a` satisfies `self` and a value `b` satisfies `other`,
    /// then `a + b` satisfies the resulting `Measurement`.
    pub fn compose(&self, other: &Self) -> Self {
        match (self, other) {
            // An `Exact` metric composed with an `Exact` metric, produces an `Exact` metric.
            (Measurement::Exact(self_value), Measurement::Exact(other_value)) => {
                Measurement::Exact(*self_value + *other_value)
            }
            // An `Exact` metric composed with a `Range` metric, produces a `Range` metric.
            (Measurement::Exact(self_value), Measurement::Range(lower_bound, upper_bound)) => {
                Measurement::Range(*self_value + *lower_bound, *self_value + *upper_bound)
            }
            // An `Exact` metric composed with an `UpperBound` metric, produces an `UpperBound` metric.
            (Measurement::Exact(self_value), Measurement::UpperBound(other_value)) => {
                Measurement::UpperBound(*self_value + *other_value)
            }
            // A `Range` metric composed with an `Exact` metric, produces a `Range` metric.
            (Measurement::Range(lower_bound, upper_bound), Measurement::Exact(other_value)) => {
                Measurement::Range(*lower_bound + *other_value, *upper_bound + *other_value)
            }
            // A `Range` metric composed with a `Range` metric, produces a `Range` metric.
            (
                Measurement::Range(self_lower_bound, self_upper_bound),
                Measurement::Range(other_lower_bound, other_upper_bound),
            ) => Measurement::Range(*self_lower_bound + *other_lower_bound, *self_upper_bound + *other_upper_bound),
            // A `Range` metric composed with an `UpperBound` metric, produces a `Range` metric.
            (Measurement::Range(lower_bound, upper_bound), Measurement::UpperBound(other_value)) => {
                Measurement::Range(*lower_bound, *upper_bound + *other_value)
            }
            // An `UpperBound` metric composed with an `UpperBound` metric, produces an `UpperBound` metric.
            (Measurement::UpperBound(self_value), Measurement::Exact(other_value)) => {
                Measurement::UpperBound(*self_value + *other_value)
            }
            // An `UpperBound` metric composed with a `Range` metric, produces an `UpperBound` metric.
            (Measurement::UpperBound(self_value), Measurement::Range(lower_bound, upper_bound)) => {
                Measurement::Range(*lower_bound, *self_value + *upper_bound)
            }
            // An `UpperBound` metric composed with an `UpperBound` metric, produces an `UpperBound` metric.
            (Measurement::UpperBound(self_value), Measurement::UpperBound(other_value)) => {
                Measurement::UpperBound(*self_value + *other_value)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use snarkvm_utilities::{test_rng, UniformRand};

    const ITERATIONS: usize = 1024;

    #[test]
    fn test_exact_is_satisfied() {
        for _ in 0..ITERATIONS {
            // Generate a random `Measurement` and candidate value.
            let value: usize = u16::rand(&mut test_rng()) as usize;
            let candidate: usize = u16::rand(&mut test_rng()) as usize;
            let metric = Measurement::Exact(value);

            // Check that the metric is only satisfied if the candidate is equal to the value.
            assert!(metric.matches(value));
            if candidate == value {
                assert!(metric.matches(candidate));
            } else {
                assert!(!metric.matches(candidate));
            }
        }
    }

    #[test]
    fn test_exclusive_exclusive_range_is_satisfied() {
        for _ in 0..ITERATIONS {
            // Generate a random `Measurement::UpperBound` and candidate value.
            let first_bound: usize = u16::rand(&mut test_rng()) as usize;
            let second_bound: usize = u16::rand(&mut test_rng()) as usize;
            let candidate: usize = u16::rand(&mut test_rng()) as usize;
            let (metric, lower_bound, upper_bound) = if first_bound <= second_bound {
                (Measurement::Range(first_bound, second_bound), first_bound, second_bound)
            } else {
                (Measurement::Range(second_bound, first_bound), second_bound, first_bound)
            };

            // Check that the metric is only satisfied if the candidate is less than upper_bound.
            assert!(!metric.matches(lower_bound));
            assert!(!metric.matches(upper_bound));
            if lower_bound < candidate && candidate < upper_bound {
                assert!(metric.matches(candidate));
            } else {
                assert!(!metric.matches(candidate));
            }
        }
    }

    #[test]
    fn test_exclusive_upper_bound_is_satisfied() {
        for _ in 0..ITERATIONS {
            // Generate a random `Measurement::UpperBound` and candidate value.
            let upper_bound: usize = u16::rand(&mut test_rng()) as usize;
            let candidate: usize = u16::rand(&mut test_rng()) as usize;
            let metric = Measurement::UpperBound(upper_bound);

            // Check that the metric is only satisfied if the candidate is less than upper_bound.
            assert!(!metric.matches(upper_bound));
            if candidate < upper_bound {
                assert!(metric.matches(candidate));
            } else {
                assert!(!metric.matches(candidate));
            }
        }
    }

    // Test composition of metrics.

    #[test]
    fn test_exact_compose_with_exact() {
        for _ in 0..ITERATIONS {
            let first: usize = u16::rand(&mut test_rng()) as usize;
            let second: usize = u16::rand(&mut test_rng()) as usize;
            let candidate: usize = u16::rand(&mut test_rng()) as usize;

            let a = Measurement::Exact(first);
            let b = Measurement::Exact(second);
            let c = a.compose(&b);

            assert!(c.matches(first + second));
            if candidate == first + second {
                assert!(c.matches(candidate));
            } else {
                assert!(!c.matches(candidate));
            }
        }
    }

    #[test]
    fn test_exact_compose_with_exclusive_exclusive_range() {
        let value: usize = u16::rand(&mut test_rng()) as usize;
        let first_bound: usize = u16::rand(&mut test_rng()) as usize;
        let second_bound: usize = u16::rand(&mut test_rng()) as usize;
        let candidate: usize = u16::rand(&mut test_rng()) as usize;

        let a = Measurement::Exact(value);
        let (b, lower_bound, upper_bound) = if first_bound <= second_bound {
            (Measurement::Range(first_bound, second_bound), first_bound, second_bound)
        } else {
            (Measurement::Range(second_bound, first_bound), second_bound, first_bound)
        };
        let c = a.compose(&b);

        assert!(!c.matches(value + lower_bound));
        assert!(!c.matches(value + upper_bound));
        if value + lower_bound < candidate && candidate < value + upper_bound {
            assert!(c.matches(candidate));
        } else {
            assert!(!c.matches(candidate));
        }
    }

    #[test]
    fn test_exact_compose_with_exclusive_upper_bound() {
        for _ in 0..ITERATIONS {
            let first: usize = u16::rand(&mut test_rng()) as usize;
            let second: usize = u16::rand(&mut test_rng()) as usize;
            let candidate: usize = u16::rand(&mut test_rng()) as usize;

            let a = Measurement::Exact(first);
            let b = Measurement::UpperBound(second);
            let c = a.compose(&b);

            assert!(!c.matches(first + second));
            if candidate < first + second {
                assert!(c.matches(candidate));
            } else {
                assert!(!c.matches(candidate));
            }
        }
    }

    #[test]
    fn test_exclusive_exclusive_range_compose_with_exact() {
        let value: usize = u16::rand(&mut test_rng()) as usize;
        let first_bound: usize = u16::rand(&mut test_rng()) as usize;
        let second_bound: usize = u16::rand(&mut test_rng()) as usize;
        let candidate: usize = u16::rand(&mut test_rng()) as usize;

        let (a, lower_bound, upper_bound) = if first_bound <= second_bound {
            (Measurement::Range(first_bound, second_bound), first_bound, second_bound)
        } else {
            (Measurement::Range(second_bound, first_bound), second_bound, first_bound)
        };
        let b = Measurement::Exact(value);
        let c = a.compose(&b);

        assert!(!c.matches(value + lower_bound));
        assert!(!c.matches(value + upper_bound));
        if value + lower_bound < candidate && candidate < value + upper_bound {
            assert!(c.matches(candidate));
        } else {
            assert!(!c.matches(candidate));
        }
    }

    #[test]
    fn test_exclusive_exclusive_range_compose_with_exclusive_exclusive_range() {
        for _ in 0..ITERATIONS {
            let first: usize = u16::rand(&mut test_rng()) as usize;
            let second: usize = u16::rand(&mut test_rng()) as usize;
            let third: usize = u16::rand(&mut test_rng()) as usize;
            let fourth: usize = u16::rand(&mut test_rng()) as usize;
            let candidate: usize = u16::rand(&mut test_rng()) as usize;

            let (a, first_lower_bound, first_upper_bound) = if first <= second {
                (Measurement::Range(first, second), first, second)
            } else {
                (Measurement::Range(second, first), second, first)
            };
            let (b, second_lower_bound, second_upper_bound) = if third <= fourth {
                (Measurement::Range(third, fourth), third, fourth)
            } else {
                (Measurement::Range(fourth, third), fourth, third)
            };
            let c = a.compose(&b);

            assert!(!c.matches(first_lower_bound + second_lower_bound));
            assert!(!c.matches(first_upper_bound + second_upper_bound));
            if first_lower_bound + second_lower_bound < candidate && candidate < first_upper_bound + second_upper_bound
            {
                assert!(c.matches(candidate));
            } else {
                assert!(!c.matches(candidate));
            }
        }
    }

    #[test]
    fn test_exclusive_exclusive_range_compose_with_exclusive_upper_bound() {
        for _ in 0..ITERATIONS {
            let first: usize = u16::rand(&mut test_rng()) as usize;
            let second: usize = u16::rand(&mut test_rng()) as usize;
            let third: usize = u16::rand(&mut test_rng()) as usize;
            let candidate: usize = u16::rand(&mut test_rng()) as usize;

            let (a, lower_bound, upper_bound) = if second <= third {
                (Measurement::Range(second, third), second, third)
            } else {
                (Measurement::Range(third, second), third, second)
            };
            let b = Measurement::UpperBound(first);
            let c = a.compose(&b);

            assert!(!c.matches(lower_bound));
            assert!(!c.matches(first + upper_bound));
            if lower_bound < candidate && candidate < first + upper_bound {
                assert!(c.matches(candidate));
            } else {
                assert!(!c.matches(candidate));
            }
        }
    }

    #[test]
    fn test_exclusive_upper_bound_compose_with_exact() {
        for _ in 0..ITERATIONS {
            let first: usize = u16::rand(&mut test_rng()) as usize;
            let second: usize = u16::rand(&mut test_rng()) as usize;
            let candidate: usize = u16::rand(&mut test_rng()) as usize;

            let a = Measurement::UpperBound(second);
            let b = Measurement::Exact(first);
            let c = a.compose(&b);

            assert!(!c.matches(first + second));
            if candidate < first + second {
                assert!(c.matches(candidate));
            } else {
                assert!(!c.matches(candidate));
            }
        }
    }

    #[test]
    fn test_exclusive_upper_bound_compose_with_exclusive_exclusive_range() {
        for _ in 0..ITERATIONS {
            let first: usize = u16::rand(&mut test_rng()) as usize;
            let second: usize = u16::rand(&mut test_rng()) as usize;
            let third: usize = u16::rand(&mut test_rng()) as usize;
            let candidate: usize = u16::rand(&mut test_rng()) as usize;

            let a = Measurement::UpperBound(first);
            let (b, lower_bound, upper_bound) = if second <= third {
                (Measurement::Range(second, third), second, third)
            } else {
                (Measurement::Range(third, second), third, second)
            };
            let c = a.compose(&b);

            assert!(!c.matches(lower_bound));
            assert!(!c.matches(first + upper_bound));
            if lower_bound < candidate && candidate < first + upper_bound {
                assert!(c.matches(candidate));
            } else {
                assert!(!c.matches(candidate));
            }
        }
    }

    #[test]
    fn test_exclusive_upper_bound_compose_with_exclusive_upper_bound() {
        for _ in 0..ITERATIONS {
            let first: usize = u16::rand(&mut test_rng()) as usize;
            let second: usize = u16::rand(&mut test_rng()) as usize;
            let candidate: usize = u16::rand(&mut test_rng()) as usize;

            let a = Measurement::UpperBound(second);
            let b = Measurement::UpperBound(first);
            let c = a.compose(&b);

            assert!(!c.matches(first + second));
            if candidate < first + second {
                assert!(c.matches(candidate));
            } else {
                assert!(!c.matches(candidate));
            }
        }
    }
}