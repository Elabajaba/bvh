//! Utilities module.

use crate::bounding_hierarchy::BHShape;
use ultraviolet::geometry::Aabb;

/// Concatenates the list of vectors into a single vector.
/// Drains the elements from the source `vectors`.
pub fn concatenate_vectors<T: Sized>(vectors: &mut [Vec<T>]) -> Vec<T> {
    let mut result = Vec::new();
    for mut vector in vectors.iter_mut() {
        result.append(&mut vector);
    }
    result
}

/// Defines a Bucket utility object. Used to store the properties of shape-partitions
/// in the BVH build procedure using SAH.
#[derive(Copy, Clone)]
pub struct Bucket {
    /// The number of shapes in this `Bucket`.
    pub size: usize,

    /// The joint `Aabb` of the shapes in this `Bucket`.
    pub aabb: Option<Aabb>,
}

impl Bucket {
    /// Returns an empty bucket.
    pub fn empty() -> Bucket {
        Bucket {
            size: 0,
            aabb: None,
        }
    }

    /// Extend this `Bucket` by a shape with the given `Aabb`.
    pub fn add_aabb(&mut self, aabb: &Aabb) {
        self.size += 1;
        self.aabb = match self.aabb {
            Some(bbox) => Some(bbox.join(aabb)),
            None => Some(*aabb),
        }
    }

    /// Join the contents of two `Bucket`s.
    pub fn join_bucket(a: Bucket, b: &Bucket) -> Bucket {
        Bucket {
            size: a.size + b.size,
            aabb: match (a.aabb, b.aabb) {
                (None, None) => None,
                (Some(a_bbox), None) => Some(a_bbox),
                (None, Some(b_bbox)) => Some(b_bbox),
                (Some(a_bbox), Some(b_bbox)) => Some(a_bbox.join(&b_bbox))
            } //Some(a.aabb.unwrap().join(&b.aabb.unwrap())),
        }
    }
}

pub fn joint_aabb_of_shapes<Shape: BHShape>(indices: &[usize], shapes: &[Shape]) -> Aabb {
    let mut aabb: Option<Aabb> = None;
    for index in indices {
        let shape = &shapes[*index];
        aabb = match aabb {
            Some(bbox) => Some(bbox.join(&shape.aabb())),
            None => Some(shape.aabb()),
        };
        // aabb.unwrap().join(&shape.aabb());
    }
    match aabb {
        Some(bbox) => bbox,
        None => panic!()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::concatenate_vectors;

    #[test]
    /// Test if concatenating no `Vec`s yields an empty `Vec`.
    fn test_concatenate_empty() {
        let mut vectors: Vec<Vec<usize>> = vec![];
        let expected = vec![];
        assert_eq!(concatenate_vectors(vectors.as_mut_slice()), expected);
        let expected_remainder: Vec<Vec<usize>> = vec![];
        assert_eq!(vectors, expected_remainder);
    }

    #[test]
    /// Test if concatenating some `Vec`s yields the concatenation of the vectors.
    fn test_concatenate_vectors() {
        let mut vectors = vec![vec![1, 2, 3], vec![], vec![4, 5, 6], vec![7, 8], vec![9]];
        let result = concatenate_vectors(vectors.as_mut_slice());
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(result, expected);
        assert_eq!(vectors, vec![vec![], vec![], vec![], vec![], vec![]]);
    }
}
