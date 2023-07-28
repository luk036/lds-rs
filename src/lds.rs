// #![feature(unboxed_closures)]

const TWO_PI: f64 = std::f64::consts::TAU;

/// Van der Corput sequence
///
/// The `vdc` function is calculating the Van der Corput sequence value for a
/// given index `k` and base `base`. It returns a `f64` value.
///
/// # Examples
///
/// ```
/// use lds_rs::lds::vdc;
///
/// assert_eq!(vdc(11, 2), 0.8125);
/// ```
pub fn vdc(k: usize, base: usize) -> f64 {
    let mut res = 0.0;
    let mut denom = 1.0;
    let mut k = k;
    while k != 0 {
        let remainder = k % base;
        denom *= base as f64;
        k /= base;
        res += remainder as f64 / denom;
    }
    res
}

/// The `VdCorput` struct is a generator for the Van der Corput sequence, a low-discrepancy sequence
/// commonly used in quasi-Monte Carlo methods.
///
/// Properties:
///
/// * `count`: The `count` property is used to keep track of the current iteration count of the Van der
/// Corput sequence. It starts at 0 and increments by 1 each time the `pop()` method is called.
/// * `base`: The `base` property represents the base of the Van der Corput sequence. It determines the
/// number of digits used in each element of the sequence.
///
/// # Examples
///
/// ```
/// use lds_rs::VdCorput;
///
/// let mut vgen = VdCorput::new(2);
/// vgen.reseed(10);
/// let result = vgen.pop();
///
/// assert_eq!(result, 0.8125);
/// ```
pub struct VdCorput {
    count: usize,
    base: usize,
}

impl VdCorput {
    /// The `new` function creates a new [`VdCorput`] object with a given base for generating the Van der
    /// Corput sequence.
    ///
    /// Arguments:
    ///
    /// * `base`: The `base` parameter is an integer value that is used to generate the Van der Corput
    /// sequence. It determines the base of the sequence, which affects the distribution and pattern of the
    /// generated numbers.
    ///
    /// Returns:
    ///
    /// The `new` function returns a `VdCorput` object.
    pub const fn new(base: usize) -> Self {
        VdCorput { count: 0, base }
    }

    /// The `pop` function is a member function of the [`VdCorput`] class in Rust that increments the count
    /// and calculates the next value in the Van der Corput sequence.
    ///
    /// Returns:
    ///
    /// The `pop` function returns a `f64` value, which is the next value in the Van der Corput sequence.
    pub fn pop(&mut self) -> f64 {
        self.count += 1;
        vdc(self.count, self.base)
    }

    /// The below code is a Rust function called `reseed` that is used to reset the state of a sequence
    /// generator to a specific seed value. This allows the sequence generator to start generating the
    /// sequence from the beginning or from a specific point in the sequence, depending on the value of the
    /// seed.
    pub fn reseed(&mut self, seed: usize) {
        self.count = seed;
    }
}

/// The [`Halton`] struct is a sequence generator that generates points in a 2-dimensional space using the
/// Halton sequence.
///
/// Properties:
///
/// * `vdc0`: A variable of type [`VdCorput`] that represents the Van der Corput sequence generator for
/// the first base. The Van der Corput sequence is a low-discrepancy sequence that is commonly used in
/// quasi-Monte Carlo methods. It generates a sequence of numbers between 0 and
/// * `vdc1`: The `vdc1` property is an instance of the [`VdCorput`] struct, which is responsible for
/// generating the Van der Corput sequence with a base of 3. The Van der Corput sequence is another
/// low-discrepancy sequence commonly used in quasi-Monte Carlo methods
///
/// # Examples
///
/// ```
/// use lds_rs::Halton;
///
/// let mut hgen = Halton::new(2, 3);
/// hgen.reseed(10);
/// let result = hgen.pop();
/// assert_eq!(result[0], 0.8125);
/// ```
pub struct Halton {
    vdc0: VdCorput,
    vdc1: VdCorput,
}

impl Halton {
    /// The `new` function creates a new [`Halton`] object with specified bases for generating the Halton
    /// sequence.
    ///
    /// Arguments:
    ///
    /// * `base`: The `base` parameter is an array of two `usize` values. These values are used as the bases
    /// for generating the Halton sequence. The first value in the array (`base[0]`) is used as the base for
    /// generating the first component of the Halton sequence, and the second
    ///
    /// Returns:
    ///
    /// The `new` function returns an instance of the `Halton` struct.
    pub fn new(base0: usize, base1: usize) -> Self {
        Self {
            vdc0: VdCorput::new(base0),
            vdc1: VdCorput::new(base1),
        }
    }

    /// Returns the pop of this [`Halton`].
    ///
    /// The `pop()` function is used to generate the next value in the sequence.
    /// For example, in the [`VdCorput`] class, `pop()` increments the count and
    /// calculates the Van der Corput sequence value for that count and base. In
    /// the [`Halton`] class, `pop()` returns the next point in the Halton sequence
    /// as a `[f64; 2]`. Similarly, in the `Circle` class, `pop()`
    /// returns the next point on the unit circle as a `[f64; 2]`. In
    /// the `Sphere` class, `pop()` returns the next point on the unit sphere as a
    /// `[f64; 3]`. And in the `Sphere3Hopf` class, `pop()` returns
    /// the next point on the 3-sphere using the Hopf fibration as a
    /// `[f64; 4]`.
    ///
    /// Returns:
    ///
    /// An array of two f64 values is being returned.
    pub fn pop(&mut self) -> [f64; 2] {
        [self.vdc0.pop(), self.vdc1.pop()]
    }

    /// The below code is a Rust function called `reseed` that is used to reset the state of a sequence
    /// generator to a specific seed value. This allows the sequence generator to start generating the
    /// sequence from the beginning or from a specific point in the sequence, depending on the value of the
    /// seed.
    #[allow(dead_code)]
    pub fn reseed(&mut self, seed: usize) {
        self.vdc0.reseed(seed);
        self.vdc1.reseed(seed);
    }
}

/// Circle sequence generator
///
/// The `Circle` struct is a generator for a circle sequence using the Van der Corput sequence.
///
/// Properties:
///
/// * `vdc`: A variable of type VdCorput, which is a sequence generator for Van der Corput sequence.
///
/// # Examples
///
/// ```
/// use lds_rs::Circle;
///
/// let mut cgen = Circle::new(2);
/// cgen.reseed(1);
/// let result = cgen.pop();
/// assert_eq!(result[0], 1.0);
/// ```
pub struct Circle {
    vdc: VdCorput,
}

impl Circle {
    /// Creates a new [`Circle`].
    ///
    /// The `new` function creates a new [`Circle`] object with a specified base value.
    ///
    /// Arguments:
    ///
    /// * `base`: The `base` parameter in the `new` function is the base value used to generate the Van
    /// der Corput sequence. The Van der Corput sequence is a low-discrepancy sequence used in
    /// quasi-Monte Carlo methods. It is generated by reversing the digits of the fractional part of the
    ///
    /// Returns:
    ///
    /// The `new` function is returning an instance of the `Circle` struct.
    pub fn new(base: usize) -> Self {
        Circle {
            vdc: VdCorput::new(base),
        }
    }

    /// Returns the pop of this [`Circle`].
    ///
    /// The `pop` function returns the coordinates of a point on a circle based on a random value.
    ///
    /// Returns:
    ///
    /// The `pop` function returns an array of two `f64` values, representing the sine and cosine of a
    /// randomly generated angle.
    pub fn pop(&mut self) -> [f64; 2] {
        // let two_pi = 2.0/// (-1.0 as f64).acos(); // ???
        let theta = self.vdc.pop() * TWO_PI; // map to [0, 2*pi];
        [theta.sin(), theta.cos()]
    }

    /// The below code is a Rust function called `reseed` that is used to reset the state of a sequence
    /// generator to a specific seed value. This allows the sequence generator to start generating the
    /// sequence from the beginning or from a specific point in the sequence, depending on the value of the
    /// seed.
    #[allow(dead_code)]
    pub fn reseed(&mut self, seed: usize) {
        self.vdc.reseed(seed);
    }
}

/// Sphere sequence generator
///
/// The `Sphere` struct is a generator for a sequence of points on a sphere.
///
/// Properties:
///
/// * `vdc`: The `vdc` property is an instance of the [`VdCorput`] struct. It is used to generate a Van
/// der Corput sequence, which is a low-discrepancy sequence used for sampling points in a unit
/// interval.
/// * `cirgen`: The `cirgen` property is an instance of the [`Circle`] struct. It is responsible for
/// generating points on a circle.
///
/// # Examples
///
/// ```
/// use lds_rs::Sphere;
///
/// let mut sgen = Sphere::new(&[2, 3]);
/// sgen.reseed(1);
/// let result = sgen.pop();
/// assert_eq!(result[2], -0.5);
/// ```
pub struct Sphere {
    vdc: VdCorput,
    cirgen: Circle,
}

impl Sphere {
    /// Creates a new [`Sphere`].
    ///
    /// The function `new` creates a new [`Sphere`] object with a given base.
    ///
    /// Arguments:
    ///
    /// * `base`: The `base` parameter is an array of `usize` values. It is used to initialize the `Sphere`
    /// struct. The first element of the `base` array is used to create a new `VdCorput` struct, and the
    /// second element is used to create a new `Circle
    ///
    /// Returns:
    ///
    /// The `new` function is returning an instance of the `Sphere` struct.
    pub fn new(base: &[usize]) -> Self {
        Sphere {
            vdc: VdCorput::new(base[0]),
            cirgen: Circle::new(base[1]),
        }
    }

    /// Returns the pop of this [`Sphere`].
    ///
    /// The `pop` function returns a random point on a sphere using the VDC and cirgen generators.
    ///
    /// Returns:
    ///
    /// an array of three `f64` values, representing the coordinates of a point on a sphere. The first
    /// two values (`sinphi * c` and `sinphi * s`) represent the x and y coordinates, while the third
    /// value (`cosphi`) represents the z coordinate.
    pub fn pop(&mut self) -> [f64; 3] {
        let cosphi = 2.0 * self.vdc.pop() - 1.0; // map to [-1, 1];
        let sinphi = (1.0 - cosphi * cosphi).sqrt();
        let [c, s] = self.cirgen.pop();
        [sinphi * c, sinphi * s, cosphi]
    }

    /// The below code is a Rust function called `reseed` that is used to reset the state of a sequence
    /// generator to a specific seed value. This allows the sequence generator to start generating the
    /// sequence from the beginning or from a specific point in the sequence, depending on the value of the
    /// seed.
    #[allow(dead_code)]
    pub fn reseed(&mut self, seed: usize) {
        self.cirgen.reseed(seed);
        self.vdc.reseed(seed);
    }
}

/// The `Sphere3Hopf` struct is a sequence generator for the S(3) sequence using Hopf coordinates.
///
/// Properties:
///
/// * `vdc0`: An instance of the VdCorput sequence generator used for the first coordinate of the Hopf
/// coordinates.
/// * `vdc1`: The `vdc1` property is an instance of the [`VdCorput`] struct, which is used to generate a
/// Van der Corput sequence. This sequence is a low-discrepancy sequence that is commonly used in
/// numerical methods for generating random numbers. In this case, it is
/// * `vdc2`: The `vdc2` property is an instance of the [`VdCorput`] struct, which is used to generate a
/// Van der Corput sequence. This sequence is a low-discrepancy sequence that is commonly used in
/// numerical methods for generating random numbers. In the context of the `
///
/// The `Sphere3Hopf` class is a sequence generator that generates points on a
/// 3-sphere using the Hopf fibration. It uses three instances of the `VdCorput`
/// class to generate the sequence values and maps them to points on the
/// 3-sphere. The `pop()` method returns the next point on the 3-sphere as a
/// `[f64; 4]`, where the first three elements represent the x, y,
/// and z coordinates of the point, and the fourth element represents the w
/// coordinate. The `reseed()` method is used to reset the state of the sequence
/// generator to a specific seed value.
///
/// # Examples
///
/// ```
/// use lds_rs::Sphere3Hopf;
/// use approx_eq::assert_approx_eq;
///
/// let mut sgen = Sphere3Hopf::new(&[2, 3, 5]);
/// sgen.reseed(0);
/// let result = sgen.pop();
/// assert_approx_eq!(result[2], 0.4472135954999573);
/// ```
pub struct Sphere3Hopf {
    vdc0: VdCorput,
    vdc1: VdCorput,
    vdc2: VdCorput,
}

impl Sphere3Hopf {
    /// Creates a new [`Sphere3Hopf`].
    ///
    /// The `new` function creates a new instance of the [`Sphere3Hopf`] struct with three `VdCorput`
    /// instances initialized with the values from the `base` slice.
    ///
    /// Arguments:
    ///
    /// * `base`: The `base` parameter is an array of three `usize` values. These values are used to
    /// initialize three instances of the `VdCorput` struct, which is a type of quasi-random number
    /// generator. Each `VdCorput` instance is initialized with a different base value from the
    ///
    /// Returns:
    ///
    /// The `new` function is returning an instance of the `Sphere3Hopf` struct.
    pub fn new(base: &[usize]) -> Self {
        Sphere3Hopf {
            vdc0: VdCorput::new(base[0]),
            vdc1: VdCorput::new(base[1]),
            vdc2: VdCorput::new(base[2]),
        }
    }

    /// The `pop` function returns a four-element array representing the coordinates of a point on a
    /// sphere in 3D space.
    ///
    /// Returns:
    ///
    /// The function `pop` returns an array of four `f64` values.
    /// Returns the pop of this [`Sphere3Hopf`].
    ///
    /// The `pop()` function is used to generate the next value in the sequence.
    /// For example, in the [`VdCorput`] class, `pop()` increments the count and
    /// calculates the Van der Corput sequence value for that count and base. In
    /// the [`Halton`] class, `pop()` returns the next point in the Halton sequence
    /// as a `[f64; 2]`. Similarly, in the [`Circle`] class, `pop()`
    /// returns the next point on the unit circle as a `[f64; 2]`. In
    /// the [`Sphere`] class, `pop()` returns the next point on the unit sphere as a
    /// `[f64; 3]`. And in the [`Sphere3Hopf`] class, `pop()` returns
    /// the next point on the 3-sphere using the Hopf fibration as a `[f64; 4]`.
    pub fn pop(&mut self) -> [f64; 4] {
        let phi = self.vdc0.pop() * TWO_PI; // map to [0, 2*pi];
        let psy = self.vdc1.pop() * TWO_PI; // map to [0, 2*pi];
        let vd = self.vdc2.pop();
        let cos_eta = vd.sqrt();
        let sin_eta = (1.0 - vd).sqrt();
        [
            cos_eta * psy.cos(),
            cos_eta * psy.sin(),
            sin_eta * (phi + psy).cos(),
            sin_eta * (phi + psy).sin(),
        ]
    }

    /// The below code is a Rust function called `reseed` that is used to reset the state of a sequence
    /// generator to a specific seed value. This allows the sequence generator to start generating the
    /// sequence from the beginning or from a specific point in the sequence, depending on the value of the
    /// seed.
    #[allow(dead_code)]
    pub fn reseed(&mut self, seed: usize) {
        self.vdc0.reseed(seed);
        self.vdc1.reseed(seed);
        self.vdc2.reseed(seed);
    }
}

// First 1000 prime numbers;
#[allow(dead_code)]
pub const PRIME_TABLE: [usize; 1000] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
    809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
    937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019, 1021, 1031, 1033, 1039,
    1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093, 1097, 1103, 1109, 1117, 1123, 1129, 1151, 1153,
    1163, 1171, 1181, 1187, 1193, 1201, 1213, 1217, 1223, 1229, 1231, 1237, 1249, 1259, 1277, 1279,
    1283, 1289, 1291, 1297, 1301, 1303, 1307, 1319, 1321, 1327, 1361, 1367, 1373, 1381, 1399, 1409,
    1423, 1427, 1429, 1433, 1439, 1447, 1451, 1453, 1459, 1471, 1481, 1483, 1487, 1489, 1493, 1499,
    1511, 1523, 1531, 1543, 1549, 1553, 1559, 1567, 1571, 1579, 1583, 1597, 1601, 1607, 1609, 1613,
    1619, 1621, 1627, 1637, 1657, 1663, 1667, 1669, 1693, 1697, 1699, 1709, 1721, 1723, 1733, 1741,
    1747, 1753, 1759, 1777, 1783, 1787, 1789, 1801, 1811, 1823, 1831, 1847, 1861, 1867, 1871, 1873,
    1877, 1879, 1889, 1901, 1907, 1913, 1931, 1933, 1949, 1951, 1973, 1979, 1987, 1993, 1997, 1999,
    2003, 2011, 2017, 2027, 2029, 2039, 2053, 2063, 2069, 2081, 2083, 2087, 2089, 2099, 2111, 2113,
    2129, 2131, 2137, 2141, 2143, 2153, 2161, 2179, 2203, 2207, 2213, 2221, 2237, 2239, 2243, 2251,
    2267, 2269, 2273, 2281, 2287, 2293, 2297, 2309, 2311, 2333, 2339, 2341, 2347, 2351, 2357, 2371,
    2377, 2381, 2383, 2389, 2393, 2399, 2411, 2417, 2423, 2437, 2441, 2447, 2459, 2467, 2473, 2477,
    2503, 2521, 2531, 2539, 2543, 2549, 2551, 2557, 2579, 2591, 2593, 2609, 2617, 2621, 2633, 2647,
    2657, 2659, 2663, 2671, 2677, 2683, 2687, 2689, 2693, 2699, 2707, 2711, 2713, 2719, 2729, 2731,
    2741, 2749, 2753, 2767, 2777, 2789, 2791, 2797, 2801, 2803, 2819, 2833, 2837, 2843, 2851, 2857,
    2861, 2879, 2887, 2897, 2903, 2909, 2917, 2927, 2939, 2953, 2957, 2963, 2969, 2971, 2999, 3001,
    3011, 3019, 3023, 3037, 3041, 3049, 3061, 3067, 3079, 3083, 3089, 3109, 3119, 3121, 3137, 3163,
    3167, 3169, 3181, 3187, 3191, 3203, 3209, 3217, 3221, 3229, 3251, 3253, 3257, 3259, 3271, 3299,
    3301, 3307, 3313, 3319, 3323, 3329, 3331, 3343, 3347, 3359, 3361, 3371, 3373, 3389, 3391, 3407,
    3413, 3433, 3449, 3457, 3461, 3463, 3467, 3469, 3491, 3499, 3511, 3517, 3527, 3529, 3533, 3539,
    3541, 3547, 3557, 3559, 3571, 3581, 3583, 3593, 3607, 3613, 3617, 3623, 3631, 3637, 3643, 3659,
    3671, 3673, 3677, 3691, 3697, 3701, 3709, 3719, 3727, 3733, 3739, 3761, 3767, 3769, 3779, 3793,
    3797, 3803, 3821, 3823, 3833, 3847, 3851, 3853, 3863, 3877, 3881, 3889, 3907, 3911, 3917, 3919,
    3923, 3929, 3931, 3943, 3947, 3967, 3989, 4001, 4003, 4007, 4013, 4019, 4021, 4027, 4049, 4051,
    4057, 4073, 4079, 4091, 4093, 4099, 4111, 4127, 4129, 4133, 4139, 4153, 4157, 4159, 4177, 4201,
    4211, 4217, 4219, 4229, 4231, 4241, 4243, 4253, 4259, 4261, 4271, 4273, 4283, 4289, 4297, 4327,
    4337, 4339, 4349, 4357, 4363, 4373, 4391, 4397, 4409, 4421, 4423, 4441, 4447, 4451, 4457, 4463,
    4481, 4483, 4493, 4507, 4513, 4517, 4519, 4523, 4547, 4549, 4561, 4567, 4583, 4591, 4597, 4603,
    4621, 4637, 4639, 4643, 4649, 4651, 4657, 4663, 4673, 4679, 4691, 4703, 4721, 4723, 4729, 4733,
    4751, 4759, 4783, 4787, 4789, 4793, 4799, 4801, 4813, 4817, 4831, 4861, 4871, 4877, 4889, 4903,
    4909, 4919, 4931, 4933, 4937, 4943, 4951, 4957, 4967, 4969, 4973, 4987, 4993, 4999, 5003, 5009,
    5011, 5021, 5023, 5039, 5051, 5059, 5077, 5081, 5087, 5099, 5101, 5107, 5113, 5119, 5147, 5153,
    5167, 5171, 5179, 5189, 5197, 5209, 5227, 5231, 5233, 5237, 5261, 5273, 5279, 5281, 5297, 5303,
    5309, 5323, 5333, 5347, 5351, 5381, 5387, 5393, 5399, 5407, 5413, 5417, 5419, 5431, 5437, 5441,
    5443, 5449, 5471, 5477, 5479, 5483, 5501, 5503, 5507, 5519, 5521, 5527, 5531, 5557, 5563, 5569,
    5573, 5581, 5591, 5623, 5639, 5641, 5647, 5651, 5653, 5657, 5659, 5669, 5683, 5689, 5693, 5701,
    5711, 5717, 5737, 5741, 5743, 5749, 5779, 5783, 5791, 5801, 5807, 5813, 5821, 5827, 5839, 5843,
    5849, 5851, 5857, 5861, 5867, 5869, 5879, 5881, 5897, 5903, 5923, 5927, 5939, 5953, 5981, 5987,
    6007, 6011, 6029, 6037, 6043, 6047, 6053, 6067, 6073, 6079, 6089, 6091, 6101, 6113, 6121, 6131,
    6133, 6143, 6151, 6163, 6173, 6197, 6199, 6203, 6211, 6217, 6221, 6229, 6247, 6257, 6263, 6269,
    6271, 6277, 6287, 6299, 6301, 6311, 6317, 6323, 6329, 6337, 6343, 6353, 6359, 6361, 6367, 6373,
    6379, 6389, 6397, 6421, 6427, 6449, 6451, 6469, 6473, 6481, 6491, 6521, 6529, 6547, 6551, 6553,
    6563, 6569, 6571, 6577, 6581, 6599, 6607, 6619, 6637, 6653, 6659, 6661, 6673, 6679, 6689, 6691,
    6701, 6703, 6709, 6719, 6733, 6737, 6761, 6763, 6779, 6781, 6791, 6793, 6803, 6823, 6827, 6829,
    6833, 6841, 6857, 6863, 6869, 6871, 6883, 6899, 6907, 6911, 6917, 6947, 6949, 6959, 6961, 6967,
    6971, 6977, 6983, 6991, 6997, 7001, 7013, 7019, 7027, 7039, 7043, 7057, 7069, 7079, 7103, 7109,
    7121, 7127, 7129, 7151, 7159, 7177, 7187, 7193, 7207, 7211, 7213, 7219, 7229, 7237, 7243, 7247,
    7253, 7283, 7297, 7307, 7309, 7321, 7331, 7333, 7349, 7351, 7369, 7393, 7411, 7417, 7433, 7451,
    7457, 7459, 7477, 7481, 7487, 7489, 7499, 7507, 7517, 7523, 7529, 7537, 7541, 7547, 7549, 7559,
    7561, 7573, 7577, 7583, 7589, 7591, 7603, 7607, 7621, 7639, 7643, 7649, 7669, 7673, 7681, 7687,
    7691, 7699, 7703, 7717, 7723, 7727, 7741, 7753, 7757, 7759, 7789, 7793, 7817, 7823, 7829, 7841,
    7853, 7867, 7873, 7877, 7879, 7883, 7901, 7907, 7919,
];

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vdc() {
        assert_eq!(vdc(1, 2), 0.5);
        assert_eq!(vdc(2, 2), 0.25);
        assert_eq!(vdc(3, 2), 0.75);
        assert_eq!(vdc(4, 2), 0.125);
        assert_eq!(vdc(5, 2), 0.625);
    }
}
