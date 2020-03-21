//! Axis Aligned Bounding Boxes.

use std::f32;
use std::fmt;
use std::ops::Index;

// use ultraviolet::{Vec3, Vec3};
// use ultraviolet::Vec3;
use ultraviolet::vec::Vec3;

use crate::axis::Axis;

/// AABB struct.
#[derive(Debug, Copy, Clone)]
pub struct AABB {
    /// Minimum coordinates
    pub min: Vec3,

    /// Maximum coordinates
    pub max: Vec3,
}

// TODO: ultraviolet::vec::Vec3 doesn't implement fmt::Display
impl fmt::Display for AABB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Min bound: {:?}; Max bound: {:?}", self.min, self.max)
    }
}

/// A trait implemented by things which can be bounded by an [`AABB`].
///
/// [`AABB`]: struct.AABB.html
///
pub trait Bounded {
    /// Returns the geometric bounds of this object in the form of an [`AABB`].
    ///
    /// # Examples
    /// ```
    /// use bvh_ultraviolet::bounded::Bounded;
    /// use bvh_ultraviolet::ultraviolet::geometry::Aabb;
    /// use bvh_ultraviolet::ultraviolet::Vec3;
    ///
    /// struct Something;
    ///
    /// impl Bounded for Something {
    ///     fn aabb(&self) -> Aabb {
    ///         let point1 = Vec3::new(0.0,0.0,0.0);
    ///         let point2 = Vec3::new(1.0,1.0,1.0);
    ///         Aabb::new(point1, point2)
    ///     }
    /// }
    ///
    /// let something = Something;
    /// let aabb = something.aabb();
    ///
    /// assert!(aabb.contains(&Vec3::new(0.0,0.0,0.0)));
    /// assert!(aabb.contains(&Vec3::new(1.0,1.0,1.0)));
    /// ```
    ///
    /// [`AABB`]: struct.AABB.html
    ///
    fn aabb(&self) -> Aabb;
}

impl AABB {
    /// Creates a new [`AABB`] with the given bounds.
    ///
    /// # Examples
    /// ```
    /// use bvh_ultraviolet::ultraviolet::geometry::Aabb;
    /// use bvh_ultraviolet::ultraviolet::Vec3;
    ///
    /// let aabb = Aabb::new(Vec3::new(-1.0,-1.0,-1.0), Vec3::new(1.0,1.0,1.0));
    /// assert_eq!(aabb.min.x, -1.0);
    /// assert_eq!(aabb.max.z, 1.0);
    /// ```
    ///
    /// [`AABB`]: struct.AABB.html
    ///
    pub fn with_bounds(min: Vec3, max: Vec3) -> Aabb {
        AABB { min, max }
    }

    /// Creates a new empty [`AABB`].
    ///
    /// # Examples
    /// ```
    /// # extern crate bvh_ultraviolet;
    /// # extern crate rand;
    /// use bvh_ultraviolet::ultraviolet::geometry::Aabb;
    ///
    /// # fn main() {
    /// let aabb = AABB::empty();
    /// let min = &aabb.min;
    /// let max = &aabb.max;
    ///
    /// // For any point
    /// let x = rand::random();
    /// let y = rand::random();
    /// let z = rand::random();
    ///
    /// // An empty AABB should not contain it
    /// assert!(x < min.x && y < min.y && z < min.z);
    /// assert!(max.x < x && max.y < y && max.z < z);
    /// # }
    /// ```
    ///
    /// [`AABB`]: struct.AABB.html
    ///
    pub fn empty() -> Aabb {
        AABB {
            min: Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            max: Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
        }
    }

    /// Returns true if the [`Vec3`] is inside the [`AABB`].
    ///
    /// # Examples
    /// ```
    /// use bvh_ultraviolet::ultraviolet::geometry::Aabb;
    /// use bvh_ultraviolet::ultraviolet::Vec3;
    ///
    /// let aabb = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
    /// let point_inside = Vec3::new(0.125, -0.25, 0.5);
    /// let point_outside = Vec3::new(1.0, -2.0, 4.0);
    ///
    /// assert!(aabb.contains(&point_inside));
    /// assert!(!aabb.contains(&point_outside));
    /// ```
    ///
    /// [`AABB`]: struct.AABB.html
    /// [`Vec3`]: http://nalgebra.org/doc/nalgebra/struct.Vec3.html
    ///
    pub fn contains(&self, p: &Vec3) -> bool {
        p.x >= self.min.x
            && p.x <= self.max.x
            && p.y >= self.min.y
            && p.y <= self.max.y
            && p.z >= self.min.z
            && p.z <= self.max.z
    }

    /// Returns true if the [`Vec3`] is approximately inside the [`AABB`]
    /// with respect to some `epsilon`.
    ///
    /// # Examples
    /// ```
    /// use bvh_ultraviolet::EPSILON;
    /// use bvh_ultraviolet::ultraviolet::geometry::Aabb;
    /// use bvh_ultraviolet::ultraviolet::Vec3;
    ///
    /// let aabb = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
    /// let point_barely_outside = Vec3::new(1.000_000_1, -1.000_000_1, 1.000_000_001);
    /// let point_outside = Vec3::new(1.0, -2.0, 4.0);
    ///
    /// assert!(aabb.approx_contains_eps(&point_barely_outside, EPSILON));
    /// assert!(!aabb.approx_contains_eps(&point_outside, EPSILON));
    /// ```
    ///
    /// [`AABB`]: struct.AABB.html
    /// [`Vec3`]: http://nalgebra.org/doc/nalgebra/struct.Vec3.html
    ///
    pub fn approx_contains_eps(&self, p: &Vec3, epsilon: f32) -> bool {
        (p.x - self.min.x) > -epsilon
            && (p.x - self.max.x) < epsilon
            && (p.y - self.min.y) > -epsilon
            && (p.y - self.max.y) < epsilon
            && (p.z - self.min.z) > -epsilon
            && (p.z - self.max.z) < epsilon
    }

    /// Returns true if the `other` [`AABB`] is approximately inside this [`AABB`]
    /// with respect to some `epsilon`.
    ///
    /// # Examples
    /// ```
    /// use bvh_ultraviolet::EPSILON;
    /// use bvh_ultraviolet::ultraviolet::geometry::Aabb;
    /// use bvh_ultraviolet::ultraviolet::Vec3;
    ///
    /// let aabb = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
    /// let point_barely_outside = Vec3::new(1.000_000_1, 1.000_000_1, 1.000_000_1);
    /// let center = aabb.center();
    /// let inner_aabb = Aabb::new(center, point_barely_outside);
    ///
    /// assert!(aabb.approx_contains_aabb_eps(&inner_aabb, EPSILON));
    /// ```
    ///
    /// [`AABB`]: struct.AABB.html
    pub fn approx_contains_aabb_eps(&self, other: &AABB, epsilon: f32) -> bool {
        self.approx_contains_eps(&other.min, epsilon)
            && self.approx_contains_eps(&other.max, epsilon)
    }

    /// Returns true if the `other` [`AABB`] is approximately equal to this [`AABB`]
    /// with respect to some `epsilon`.
    ///
    /// # Examples
    /// ```
    /// use bvh_ultraviolet::EPSILON;
    /// use bvh_ultraviolet::ultraviolet::geometry::Aabb;
    /// use bvh_ultraviolet::ultraviolet::Vec3;
    ///
    /// let aabb = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
    /// let point_barely_outside_min = Vec3::new(-1.000_000_1, -1.000_000_1, -1.000_000_1);
    /// let point_barely_outside_max = Vec3::new(1.000_000_1, 1.000_000_1, 1.000_000_1);
    /// let other = Aabb::new(point_barely_outside_min, point_barely_outside_max);
    ///
    /// assert!(aabb.relative_eq(&other, EPSILON));
    /// ```
    ///
    /// [`AABB`]: struct.AABB.html
 

    // TODO: Profile this, see if it's the cause of the performance regressions vs the nalgebra version
    // Initial benchmarks of this implementation backported to nalgebra version and regular nalgebra version
    // is that it's only a noticeable difference for flat_bvh::bench::bench_build_* where it's ~20% slower.
    
    pub fn relative_eq(&self, other: &AABB, epsilon: f32) -> bool {
        // relative_eq!(self.min, other.min, epsilon = epsilon)
        //     && relative_eq!(self.max, other.max, epsilon = epsilon)

        fn relative_equal(a: Vec3, b: Vec3, epsilon: Vec3) -> bool {
            let a_hi = a + epsilon;
            let a_low = a - epsilon;
            let b_hi = b + epsilon;
            let b_low = b - epsilon;

            fn compare(hi: Vec3, low: Vec3) -> bool {
                (hi.x >= low.x) && (hi.y >= low.y) && (hi.z >= low.z)
            }

            compare(a_hi, b) && compare(b, a_low) && compare(a, b_low) && compare(b_hi, a)
        }

        let epsilon_vec = Vec3::new(epsilon, epsilon, epsilon);

        relative_equal(self.min, other.min, epsilon_vec)
            && relative_equal(self.max, other.max, epsilon_vec)
    }

    /// Returns a new minimal [`AABB`] which contains both this [`AABB`] and `other`.
    /// The result is the convex hull of the both [`AABB`]s.
    ///
    /// # Examples
    /// ```
    /// use bvh_ultraviolet::ultraviolet::geometry::Aabb;
    /// use bvh_ultraviolet::ultraviolet::Vec3;
    ///
    /// let aabb1 = Aabb::new(Vec3::new(-101.0, 0.0, 0.0), Vec3::new(-100.0, 1.0, 1.0));
    /// let aabb2 = Aabb::new(Vec3::new(100.0, 0.0, 0.0), Vec3::new(101.0, 1.0, 1.0));
    /// let joint = aabb1.join(&aabb2);
    ///
    /// let point_inside_aabb1 = Vec3::new(-100.5, 0.5, 0.5);
    /// let point_inside_aabb2 = Vec3::new(100.5, 0.5, 0.5);
    /// let point_inside_joint = Vec3::new(0.0, 0.5, 0.5);
    ///
    /// # assert!(aabb1.contains(&point_inside_aabb1));
    /// # assert!(!aabb1.contains(&point_inside_aabb2));
    /// # assert!(!aabb1.contains(&point_inside_joint));
    /// #
    /// # assert!(!aabb2.contains(&point_inside_aabb1));
    /// # assert!(aabb2.contains(&point_inside_aabb2));
    /// # assert!(!aabb2.contains(&point_inside_joint));
    ///
    /// assert!(joint.contains(&point_inside_aabb1));
    /// assert!(joint.contains(&point_inside_aabb2));
    /// assert!(joint.contains(&point_inside_joint));
    /// ```
    ///
    /// [`AABB`]: struct.AABB.html
    ///
    pub fn join(&self, other: &AABB) -> Aabb {
        Aabb::new(
            Vec3::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            Vec3::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        )
    }

    /// Mutable version of [`AABB::join`].
    ///
    /// # Examples
    /// ```
    /// use bvh_ultraviolet::ultraviolet::geometry::Aabb;
    /// use bvh_ultraviolet::ultraviolet::Vec3;
    ///
    /// let size = Vec3::new(1.0, 1.0, 1.0);
    /// let aabb_pos = Vec3::new(-101.0, 0.0, 0.0);
    /// let mut aabb = Aabb::new(aabb_pos, aabb_pos + size);
    ///
    /// let other_pos = Vec3::new(100.0, 0.0, 0.0);
    /// let other = Aabb::new(other_pos, other_pos + size);
    ///
    /// let point_inside_aabb = aabb_pos + size / 2.0;
    /// let point_inside_other = other_pos + size / 2.0;
    /// let point_inside_joint = Vec3::new(0.0, 0.0, 0.0) + size / 2.0;
    ///
    /// # assert!(aabb.contains(&point_inside_aabb));
    /// # assert!(!aabb.contains(&point_inside_other));
    /// # assert!(!aabb.contains(&point_inside_joint));
    /// #
    /// # assert!(!other.contains(&point_inside_aabb));
    /// # assert!(other.contains(&point_inside_other));
    /// # assert!(!other.contains(&point_inside_joint));
    ///
    /// aabb.join_mut(&other);
    ///
    /// assert!(aabb.contains(&point_inside_aabb));
    /// assert!(aabb.contains(&point_inside_other));
    /// assert!(aabb.contains(&point_inside_joint));
    /// ```
    ///
    /// [`AABB::join`]: struct.AABB.html
    ///
    pub fn join_mut(&mut self, other: &AABB) {
        self.min = Vec3::new(
            self.min.x.min(other.min.x),
            self.min.y.min(other.min.y),
            self.min.z.min(other.min.z),
        );
        self.max = Vec3::new(
            self.max.x.max(other.max.x),
            self.max.y.max(other.max.y),
            self.max.z.max(other.max.z),
        );
    }

    /// Returns a new minimal [`AABB`] which contains both
    /// this [`AABB`] and the [`Vec3`] `other`.
    ///
    /// # Examples
    /// ```
    /// use bvh_ultraviolet::ultraviolet::geometry::Aabb;
    /// use bvh_ultraviolet::ultraviolet::Vec3;
    ///
    /// let point1 = Vec3::new(0.0, 0.0, 0.0);
    /// let point2 = Vec3::new(1.0, 1.0, 1.0);
    /// let Vec3 = Vec3::new(2.0, 2.0, 2.0);
    ///
    /// let aabb = AABB::empty();
    /// assert!(!aabb.contains(&point1));
    ///
    /// let aabb1 = aabb.grow(&point1);
    /// assert!(aabb1.contains(&point1));
    ///
    /// let aabb2 = aabb.grow(&point2);
    /// assert!(aabb2.contains(&point2));
    /// assert!(!aabb2.contains(&Vec3));
    /// ```
    ///
    /// [`AABB`]: struct.AABB.html
    /// [`Vec3`]: http://nalgebra.org/doc/nalgebra/struct.Vec3.html
    ///
    pub fn grow(&self, other: &Vec3) -> Aabb {
        Aabb::new(
            Vec3::new(
                self.min.x.min(other.x),
                self.min.y.min(other.y),
                self.min.z.min(other.z),
            ),
            Vec3::new(
                self.max.x.max(other.x),
                self.max.y.max(other.y),
                self.max.z.max(other.z),
            ),
        )
    }

    /// Mutable version of [`AABB::grow`].
    ///
    /// # Examples
    /// ```
    /// use bvh_ultraviolet::ultraviolet::geometry::Aabb;
    /// use bvh_ultraviolet::ultraviolet::Vec3;
    ///
    /// let point1 = Vec3::new(0.0, 0.0, 0.0);
    /// let point2 = Vec3::new(1.0, 1.0, 1.0);
    /// let Vec3 = Vec3::new(2.0, 2.0, 2.0);
    ///
    /// let mut aabb = AABB::empty();
    /// assert!(!aabb.contains(&point1));
    ///
    /// aabb.grow_mut(&point1);
    /// assert!(aabb.contains(&point1));
    /// assert!(!aabb.contains(&point2));
    ///
    /// aabb.grow_mut(&point2);
    /// assert!(aabb.contains(&point2));
    /// assert!(!aabb.contains(&Vec3));
    /// ```
    ///
    /// [`AABB::grow`]: struct.AABB.html
    /// [`Vec3`]: http://nalgebra.org/doc/nalgebra/struct.Vec3.html
    ///
    pub fn grow_mut(&mut self, other: &Vec3) {
        self.min = Vec3::new(
            self.min.x.min(other.x),
            self.min.y.min(other.y),
            self.min.z.min(other.z),
        );
        self.max = Vec3::new(
            self.max.x.max(other.x),
            self.max.y.max(other.y),
            self.max.z.max(other.z),
        );
    }

    /// Returns a new minimal [`AABB`] which contains both this [`AABB`] and the [`Bounded`]
    /// `other`.
    ///
    /// # Examples
    /// ```
    /// use bvh_ultraviolet::bounded::Bounded;
    /// use bvh_ultraviolet::ultraviolet::geometry::Aabb;
    /// use bvh_ultraviolet::ultraviolet::Vec3;
    ///
    /// struct Something;
    ///
    /// impl Bounded for Something {
    ///     fn aabb(&self) -> Aabb {
    ///         let point1 = Vec3::new(0.0,0.0,0.0);
    ///         let point2 = Vec3::new(1.0,1.0,1.0);
    ///         Aabb::new(point1, point2)
    ///     }
    /// }
    ///
    /// let aabb = AABB::empty();
    /// let something = Something;
    /// let aabb1 = aabb.join_bounded(&something);
    ///
    /// let center = something.aabb().center();
    /// assert!(aabb1.contains(&center));
    /// ```
    ///
    /// [`AABB`]: struct.AABB.html
    /// [`Bounded`]: trait.Bounded.html
    ///
    pub fn join_bounded<T: Bounded>(&self, other: &T) -> Aabb {
        self.join(&other.aabb())
    }

    /// Returns the size of this [`AABB`] in all three dimensions.
    ///
    /// # Examples
    /// ```
    /// use bvh_ultraviolet::ultraviolet::geometry::Aabb;
    /// use bvh_ultraviolet::ultraviolet::Vec3;
    ///
    /// let aabb = Aabb::new(Vec3::new(-1.0,-1.0,-1.0), Vec3::new(1.0,1.0,1.0));
    /// let size = aabb.size();
    /// assert!(size.x == 2.0 && size.y == 2.0 && size.z == 2.0);
    /// ```
    ///
    /// [`AABB`]: struct.AABB.html
    ///
    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    /// Returns the center [`Vec3`] of the [`AABB`].
    ///
    /// # Examples
    /// ```
    /// use bvh_ultraviolet::ultraviolet::geometry::Aabb;
    /// use bvh_ultraviolet::ultraviolet::Vec3;
    ///
    /// let min = Vec3::new(41.0,41.0,41.0);
    /// let max = Vec3::new(43.0,43.0,43.0);
    ///
    /// let aabb = Aabb::new(min, max);
    /// let center = aabb.center();
    /// assert!(center.x == 42.0 && center.y == 42.0 && center.z == 42.0);
    /// ```
    ///
    /// [`AABB`]: struct.AABB.html
    /// [`Vec3`]: http://nalgebra.org/doc/nalgebra/struct.Vec3.html
    ///
    pub fn center(&self) -> Vec3 {
        self.min + (self.size() / 2.0)
    }

    /// An empty [`AABB`] is an [`AABB`] where the lower bound is greater than
    /// the upper bound in at least one component
    ///
    /// # Examples
    /// ```
    /// use bvh_ultraviolet::ultraviolet::geometry::Aabb;
    /// use bvh_ultraviolet::ultraviolet::Vec3;
    ///
    /// let empty_aabb = AABB::empty();
    /// assert!(empty_aabb.is_empty());
    ///
    /// let min = Vec3::new(41.0,41.0,41.0);
    /// let max = Vec3::new(43.0,43.0,43.0);
    ///
    /// let aabb = Aabb::new(min, max);
    /// assert!(!aabb.is_empty());
    /// ```
    ///
    /// [`AABB`]: struct.AABB.html
    ///
    pub fn is_empty(&self) -> bool {
        self.min.x > self.max.x || self.min.y > self.max.y || self.min.z > self.max.z
    }

    /// Returns the total surface area of this [`AABB`].
    ///
    /// # Examples
    /// ```
    /// use bvh_ultraviolet::ultraviolet::geometry::Aabb;
    /// use bvh_ultraviolet::ultraviolet::Vec3;
    ///
    /// let min = Vec3::new(41.0,41.0,41.0);
    /// let max = Vec3::new(43.0,43.0,43.0);
    ///
    /// let aabb = Aabb::new(min, max);
    /// let surface_area = aabb.surface_area();
    /// assert!(surface_area == 24.0);
    /// ```
    ///
    /// [`AABB`]: struct.AABB.html
    ///
    pub fn surface_area(&self) -> f32 {
        let size = self.size();
        2.0 * (size.x * size.y + size.x * size.z + size.y * size.z)
    }

    /// Returns the volume of this [`AABB`].
    ///
    /// # Examples
    /// ```
    /// use bvh_ultraviolet::ultraviolet::geometry::Aabb;
    /// use bvh_ultraviolet::ultraviolet::Vec3;
    ///
    /// let min = Vec3::new(41.0,41.0,41.0);
    /// let max = Vec3::new(43.0,43.0,43.0);
    ///
    /// let aabb = Aabb::new(min, max);
    /// let volume = aabb.volume();
    /// assert!(volume == 8.0);
    /// ```
    ///
    /// [`AABB`]: struct.AABB.html
    ///
    pub fn volume(&self) -> f32 {
        let size = self.size();
        size.x * size.y * size.z
    }

    /// Returns the axis along which the [`AABB`] is stretched the most.
    ///
    /// # Examples
    /// ```
    /// use bvh_ultraviolet::ultraviolet::geometry::Aabb;
    /// use bvh_ultraviolet::axis::Axis;
    /// use bvh_ultraviolet::ultraviolet::Vec3;
    ///
    /// let min = Vec3::new(-100.0,0.0,0.0);
    /// let max = Vec3::new(100.0,0.0,0.0);
    ///
    /// let aabb = Aabb::new(min, max);
    /// let axis = aabb.largest_axis();
    /// assert!(axis == Axis::X);
    /// ```
    ///
    /// [`AABB`]: struct.AABB.html
    ///
    pub fn largest_axis(&self) -> Axis {
        let size = self.size();
        if size.x > size.y && size.x > size.z {
            Axis::X
        } else if size.y > size.z {
            Axis::Y
        } else {
            Axis::Z
        }
    }
}

/// Default instance for [`AABB`]s. Returns an [`AABB`] which is [`empty()`].
///
/// [`AABB`]: struct.AABB.html
/// [`empty()`]: #method.empty
///
impl Default for AABB {
    fn default() -> Aabb {
        AABB::empty()
    }
}

/// Make [`AABB`]s indexable. `aabb[0]` gives a reference to the minimum bound.
/// All other indices return a reference to the maximum bound.
///
/// # Examples
/// ```
/// use bvh_ultraviolet::ultraviolet::geometry::Aabb;
/// use bvh_ultraviolet::ultraviolet::Vec3;
///
/// let min = Vec3::new(3.0,4.0,5.0);
/// let max = Vec3::new(123.0,123.0,123.0);
///
/// let aabb = Aabb::new(min, max);
/// 
/// # for i in 0..3 {
/// assert_eq!(aabb[0][i], min[i]);
/// assert_eq!(aabb[1][i], max[i]);
/// # }
/// ```
///
/// [`AABB`]: struct.AABB.html
///
impl Index<usize> for AABB {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Vec3 {
        if index == 0 {
            &self.min
        } else {
            &self.max
        }
    }
}

/// Implementation of [`Bounded`] for [`AABB`].
///
/// # Examples
/// ```
/// use bvh_ultraviolet::bounded::Bounded;
/// use bvh_ultraviolet::ultraviolet::geometry::Aabb;
/// use bvh_ultraviolet::ultraviolet::Vec3;
///
/// let point_a = Vec3::new(3.0,4.0,5.0);
/// let point_b = Vec3::new(17.0,18.0,19.0);
/// let aabb = AABB::empty().grow(&point_a).grow(&point_b);
///
/// let aabb_aabb = aabb.aabb();
///
/// # for i in 0..3 {
/// assert_eq!(aabb_aabb.min[i], aabb.min[i]);
/// assert_eq!(aabb_aabb.max[i], aabb.max[i]);
/// # }
/// ```
///
/// [`AABB`]: struct.AABB.html
/// [`Bounded`]: trait.Bounded.html
///
impl Bounded for AABB {
    fn aabb(&self) -> Aabb {
        *self
    }
}

/// Implementation of [`Bounded`] for [`Vec3`].
///
/// # Examples
/// ```
/// use bvh_ultraviolet::bounded::Bounded;
/// use bvh_ultraviolet::ultraviolet::geometry::Aabb;
/// use bvh_ultraviolet::ultraviolet::Vec3;
///
/// let point = Vec3::new(3.0,4.0,5.0);
///
/// let aabb = point.aabb();
/// assert!(aabb.contains(&point));
/// ```
///
/// [`Bounded`]: trait.Bounded.html
/// [`Vec3`]: http://nalgebra.org/doc/nalgebra/struct.Vec3.html
///
impl Bounded for Vec3 {
    fn aabb(&self) -> Aabb {
        Aabb::new(*self, *self)
    }
}

#[cfg(test)]
mod tests {
    use crate::aabb::{Bounded, AABB};
    use crate::testbase::{tuple_to_point, tuple_to_vector, TupleVec};
    use crate::EPSILON;

    use ultraviolet::Vec3;
    // use ultraviolet::{Vec3, Vec3};
    use quickcheck::quickcheck;

    /// Test whether an empty `AABB` does not contains anything.
    quickcheck! {
        fn test_empty_contains_nothing(tpl: TupleVec) -> bool {
            // Define a random Point
            let p = tuple_to_point(&tpl);

            // Create an empty AABB
            let aabb = AABB::empty();

            // It should not contain anything
            !aabb.contains(&p)
        }
    }

    /// Test whether a default `AABB` is empty.
    quickcheck! {
        fn test_default_is_empty(tpl: TupleVec) -> bool {
            // Define a random Point
            let p = tuple_to_point(&tpl);

            // Create a default AABB
            let aabb: AABB = Default::default();

            // It should not contain anything
            !aabb.contains(&p)
        }
    }

    /// Test whether an `AABB` always contains its center.
    quickcheck! {
        fn test_aabb_contains_center(a: TupleVec, b: TupleVec) -> bool {
            // Define two points which will be the corners of the `AABB`
            let p1 = tuple_to_point(&a);
            let p2 = tuple_to_point(&b);

            // Span the `AABB`
            let aabb = AABB::empty().grow(&p1).join_bounded(&p2);

            // Its center should be inside the `AABB`
            aabb.contains(&aabb.center())
        }
    }

    /// Test whether the joint of two point-sets contains all the points.
    quickcheck! {
        fn test_join_two_aabbs(a: (TupleVec, TupleVec, TupleVec, TupleVec, TupleVec),
                               b: (TupleVec, TupleVec, TupleVec, TupleVec, TupleVec))
                               -> bool {
            // Define an array of ten points
            let points = [a.0, a.1, a.2, a.3, a.4, b.0, b.1, b.2, b.3, b.4];

            // Convert these points to `Vec3`
            let points = points.iter().map(tuple_to_point).collect::<Vec<Vec3>>();

            // Create two `AABB`s. One spanned the first five points,
            // the other by the last five points
            let aabb1 = points.iter().take(5).fold(AABB::empty(), |aabb, point| aabb.grow(&point));
            let aabb2 = points.iter().skip(5).fold(AABB::empty(), |aabb, point| aabb.grow(&point));

            // The `AABB`s should contain the points by which they are spanned
            let aabb1_contains_init_five = points.iter()
                .take(5)
                .fold(true, |b, point| b && aabb1.contains(&point));
            let aabb2_contains_last_five = points.iter()
                .skip(5)
                .fold(true, |b, point| b && aabb2.contains(&point));

            // Build the joint of the two `AABB`s
            let aabbu = aabb1.join(&aabb2);

            // The joint should contain all points
            let aabbu_contains_all = points.iter()
                .fold(true, |b, point| b && aabbu.contains(&point));

            // Return the three properties
            aabb1_contains_init_five && aabb2_contains_last_five && aabbu_contains_all
        }
    }

    /// Test whether some points relative to the center of an AABB are classified correctly.
    quickcheck! {
        fn test_points_relative_to_center_and_size(a: TupleVec, b: TupleVec) -> bool {
            // Generate some nonempty AABB
            let aabb = AABB::empty()
                .grow(&tuple_to_point(&a))
                .grow(&tuple_to_point(&b));

            // Get its size and center
            let size = aabb.size();
            let size_half = size / 2.0;
            let center = aabb.center();

            // Compute the min and the max corners of the AABB by hand
            let inside_ppp = center + size_half;
            let inside_mmm = center - size_half;

            // Generate two points which are outside the AABB
            let outside_ppp = inside_ppp + Vec3::new(0.1, 0.1, 0.1);
            let outside_mmm = inside_mmm - Vec3::new(0.1, 0.1, 0.1);

            assert!(aabb.approx_contains_eps(&inside_ppp, EPSILON));
            assert!(aabb.approx_contains_eps(&inside_mmm, EPSILON));
            assert!(!aabb.contains(&outside_ppp));
            assert!(!aabb.contains(&outside_mmm));

            true
        }
    }

    /// Test whether the surface of a nonempty AABB is always positive.
    quickcheck! {
        fn test_surface_always_positive(a: TupleVec, b: TupleVec) -> bool {
            let aabb = AABB::empty()
                .grow(&tuple_to_point(&a))
                .grow(&tuple_to_point(&b));
            aabb.surface_area() >= 0.0
        }
    }

    /// Compute and compare the surface area of an AABB by hand.
    quickcheck! {
        fn test_surface_area_cube(pos: TupleVec, size: f32) -> bool {
            // Generate some non-empty AABB
            let pos = tuple_to_point(&pos);
            let size_vec = Vec3::new(size, size, size);
            let aabb = Aabb::new(pos, pos + size_vec);

            // Check its surface area
            let area_a = aabb.surface_area();
            let area_b = 6.0 * size * size;
            (1.0 - (area_a / area_b)).abs() < EPSILON
        }
    }

    /// Test whether the volume of a nonempty AABB is always positive.
    quickcheck! {
        fn test_volume_always_positive(a: TupleVec, b: TupleVec) -> bool {
            let aabb = AABB::empty()
                .grow(&tuple_to_point(&a))
                .grow(&tuple_to_point(&b));
            aabb.volume() >= 0.0
        }
    }

    /// Compute and compare the volume of an AABB by hand.
    quickcheck! {
        fn test_volume_by_hand(pos: TupleVec, size: TupleVec) -> bool {
            // Generate some non-empty AABB
            let pos = tuple_to_point(&pos);
            let size = tuple_to_vector(&size);
            let aabb = pos.aabb().grow(&(pos + size));

            // Check its volume
            let volume_a = aabb.volume();
            let volume_b = (size.x * size.y * size.z).abs();
            (1.0 - (volume_a / volume_b)).abs() < EPSILON
        }
    }

    /// Test whether generating an `AABB` from the min and max bounds yields the same `AABB`.
    quickcheck! {
        fn test_create_aabb_from_indexable(a: TupleVec, b: TupleVec, p: TupleVec) -> bool {
            // Create a random point
            let point = tuple_to_point(&p);

            // Create a random AABB
            let aabb = AABB::empty()
                .grow(&tuple_to_point(&a))
                .grow(&tuple_to_point(&b));

            // Create an AABB by using the index-access method
            let aabb_by_index = Aabb::new(aabb[0], aabb[1]);

            // The AABBs should be the same
            aabb.contains(&point) == aabb_by_index.contains(&point)
        }
    }
}
