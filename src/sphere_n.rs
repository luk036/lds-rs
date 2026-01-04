//! Generates points on n-dimensional spheres.
//!
//! # Algorithm Overview
//!
//! ```text
//!          VdCorput Sequence
//!                 |
//!                 v
//!   [0,1] ------------------> [0,π] -----> Sphere(n)
//!                    Mapping      Interpolation
//! ```
//!
//! This module implements sphere generators for arbitrary dimensions using
//! low-discrepancy sequences.

use crate::VdCorput;
use std::f64::consts::PI;

/// Simple implementation of numpy.linspace
fn linspace(start: f64, stop: f64, num: usize) -> Vec<f64> {
    if num == 1 {
        return vec![start];
    }
    let step = (stop - start) / (num as f64 - 1.0);
    (0..num).map(|i| start + i as f64 * step).collect()
}

/// Simple implementation of numpy.interp for 1D interpolation
fn simple_interp(x: f64, xp: &[f64], yp: &[f64]) -> f64 {
    if x <= xp[0] {
        return yp[0];
    }
    if x >= xp[xp.len() - 1] {
        return yp[yp.len() - 1];
    }

    for i in 0..xp.len() - 1 {
        if xp[i] <= x && x <= xp[i + 1] {
            // Linear interpolation
            let t = (x - xp[i]) / (xp[i + 1] - xp[i]);
            return yp[i] + t * (yp[i + 1] - yp[i]);
        }
    }

    yp[yp.len() - 1] // fallback
}

/// Precomputed tables for sphere generation
struct SphereTables {
    x: Vec<f64>,
    neg_cosine: Vec<f64>,
    sine: Vec<f64>,
    f2: Vec<f64>,
    half_pi: f64,
}

impl SphereTables {
    fn new() -> Self {
        let x = linspace(0.0, PI, 300);
        let neg_cosine = x.iter().map(|&x| -x.cos()).collect();
        let sine = x.iter().map(|&x| x.sin()).collect();
        let f2 = x
            .iter()
            .zip(&neg_cosine)
            .zip(&sine)
            .map(|((&x, &nc), &s)| (x + nc * s) / 2.0)
            .collect();
        let half_pi = PI / 2.0;

        Self {
            x,
            neg_cosine,
            sine,
            f2,
            half_pi,
        }
    }

    fn get(&self) -> (&[f64], &[f64], &[f64], &[f64], f64) {
        (
            &self.x,
            &self.neg_cosine,
            &self.sine,
            &self.f2,
            self.half_pi,
        )
    }
}

/// Thread-safe cached sphere tables
static SPHERE_TABLES: once_cell::sync::Lazy<SphereTables> =
    once_cell::sync::Lazy::new(SphereTables::new);

/// Calculates the table-lookup of the mapping function for n
fn get_tp(n: usize) -> Vec<f64> {
    use once_cell::sync::Lazy;
    use std::sync::Mutex;

    static TP_CACHE: Lazy<Mutex<Vec<Vec<f64>>>> = Lazy::new(|| Mutex::new(Vec::new()));

    let mut cache = TP_CACHE.lock().unwrap();

    // If already computed, return a copy
    if n < cache.len() {
        return cache[n].clone();
    }

    // Ensure cache has entries up to n
    while cache.len() <= n {
        let tables = SPHERE_TABLES.get();
        let x = &tables.0;
        let neg_cosine = &tables.1;
        let sine = &tables.2;

        let new_n = cache.len();
        let tp = if new_n == 0 {
            x.to_vec()
        } else if new_n == 1 {
            neg_cosine.to_vec()
        } else {
            let tp_minus2 = &cache[new_n - 2];
            x.iter()
                .enumerate()
                .map(|(i, _xi)| {
                    ((new_n - 1) as f64 * tp_minus2[i]
                        + neg_cosine[i] * sine[i].powi((new_n - 1) as i32))
                        / new_n as f64
                })
                .collect()
        };
        cache.push(tp);
    }

    cache[n].clone()
}

/// Base trait for sphere generators
pub trait SphereGen: Send + Sync {
    /// Generates and returns a vector of values
    fn pop(&mut self) -> Vec<f64>;

    /// Reseeds the generator with a new seed
    fn reseed(&mut self, seed: u32);
}

/// Wrapper for Sphere that implements SphereGen trait
struct SphereWrapper {
    sphere: crate::Sphere,
}

impl SphereWrapper {
    fn new(base: [u32; 2]) -> Self {
        Self {
            sphere: crate::Sphere::new(base),
        }
    }
}

impl SphereGen for SphereWrapper {
    fn pop(&mut self) -> Vec<f64> {
        self.sphere.pop().to_vec()
    }

    fn reseed(&mut self, seed: u32) {
        self.sphere.reseed(seed);
    }
}

/// 3-Sphere sequence generator
///
/// # Examples
///
/// ```
/// use lds_gen::sphere_n::{Sphere3, SphereGen};
/// let mut sgen = Sphere3::new(&[2, 3, 5]);
/// sgen.reseed(0);
/// let point = sgen.pop();
/// assert_eq!(point.len(), 4);
/// ```
pub struct Sphere3 {
    vdc: VdCorput,
    sphere2: SphereWrapper,
    half_pi: f64,
    x: Vec<f64>,
    f2: Vec<f64>,
}

impl Sphere3 {
    /// Creates a new 3-Sphere generator
    ///
    /// # Arguments
    ///
    /// * `base` - Array of 3 integers used as bases for the sequence
    pub fn new(base: &[u32]) -> Self {
        assert!(base.len() >= 3, "Sphere3 requires at least 3 bases");
        let tables = SPHERE_TABLES.get();
        Self {
            vdc: VdCorput::new(base[0]),
            sphere2: SphereWrapper::new([base[1], base[2]]),
            half_pi: tables.4,
            x: tables.0.to_vec(),
            f2: tables.3.to_vec(),
        }
    }
}

impl SphereGen for Sphere3 {
    fn pop(&mut self) -> Vec<f64> {
        let ti = self.half_pi * self.vdc.pop(); // map to [t0, tm-1]
        let xi = simple_interp(ti, &self.f2, &self.x);
        let cosxi = xi.cos();
        let sinxi = xi.sin();

        let sphere2_point = self.sphere2.pop();
        let mut result = Vec::with_capacity(4);
        for &s in &sphere2_point {
            result.push(sinxi * s);
        }
        result.push(cosxi);
        result
    }

    fn reseed(&mut self, seed: u32) {
        self.vdc.reseed(seed);
        self.sphere2.reseed(seed);
    }
}

/// Sphere-N sequence generator for arbitrary dimensions
///
/// # Examples
///
/// ```
/// use lds_gen::sphere_n::{SphereN, SphereGen};
/// let mut sgen = SphereN::new(&[2, 3, 5, 7]);
/// sgen.reseed(0);
/// let point = sgen.pop();
/// assert_eq!(point.len(), 5); // 4 bases produce 5D point
/// ```
pub struct SphereN {
    vdc: VdCorput,
    s_gen: Box<dyn SphereGen>,
    n: usize,
    tp: Vec<f64>,
    tp_start: f64,
    range: f64,
}

impl SphereN {
    /// Creates a new n-sphere generator
    ///
    /// # Arguments
    ///
    /// * `base` - Array of integers used as bases for the sequence
    ///   Length must be at least 3 (produces n+1 dimensional sphere)
    pub fn new(base: &[u32]) -> Self {
        let n = base.len() - 1;
        assert!(n >= 2, "SphereN requires at least 3 bases (n >= 2)");

        let vdc = VdCorput::new(base[0]);

        let s_gen: Box<dyn SphereGen> = if n == 2 {
            Box::new(SphereWrapper::new([base[1], base[2]]))
        } else {
            Box::new(SphereN::new(&base[1..]))
        };

        let tp = get_tp(n);
        let tp_start = tp[0];
        let range = tp[tp.len() - 1] - tp_start;

        Self {
            vdc,
            s_gen,
            n,
            tp,
            tp_start,
            range,
        }
    }
}

impl SphereGen for SphereN {
    fn pop(&mut self) -> Vec<f64> {
        if self.n == 2 {
            let tables = SPHERE_TABLES.get();
            let ti = tables.4 * self.vdc.pop(); // map to [t0, tm-1]
            let xi = simple_interp(ti, tables.3, tables.0);
            let cosxi = xi.cos();
            let sinxi = xi.sin();

            let sphere_point = self.s_gen.pop();
            let mut result = Vec::with_capacity(sphere_point.len() + 1);
            for &s in &sphere_point {
                result.push(sinxi * s);
            }
            result.push(cosxi);
            return result;
        }

        let vd = self.vdc.pop();
        let ti = self.tp_start + self.range * vd; // map to [t0, tm-1]
        let xi = simple_interp(ti, &self.tp, &SPHERE_TABLES.x);
        let sinphi = xi.sin();

        let sphere_point = self.s_gen.pop();
        let mut result = Vec::with_capacity(sphere_point.len() + 1);
        for &s in &sphere_point {
            result.push(s * sinphi);
        }
        result.push(xi.cos());
        result
    }

    fn reseed(&mut self, seed: u32) {
        self.vdc.reseed(seed);
        self.s_gen.reseed(seed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_linspace() {
        let result = linspace(0.0, 1.0, 5);
        let expected = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        assert_eq!(result.len(), 5);
        for i in 0..5 {
            assert_relative_eq!(result[i], expected[i], epsilon = 1e-10);
        }

        let result = linspace(0.0, 1.0, 1);
        assert_eq!(result, vec![0.0]);

        let result = linspace(-1.0, 1.0, 3);
        let expected = vec![-1.0, 0.0, 1.0];
        for i in 0..3 {
            assert_relative_eq!(result[i], expected[i], epsilon = 1e-10);
        }
    }

    #[test]
    fn test_simple_interp() {
        let xp = vec![0.0, 1.0, 2.0, 3.0];
        let yp = vec![0.0, 2.0, 4.0, 6.0]; // Linear function y = 2x

        let result = simple_interp(0.5, &xp, &yp);
        assert_relative_eq!(result, 1.0, epsilon = 1e-10);

        let result = simple_interp(1.5, &xp, &yp);
        assert_relative_eq!(result, 3.0, epsilon = 1e-10);

        let result = simple_interp(-0.5, &xp, &yp);
        assert_relative_eq!(result, 0.0, epsilon = 1e-10);

        let result = simple_interp(3.5, &xp, &yp);
        assert_relative_eq!(result, 6.0, epsilon = 1e-10);

        let result = simple_interp(2.0, &xp, &yp);
        assert_relative_eq!(result, 4.0, epsilon = 1e-10);
    }

    #[test]
    fn test_get_tp() {
        let tp0 = get_tp(0);
        assert_eq!(tp0.len(), 300);
        assert_relative_eq!(tp0[0], 0.0, epsilon = 1e-10);
        assert_relative_eq!(tp0[tp0.len() - 1], PI, epsilon = 1e-10);

        let tp1 = get_tp(1);
        assert_eq!(tp1.len(), 300);
        assert_relative_eq!(tp1[0], -0.0f64.cos(), epsilon = 1e-10);
        assert_relative_eq!(tp1[tp1.len() - 1], -PI.cos(), epsilon = 1e-10);

        let tp2 = get_tp(2);
        assert_eq!(tp2.len(), 300);
    }

    #[test]
    fn test_sphere3_basic() {
        let mut sgen = Sphere3::new(&[2, 3, 5]);
        sgen.reseed(0);

        let point = sgen.pop();
        assert_eq!(point.len(), 4);

        let radius_sq = point.iter().map(|&x| x * x).sum::<f64>();
        assert_relative_eq!(radius_sq, 1.0, epsilon = 1e-10);

        for &coord in &point {
            assert!(-1.0 <= coord && coord <= 1.0);
        }
    }

    #[test]
    fn test_sphere3_consistency() {
        let bases = vec![vec![2, 3, 5], vec![2, 5, 3], vec![3, 2, 7]];

        for base in bases {
            let mut sgen = Sphere3::new(&base);
            sgen.reseed(0);

            let points: Vec<_> = (0..5).map(|_| sgen.pop()).collect();

            for (i, point) in points.iter().enumerate() {
                let radius_sq = point.iter().map(|&x| x * x).sum::<f64>();
                assert_relative_eq!(radius_sq, 1.0, epsilon = 1e-10);
                // Additional check with custom message if needed
                if (radius_sq - 1.0).abs() > 1e-10 {
                    panic!(
                        "Base {:?}, Point {}: {:?}, r²={}",
                        base, i, point, radius_sq
                    );
                }
            }
        }
    }

    #[test]
    fn test_sphere3_reseed() {
        let mut sgen = Sphere3::new(&[2, 3, 5]);

        sgen.reseed(0);
        let seq1: Vec<_> = (0..3).map(|_| sgen.pop()).collect();

        sgen.reseed(0);
        let seq2: Vec<_> = (0..3).map(|_| sgen.pop()).collect();

        for i in 0..3 {
            for j in 0..4 {
                assert_relative_eq!(seq1[i][j], seq2[i][j], epsilon = 1e-10);
            }
        }

        sgen.reseed(1);
        let seq3: Vec<_> = (0..3).map(|_| sgen.pop()).collect();

        let mut different = false;
        for i in 0..3 {
            for j in 0..4 {
                if (seq1[i][j] - seq3[i][j]).abs() > 1e-10 {
                    different = true;
                    break;
                }
            }
            if different {
                break;
            }
        }
        assert!(
            different,
            "Sequences with different seeds should be different"
        );
    }

    #[test]
    fn test_spheren_basic() {
        let mut sgen = SphereN::new(&[2, 3, 5, 7]);
        sgen.reseed(0);

        let point = sgen.pop();
        assert_eq!(point.len(), 5);

        let radius_sq = point.iter().map(|&x| x * x).sum::<f64>();
        assert_relative_eq!(radius_sq, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_spheren_higher_dimensions() {
        let mut sgen = SphereN::new(&[2, 3, 5, 7, 11]);
        sgen.reseed(0);

        let point = sgen.pop();
        assert_eq!(point.len(), 6);

        let radius_sq = point.iter().map(|&x| x * x).sum::<f64>();
        assert_relative_eq!(radius_sq, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_spheren_reseed() {
        let mut sgen = SphereN::new(&[2, 3, 5, 7]);

        sgen.reseed(0);
        let seq1: Vec<_> = (0..3).map(|_| sgen.pop()).collect();

        sgen.reseed(0);
        let seq2: Vec<_> = (0..3).map(|_| sgen.pop()).collect();

        for i in 0..3 {
            for j in 0..5 {
                assert_relative_eq!(seq1[i][j], seq2[i][j], epsilon = 1e-10);
            }
        }
    }

    #[test]
    fn test_comparison_with_python() {
        // Expected values from Python doctest examples
        let expected_sphere3 = vec![
            0.2913440162992141,
            0.8966646826186098,
            -0.33333333333333337,
            6.123233995736766e-17,
        ];

        let expected_spheren = vec![
            0.4809684718990214,
            0.6031153874276115,
            -0.5785601510223212,
            0.2649326520763179,
            6.123233995736766e-17,
        ];

        let mut sgen3 = Sphere3::new(&[2, 3, 5]);
        sgen3.reseed(0);
        let result3 = sgen3.pop();

        for i in 0..4 {
            assert_relative_eq!(result3[i], expected_sphere3[i], epsilon = 1e-10);
        }

        let mut sgen_n = SphereN::new(&[2, 3, 5, 7]);
        sgen_n.reseed(0);
        let result_n = sgen_n.pop();

        for i in 0..5 {
            assert_relative_eq!(result_n[i], expected_spheren[i], epsilon = 1e-10);
        }
    }

    #[test]
    #[should_panic(expected = "Sphere3 requires at least 3 bases")]
    fn test_sphere3_insufficient_bases() {
        Sphere3::new(&[2, 3]);
    }

    #[test]
    #[should_panic(expected = "SphereN requires at least 3 bases")]
    fn test_spheren_insufficient_bases() {
        SphereN::new(&[2, 3]);
    }

    #[test]
    fn test_sphere_tables_thread_safety() {
        use std::sync::{Arc, Barrier};
        use std::thread;

        let num_threads = 8;
        let barrier = Arc::new(Barrier::new(num_threads));
        let mut handles = vec![];

        for _ in 0..num_threads {
            let barrier_clone = Arc::clone(&barrier);
            let handle = thread::spawn(move || {
                // Wait for all threads to be ready
                barrier_clone.wait();

                // All threads access SPHERE_TABLES simultaneously
                let tables = SPHERE_TABLES.get();
                assert_eq!(tables.0.len(), 300); // x table
                assert_eq!(tables.1.len(), 300); // neg_cosine table
                assert_eq!(tables.2.len(), 300); // sine table
                assert_eq!(tables.3.len(), 300); // f2 table
                assert_eq!(tables.4, std::f64::consts::PI / 2.0); // half_pi
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_get_tp_cache_thread_safety() {
        use std::sync::{Arc, Barrier};
        use std::thread;

        let num_threads = 8;
        let barrier = Arc::new(Barrier::new(num_threads));
        let mut handles = vec![];

        for thread_id in 0..num_threads {
            let barrier_clone = Arc::clone(&barrier);
            let handle = thread::spawn(move || {
                barrier_clone.wait();

                // Each thread requests different tp values
                let n = thread_id % 5; // Request tp values 0-4
                let tp = get_tp(n);

                // Verify the returned tp values
                assert_eq!(tp.len(), 300);

                // For n=0, tp is x which ranges from 0 to PI
                // For n=1, tp is neg_cosine which ranges from -1 to 1
                // For n>1, tp can have different ranges
                if n == 0 {
                    assert!(tp[0] >= 0.0 && tp[0] <= std::f64::consts::PI);
                    assert!(tp[tp.len() - 1] >= 0.0 && tp[tp.len() - 1] <= std::f64::consts::PI);
                } else if n == 1 {
                    assert!(tp[0] >= -1.0 && tp[0] <= 1.0);
                    assert!(tp[tp.len() - 1] >= -1.0 && tp[tp.len() - 1] <= 1.0);
                }
                // For n>1, we just check that the values are finite
                for &val in &tp {
                    assert!(val.is_finite());
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_sphere3_concurrent_access() {
        use std::sync::{Arc, Mutex};
        use std::thread;

        let sgen = Arc::new(Mutex::new(Sphere3::new(&[2, 3, 5])));
        sgen.lock().unwrap().reseed(0);

        let mut handles = vec![];
        let results = Arc::new(Mutex::new(Vec::new()));

        for _ in 0..4 {
            let sgen_clone = Arc::clone(&sgen);
            let results_clone = Arc::clone(&results);

            let handle = thread::spawn(move || {
                let mut local_points = Vec::new();
                for _ in 0..5 {
                    let mut generator = sgen_clone.lock().unwrap();
                    let point = generator.pop();
                    local_points.push(point);
                }
                let mut results = results_clone.lock().unwrap();
                results.push(local_points);
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let results = results.lock().unwrap();
        assert_eq!(results.len(), 4);

        for thread_results in results.iter() {
            assert_eq!(thread_results.len(), 5);
            for point in thread_results {
                assert_eq!(point.len(), 4);
                let radius_sq = point.iter().map(|&x| x * x).sum::<f64>();
                assert_relative_eq!(radius_sq, 1.0, epsilon = 1e-10);
            }
        }
    }

    #[test]
    fn test_spheren_concurrent_access() {
        use std::sync::{Arc, Mutex};
        use std::thread;

        let sgen = Arc::new(Mutex::new(SphereN::new(&[2, 3, 5, 7, 11])));
        sgen.lock().unwrap().reseed(0);

        let mut handles = vec![];
        let results = Arc::new(Mutex::new(Vec::new()));

        for _ in 0..4 {
            let sgen_clone = Arc::clone(&sgen);
            let results_clone = Arc::clone(&results);

            let handle = thread::spawn(move || {
                let mut local_points = Vec::new();
                for _ in 0..3 {
                    let mut generator = sgen_clone.lock().unwrap();
                    let point = generator.pop();
                    local_points.push(point);
                }
                let mut results = results_clone.lock().unwrap();
                results.push(local_points);
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let results = results.lock().unwrap();
        assert_eq!(results.len(), 4);

        for thread_results in results.iter() {
            assert_eq!(thread_results.len(), 3);
            for point in thread_results {
                assert_eq!(point.len(), 6);
                let radius_sq = point.iter().map(|&x| x * x).sum::<f64>();
                assert_relative_eq!(radius_sq, 1.0, epsilon = 1e-10);
            }
        }
    }

    #[test]
    fn test_multiple_sphere_instances_concurrent() {
        use std::sync::{Arc, Barrier};
        use std::thread;

        let num_threads = 6;
        let barrier = Arc::new(Barrier::new(num_threads));
        let mut handles = vec![];

        for thread_id in 0..num_threads {
            let barrier_clone = Arc::clone(&barrier);
            let handle = thread::spawn(move || {
                barrier_clone.wait();

                // Each thread creates its own sphere generator
                let bases = match thread_id % 3 {
                    0 => &[2, 3, 5][..],
                    1 => &[3, 5, 7][..],
                    _ => &[5, 7, 11][..],
                };

                let mut sgen: Box<dyn SphereGen> = if thread_id < 3 {
                    Box::new(Sphere3::new(bases))
                } else {
                    Box::new(SphereN::new(&[bases[0], bases[1], bases[2], 13]))
                };

                sgen.reseed(thread_id as u32);

                // Generate points
                for _ in 0..5 {
                    let point = sgen.pop();
                    let radius_sq = point.iter().map(|&x| x * x).sum::<f64>();
                    assert_relative_eq!(radius_sq, 1.0, epsilon = 1e-10);
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_sphere_trait_send_sync() {
        // This test verifies that SphereGen trait objects are Send + Sync
        fn is_send_sync<T: Send + Sync>() {}

        is_send_sync::<Sphere3>();
        is_send_sync::<SphereN>();

        // Verify trait objects are Send
        let mut sgen3: Box<dyn SphereGen> = Box::new(Sphere3::new(&[2, 3, 5]));
        let mut sgen_n: Box<dyn SphereGen> = Box::new(SphereN::new(&[2, 3, 5, 7]));

        // These operations should compile if Send is implemented
        sgen3.reseed(0);
        sgen_n.reseed(0);

        let _point3 = sgen3.pop();
        let _point_n = sgen_n.pop();
    }

    // Additional comprehensive tests for edge cases and higher dimensions

    #[test]
    fn test_linspace_edge_cases() {
        // Test with start = stop
        let result = linspace(1.0, 1.0, 5);
        assert_eq!(result, vec![1.0, 1.0, 1.0, 1.0, 1.0]);

        // Test with negative range
        let result = linspace(-1.0, -0.5, 3);
        let expected = vec![-1.0, -0.75, -0.5];
        for i in 0..3 {
            assert_relative_eq!(result[i], expected[i], epsilon = 1e-10);
        }

        // Test with large number of points
        let result = linspace(0.0, 1.0, 1000);
        assert_eq!(result.len(), 1000);
        assert_relative_eq!(result[0], 0.0, epsilon = 1e-10);
        assert_relative_eq!(result[999], 1.0, epsilon = 1e-10);
        assert_relative_eq!(result[500], 0.5005005005005005, epsilon = 1e-10);
    }

    #[test]
    fn test_simple_interp_edge_cases() {
        // Test with single point
        let xp = vec![0.5];
        let yp = vec![1.0];
        let result = simple_interp(0.5, &xp, &yp);
        assert_relative_eq!(result, 1.0, epsilon = 1e-10);

        // Test with constant function
        let xp = vec![0.0, 1.0, 2.0];
        let yp = vec![5.0, 5.0, 5.0];
        let result = simple_interp(1.5, &xp, &yp);
        assert_relative_eq!(result, 5.0, epsilon = 1e-10);

        // Test with non-uniform x points
        let xp = vec![0.0, 0.1, 0.5, 2.0];
        let yp = vec![0.0, 1.0, 2.0, 3.0];
        let result = simple_interp(0.3, &xp, &yp);
        // Should interpolate between points (0.1, 1.0) and (0.5, 2.0)
        let expected = 1.0 + (0.3 - 0.1) / (0.5 - 0.1) * (2.0 - 1.0);
        assert_relative_eq!(result, expected, epsilon = 1e-10);
    }

    #[test]
    fn test_sphere_n_higher_dimensions() {
        // Test with 10 dimensions (11 bases)
        let bases: Vec<u32> = (2..=12).collect();
        let mut sgen = SphereN::new(&bases);
        sgen.reseed(0);

        let point = sgen.pop();
        assert_eq!(point.len(), 12); // n+1 dimensions where n = 11

        let radius_sq = point.iter().map(|&x| x * x).sum::<f64>();
        assert_relative_eq!(radius_sq, 1.0, epsilon = 1e-10);

        // Test with 20 dimensions (21 bases)
        let bases: Vec<u32> = (2..=22).collect();
        let mut sgen = SphereN::new(&bases);
        sgen.reseed(0);

        let point = sgen.pop();
        assert_eq!(point.len(), 22); // n+1 dimensions where n = 21

        let radius_sq = point.iter().map(|&x| x * x).sum::<f64>();
        assert_relative_eq!(radius_sq, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_sphere_n_recursive_structure() {
        // Test that SphereN with 4 bases wraps a Sphere3
        let mut sgen4 = SphereN::new(&[2, 3, 5, 7]);
        sgen4.reseed(0);

        let point4 = sgen4.pop();
        assert_eq!(point4.len(), 5);

        // Test that SphereN with 5 bases wraps a SphereN with 4 bases
        let mut sgen5 = SphereN::new(&[2, 3, 5, 7, 11]);
        sgen5.reseed(0);

        let point5 = sgen5.pop();
        assert_eq!(point5.len(), 6);

        // Both should be on unit sphere
        let radius_sq4 = point4.iter().map(|&x| x * x).sum::<f64>();
        let radius_sq5 = point5.iter().map(|&x| x * x).sum::<f64>();
        assert_relative_eq!(radius_sq4, 1.0, epsilon = 1e-10);
        assert_relative_eq!(radius_sq5, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_sphere_tables_properties() {
        let tables = SPHERE_TABLES.get();
        let (x, neg_cosine, sine, f2, half_pi) = tables;

        // Check table lengths
        assert_eq!(x.len(), 300);
        assert_eq!(neg_cosine.len(), 300);
        assert_eq!(sine.len(), 300);
        assert_eq!(f2.len(), 300);

        // Check half_pi value
        assert_relative_eq!(half_pi, PI / 2.0, epsilon = 1e-10);

        // Check x table ranges from 0 to PI
        assert_relative_eq!(x[0], 0.0, epsilon = 1e-10);
        assert_relative_eq!(x[299], PI, epsilon = 1e-10);

        // Check neg_cosine is -cos(x)
        for i in 0..300 {
            assert_relative_eq!(neg_cosine[i], -x[i].cos(), epsilon = 1e-10);
        }

        // Check sine is sin(x)
        for i in 0..300 {
            assert_relative_eq!(sine[i], x[i].sin(), epsilon = 1e-10);
        }

        // Check f2 formula: (x + neg_cosine * sine) / 2.0
        for i in 0..300 {
            let expected = (x[i] + neg_cosine[i] * sine[i]) / 2.0;
            assert_relative_eq!(f2[i], expected, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_get_tp_higher_dimensions() {
        // Test tp values for higher dimensions
        let tp5 = get_tp(5);
        assert_eq!(tp5.len(), 300);

        let tp10 = get_tp(10);
        assert_eq!(tp10.len(), 300);

        let tp20 = get_tp(20);
        assert_eq!(tp20.len(), 300);

        // All values should be finite
        for &val in &tp5 {
            assert!(val.is_finite());
        }
        for &val in &tp10 {
            assert!(val.is_finite());
        }
        for &val in &tp20 {
            assert!(val.is_finite());
        }
    }

    #[test]
    fn test_sphere_sequence_distribution() {
        // Test that points are well-distributed (basic check)
        let mut sgen = SphereN::new(&[2, 3, 5, 7, 11, 13]);
        sgen.reseed(0);

        let mut points = Vec::new();
        for _ in 0..100 {
            points.push(sgen.pop());
        }

        // All points should be on unit sphere
        for point in &points {
            let radius_sq = point.iter().map(|&x| x * x).sum::<f64>();
            assert_relative_eq!(radius_sq, 1.0, epsilon = 1e-10);
        }

        // Points should be different (basic diversity check)
        for i in 1..points.len() {
            let mut same = true;
            for j in 0..points[i].len() {
                if (points[i][j] - points[0][j]).abs() > 1e-10 {
                    same = false;
                    break;
                }
            }
            if i < 10 {
                assert!(!same, "First few points should be different");
            }
        }
    }

    #[test]
    fn test_sphere_wrapper() {
        // Test SphereWrapper directly
        let mut wrapper = SphereWrapper::new([2, 3]);
        wrapper.reseed(0);

        let point = wrapper.pop();
        assert_eq!(point.len(), 3);

        let radius_sq = point.iter().map(|&x| x * x).sum::<f64>();
        assert_relative_eq!(radius_sq, 1.0, epsilon = 1e-10);

        // Test reseed
        wrapper.reseed(0);
        let point1 = wrapper.pop();

        wrapper.reseed(0);
        let point2 = wrapper.pop();

        for i in 0..3 {
            assert_relative_eq!(point1[i], point2[i], epsilon = 1e-10);
        }
    }

    #[test]
    fn test_sphere_n_different_bases() {
        // Test with different prime bases
        let bases = vec![vec![3, 5, 7, 11], vec![5, 7, 11, 13], vec![7, 11, 13, 17]];

        for base in bases {
            let mut sgen = SphereN::new(&base);
            sgen.reseed(0);

            let point = sgen.pop();
            assert_eq!(point.len(), base.len() + 1); // n+1 dimensions where n = base.len()

            let radius_sq = point.iter().map(|&x| x * x).sum::<f64>();
            assert_relative_eq!(radius_sq, 1.0, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_sphere3_different_bases() {
        // Test with different prime bases
        let bases = vec![vec![3, 5, 7], vec![5, 7, 11], vec![7, 11, 13]];

        for base in bases {
            let mut sgen = Sphere3::new(&base);
            sgen.reseed(0);

            let point = sgen.pop();
            assert_eq!(point.len(), 4);

            let radius_sq = point.iter().map(|&x| x * x).sum::<f64>();
            assert_relative_eq!(radius_sq, 1.0, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_sphere_n_large_seed() {
        // Test with large seed values
        let mut sgen = SphereN::new(&[2, 3, 5, 7]);

        for seed in [0, 100, 1000, 10000, 100000] {
            sgen.reseed(seed);
            let point = sgen.pop();

            let radius_sq = point.iter().map(|&x| x * x).sum::<f64>();
            assert_relative_eq!(radius_sq, 1.0, epsilon = 1e-10);

            for &coord in &point {
                assert!(coord.is_finite());
                assert!(coord >= -1.0 && coord <= 1.0);
            }
        }
    }

    #[test]
    fn test_sphere_coordinate_bounds() {
        // Test that all coordinates are within expected bounds
        let mut sgen = SphereN::new(&[2, 3, 5, 7, 11, 13, 17]);
        sgen.reseed(0);

        for _ in 0..100 {
            let point = sgen.pop();

            for &coord in &point {
                assert!(coord >= -1.0 && coord <= 1.0);
                assert!(coord.is_finite());
            }
        }
    }
}
