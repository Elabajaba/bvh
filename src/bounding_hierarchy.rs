//! This module defines the `BoundingHierarchy` trait.

use crate::bounded::Bounded;
use crate::ray::Ray;

/// Describes a shape as referenced by a [`BoundingHierarchy`] leaf node.
/// Knows the index of the node in the [`BoundingHierarchy`] it is in.
///
/// [`BoundingHierarchy`]: struct.BoundingHierarchy.html
///
pub trait BHShape: Bounded {
    /// Sets the index of the referenced [`BoundingHierarchy`] node.
    ///
    /// [`BoundingHierarchy`]: struct.BoundingHierarchy.html
    ///
    fn set_bh_node_index(&mut self, _: usize);

    /// Gets the index of the referenced [`BoundingHierarchy`] node.
    ///
    /// [`BoundingHierarchy`]: struct.BoundingHierarchy.html
    ///
    fn bh_node_index(&self) -> usize;
}

/// This trait defines an acceleration structure with space partitioning.
/// This structure is used to efficiently compute ray-scene intersections.
pub trait BoundingHierarchy {
    /// Creates a new [`BoundingHierarchy`] from the `shapes` slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use bvh_ultraviolet::bounded::Bounded;
    /// use bvh_ultraviolet::bounding_hierarchy::BoundingHierarchy;
    /// use bvh_ultraviolet::ultraviolet::geometry::Aabb;
    /// use bvh_ultraviolet::ultraviolet::Vec3;
    /// # use bvh_ultraviolet::bounding_hierarchy::BHShape;
    /// # pub struct UnitBox {
    /// #     pub id: i32,
    /// #     pub pos: Vec3,
    /// #     node_index: usize,
    /// # }
    /// #
    /// # impl UnitBox {
    /// #     pub fn new(id: i32, pos: Vec3) -> UnitBox {
    /// #         UnitBox {
    /// #             id: id,
    /// #             pos: pos,
    /// #             node_index: 0,
    /// #         }
    /// #     }
    /// # }
    /// #
    /// # impl Bounded for UnitBox {
    /// #     fn aabb(&self) -> Aabb {
    /// #         let min = self.pos + Vec3::new(-0.5, -0.5, -0.5);
    /// #         let max = self.pos + Vec3::new(0.5, 0.5, 0.5);
    /// #         Aabb::new(min, max)
    /// #     }
    /// # }
    /// #
    /// # impl BHShape for UnitBox {
    /// #     fn set_bh_node_index(&mut self, index: usize) {
    /// #         self.node_index = index;
    /// #     }
    /// #
    /// #     fn bh_node_index(&self) -> usize {
    /// #         self.node_index
    /// #     }
    /// # }
    /// #
    /// # fn create_bhshapes() -> Vec<UnitBox> {
    /// #     let mut shapes = Vec::new();
    /// #     for i in 0..1000 {
    /// #         let position = Vec3::new(i as f32, i as f32, i as f32);
    /// #         shapes.push(UnitBox::new(i, position));
    /// #     }
    /// #     shapes
    /// # }
    ///
    /// let mut shapes = create_bhshapes();
    /// // Construct a normal `BVH`.
    /// {
    ///     use bvh_ultraviolet::bvh::BVH;
    ///     let bvh = BVH::build(&mut shapes);
    /// }
    ///
    /// // Or construct a `FlatBVH`.
    /// {
    ///     use bvh_ultraviolet::flat_bvh::FlatBVH;
    ///     let bvh = FlatBVH::build(&mut shapes);
    /// }
    /// ```
    ///
    /// [`BoundingHierarchy`]: trait.BoundingHierarchy.html
    ///
    fn build<Shape: BHShape>(shapes: &mut [Shape]) -> Self;

    /// Traverses the [`BoundingHierarchy`].
    /// Returns a subset of `shapes`, in which the [`AABB`]s of the elements were hit by `ray`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bvh_ultraviolet::bounded::Bounded;
    /// use bvh_ultraviolet::ultraviolet::geometry::Aabb;
    /// use bvh_ultraviolet::bounding_hierarchy::BoundingHierarchy;
    /// use bvh_ultraviolet::bvh::BVH;
    /// use bvh_ultraviolet::ultraviolet::Vec3;
    /// use bvh_ultraviolet::ray::Ray;
    /// # use bvh_ultraviolet::bounding_hierarchy::BHShape;
    /// # pub struct UnitBox {
    /// #     pub id: i32,
    /// #     pub pos: Vec3,
    /// #     node_index: usize,
    /// # }
    /// #
    /// # impl UnitBox {
    /// #     pub fn new(id: i32, pos: Vec3) -> UnitBox {
    /// #         UnitBox {
    /// #             id: id,
    /// #             pos: pos,
    /// #             node_index: 0,
    /// #         }
    /// #     }
    /// # }
    /// #
    /// # impl Bounded for UnitBox {
    /// #     fn aabb(&self) -> Aabb {
    /// #         let min = self.pos + Vec3::new(-0.5, -0.5, -0.5);
    /// #         let max = self.pos + Vec3::new(0.5, 0.5, 0.5);
    /// #         Aabb::new(min, max)
    /// #     }
    /// # }
    /// #
    /// # impl BHShape for UnitBox {
    /// #     fn set_bh_node_index(&mut self, index: usize) {
    /// #         self.node_index = index;
    /// #     }
    /// #
    /// #     fn bh_node_index(&self) -> usize {
    /// #         self.node_index
    /// #     }
    /// # }
    /// #
    /// # fn create_bvh() -> (BVH, Vec<UnitBox>) {
    /// #     let mut shapes = Vec::new();
    /// #     for i in 0..1000 {
    /// #         let position = Vec3::new(i as f32, i as f32, i as f32);
    /// #         shapes.push(UnitBox::new(i, position));
    /// #     }
    /// #     let bvh = BVH::build(&mut shapes);
    /// #     (bvh, shapes)
    /// # }
    ///
    /// let (bvh, shapes) = create_bvh();
    ///
    /// let origin = Vec3::new(0.0, 0.0, 0.0);
    /// let direction = Vec3::new(1.0, 0.0, 0.0);
    /// let ray = Ray::new(origin, direction);
    /// let hit_shapes = bvh.traverse(&ray, &shapes);
    /// ```
    ///
    /// [`BoundingHierarchy`]: trait.BoundingHierarchy.html
    /// [`AABB`]: ../aabb/struct.AABB.html
    ///
    fn traverse<'a, Shape: BHShape>(&'a self, ray: &Ray, shapes: &'a [Shape]) -> Vec<&Shape>;

    /// Prints the [`BoundingHierarchy`] in a tree-like visualization.
    ///
    /// [`BoundingHierarchy`]: trait.BoundingHierarchy.html
    ///
    fn pretty_print(&self) {}
}
