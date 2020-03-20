# bvh
[![dependency status](https://deps.rs/repo/github/svenstaro/bvh/status.svg)](https://deps.rs/repo/github/Elabajaba/bvh)
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Elabajaba/bvh/LICENSE)

**A quick and dirty port of the [bvh](https://github.com/svenstaro/bvh) crate from [nalgebra](http://nalgebra.org/) to [ultraviolet](https://docs.rs/crate/ultraviolet/0.4.5). It currently does not take advantage of the packed wec data types offered by the ultraviolet crate.**

**A crate which exports rays, axis-aligned bounding boxes, and binary bounding
volume hierarchies.**

Benchmark of my initial implementation compared to the nalgebra version.

```
 name                                                                 nalgebra ns/iter  ultraviolet ns/iter  diff ns/iter   diff %  speedup 
 bvh::optimization::bench::bench_intersect_120k_after_optimize_00p    952               1,099                         147   15.44%   x 0.87 
 bvh::optimization::bench::bench_intersect_120k_after_optimize_01p    166,183           174,379                     8,196    4.93%   x 0.95 
 bvh::optimization::bench::bench_intersect_120k_after_optimize_10p    1,187,026         1,353,476                 166,450   14.02%   x 0.88 
 bvh::optimization::bench::bench_intersect_120k_after_optimize_50p    1,949,999         2,267,949                 317,950   16.31%   x 0.86 
 bvh::optimization::bench::bench_intersect_120k_with_rebuild_00p      950               1,093                         143   15.05%   x 0.87 
 bvh::optimization::bench::bench_intersect_120k_with_rebuild_01p      1,021             1,169                         148   14.50%   x 0.87 
 bvh::optimization::bench::bench_intersect_120k_with_rebuild_10p      2,004             2,238                         234   11.68%   x 0.90 
 bvh::optimization::bench::bench_intersect_120k_with_rebuild_50p      2,252             2,501                         249   11.06%   x 0.90 
 bvh::optimization::bench::bench_intersect_sponza_after_optimize_00p  1,659             1,994                         335   20.19%   x 0.83 
 bvh::optimization::bench::bench_intersect_sponza_after_optimize_01p  3,280             3,937                         657   20.03%   x 0.83 
 bvh::optimization::bench::bench_intersect_sponza_after_optimize_10p  4,416             5,111                         695   15.74%   x 0.86 
 bvh::optimization::bench::bench_intersect_sponza_after_optimize_50p  6,924             7,835                         911   13.16%   x 0.88 
 bvh::optimization::bench::bench_intersect_sponza_with_rebuild_00p    1,651             1,996                         345   20.90%   x 0.83 
 bvh::optimization::bench::bench_intersect_sponza_with_rebuild_01p    1,782             2,138                         356   19.98%   x 0.83 
 bvh::optimization::bench::bench_intersect_sponza_with_rebuild_10p    2,177             2,590                         413   18.97%   x 0.84 
 bvh::optimization::bench::bench_intersect_sponza_with_rebuild_50p    2,783             3,270                         487   17.50%   x 0.85 
 bvh::optimization::bench::bench_optimize_bvh_120k_00p                1,086,596         1,351,638                 265,042   24.39%   x 0.80 
 bvh::optimization::bench::bench_optimize_bvh_120k_01p                9,970,866         7,983,913              -1,986,953  -19.93%   x 1.25 
 bvh::optimization::bench::bench_optimize_bvh_120k_10p                72,811,524        53,253,556            -19,557,968  -26.86%   x 1.37 
 bvh::optimization::bench::bench_optimize_bvh_120k_50p                302,252,655       215,600,127           -86,652,528  -28.67%   x 1.40 
 bvh::optimization::bench::bench_randomize_120k_50p                   5,208,653         5,090,327                -118,326   -2.27%   x 1.02 
 flat_bvh::bench::bench_build_1200_triangles_flat_bvh                 1,328,015         680,239                  -647,776  -48.78%   x 1.95 
 flat_bvh::bench::bench_build_120k_triangles_flat_bvh                 185,600,059       96,599,752            -89,000,307  -47.95%   x 1.92 
 flat_bvh::bench::bench_build_12k_triangles_flat_bvh                  15,789,963        8,021,854              -7,768,109  -49.20%   x 1.97 
 flat_bvh::bench::bench_flatten_120k_triangles_bvh                    3,902,990         3,889,989                 -13,001   -0.33%   x 1.00 
 flat_bvh::bench::bench_intersect_1200_triangles_flat_bvh             200               184                           -16   -8.00%   x 1.09 
 flat_bvh::bench::bench_intersect_120k_triangles_flat_bvh             1,130             1,108                         -22   -1.95%   x 1.02 
 flat_bvh::bench::bench_intersect_12k_triangles_flat_bvh              441               423                           -18   -4.08%   x 1.04 
 ray::bench::bench_intersects_aabb                                    60,966            51,723                     -9,243  -15.16%   x 1.18 
 ray::bench::bench_intersects_aabb_branchless                         60,964            51,796                     -9,168  -15.04%   x 1.18 
 ray::bench::bench_intersects_aabb_naive                              60,973            51,773                     -9,200  -15.09%   x 1.18 
 testbase::bench_intersect_120k_triangles_list                        905,052           1,933,707               1,028,655  113.66%   x 0.47 
 testbase::bench_intersect_120k_triangles_list_aabb                   386,645           516,144                   129,499   33.49%   x 0.75 
 testbase::bench_intersect_sponza_list                                635,951           1,092,405                 456,454   71.78%   x 0.58 
 testbase::bench_intersect_sponza_list_aabb                           199,789           268,844                    69,055   34.56%   x 0.74 
```


## About

This crate can be used for applications which contain intersection computations of rays
with primitives. For this purpose a binary tree BVH (Bounding Volume Hierarchy) is of great
use if the scene which the ray traverses contains a huge number of primitives. With a BVH the
intersection test complexity is reduced from O(n) to O(log2(n)) at the cost of building
the BVH once in advance. This technique is especially useful in ray/path tracers. For
use in a shader this module also exports a flattening procedure, which allows for
iterative traversal of the BVH.
This library is built on top of [ultraviolet](https://docs.rs/crate/ultraviolet/0.4.5), but currently does not do any SIMD.

## Example

```rust
use bvh_ultraviolet::aabb::{AABB, Bounded};
use bvh_ultraviolet::bvh::BVH;
use bvh_ultraviolet::ultraviolet::Vec3;
use bvh_ultraviolet::ray::Ray;

let origin = Vec3::new(0.0,0.0,0.0);
let direction = Vec3::new(1.0,0.0,0.0);
let ray = Ray::new(origin, direction);

struct Sphere {
    position: Vec3,
    radius: f32,
}

impl Bounded for Sphere {
    fn aabb(&self) -> AABB {
        let half_size = Vec3::new(self.radius, self.radius, self.radius);
        let min = self.position - half_size;
        let max = self.position + half_size;
        AABB::with_bounds(min, max)
    }
}

let mut spheres = Vec::new();
for i in 0..1000u32 {
    let position = Vec3::new(i as f32, i as f32, i as f32);
    let radius = (i % 10) as f32 + 1.0;
    spheres.push(Sphere {
        position: position,
        radius: radius,
    });
}

let bvh = BVH::build(&spheres);
let hit_sphere_aabbs = bvh.traverse_recursive(&ray, &spheres);
```

## Optimization

This crate provides BVH updating, which is also called optimization. With BVH optimization
you can mutate the shapes on which the BVH is built and update the tree accordingly without rebuilding it completely.
This method is very useful when there are only very few changes to a huge scene. When the major part of the scene is static,
it is faster to update the tree, instead of rebuilding it from scratch.

### Drawbacks

First of all, optimizing is not helpful if more than half of the scene is not static.
This is due to how optimizing takes place:
Given a set of indices of all shapes which have changed, the optimize procedure tries to rotate fixed constellations
in search for a better surface area heuristic (SAH) value. This is done recursively from bottom to top while fixing the AABBs
in the inner nodes of the BVH. Which is why it is inefficient to update the BVH in comparison to rebuilding, when a lot
of shapes have moved.

Another problem with updated BVHs is, that the resulting BVH is not optimal. Assume that the scene is composed of two major
groups separated by a large gap. When one shape moves from one group to another, the updating procedure will not be able to
find a sequence of bottom-up rotations which inserts the shape deeply into the other branch.

The following benchmarks are run with two different datasets:
* A randomly generated scene with unit sized cubes containing a total of (1200, 12000, and 120000 triangles).
* Sponza, a popular scene for benchmarking graphics engines.


### Intersection via traversal of the list of triangles

```rust
// TODO redo all benchmarks comparing nalgebra version and ultraviolet version.
// These are currently the original author's benchmarks using nalgebra.
test testbase::bench_intersect_120k_triangles_list                       ... bench:   1,018,566 ns/iter (+/- 91,405)
test testbase::bench_intersect_sponza_list                               ... bench:     669,474 ns/iter (+/- 18,928)
```

This is the most naive approach to intersecting a scene with a ray. It defines the baseline.

### Intersection via traversal of the list of triangles with AABB checks

```rust
// TODO redo all benchmarks comparing nalgebra version and ultraviolet version.
// These are currently the original author's benchmarks using nalgebra.
test testbase::bench_intersect_120k_triangles_list_aabb                  ... bench:     400,877 ns/iter (+/- 23,775)
test testbase::bench_intersect_sponza_list_aabb                          ... bench:     206,014 ns/iter (+/- 4,508)
```

AABB checks are cheap, compared to triangle-intersection algorithms. Therefore, preceeding AABB checks
increase intersection speed by filtering negative results a lot faster.

### Build of a BVH from scratch

```rust
// TODO redo all benchmarks comparing nalgebra version and ultraviolet version.
// These are currently the original author's benchmarks using nalgebra.
test bvh::bvh::tests::bench_build_1200_triangles_bvh                     ... bench:   1,300,824 ns/iter (+/- 32,262)
test bvh::bvh::tests::bench_build_12k_triangles_bvh                      ... bench:  15,327,304 ns/iter (+/- 360,985)
test bvh::bvh::tests::bench_build_120k_triangles_bvh                     ... bench: 181,138,173 ns/iter (+/- 5,296,719)

test bvh::bvh::tests::bench_build_sponza_bvh                             ... bench: 120,335,877 ns/iter (+/- 3,787,414)
```

As you can see, building a BVH takes a long time. Building a BVH is only useful if the number of intersections performed on the
scene exceeds the build duration. This is the case in applications such as ray and path tracing, where the minimum
number of intersections is `1280 * 720` for an HD image.

### Intersection via BVH traversal

```rust
// TODO redo all benchmarks comparing nalgebra version and ultraviolet version.
// These are currently the original author's benchmarks using nalgebra.
test bvh::bvh::tests::bench_intersect_1200_triangles_bvh                 ... bench:         202 ns/iter (+/- 3)
test bvh::bvh::tests::bench_intersect_120k_triangles_bvh                 ... bench:         959 ns/iter (+/- 26)
test bvh::bvh::tests::bench_intersect_12k_triangles_bvh                  ... bench:         461 ns/iter (+/- 14)
test bvh::bvh::tests::bench_intersect_sponza_bvh                         ... bench:       1,784 ns/iter (+/- 202)
```

These performance measurements show that traversing a BVH is much faster than traversing a list.

### Optimization

The benchmarks for how long it takes to update the scene also contain a randomization process which takes some time.

```rust
// TODO redo all benchmarks comparing nalgebra version and ultraviolet version.
// These are currently the original author's benchmarks using nalgebra.
test bvh::optimization::tests::bench_randomize_120k_50p                  ... bench:  14,248,069 ns/iter (+/- 2,368,251)

test bvh::optimization::tests::bench_optimize_bvh_120k_00p               ... bench:   2,338,563 ns/iter (+/- 59,248)
test bvh::optimization::tests::bench_optimize_bvh_120k_01p               ... bench:  12,690,322 ns/iter (+/- 5,235,405)
test bvh::optimization::tests::bench_optimize_bvh_120k_10p               ... bench: 117,318,325 ns/iter (+/- 34,879,930)
test bvh::optimization::tests::bench_optimize_bvh_120k_50p               ... bench: 502,788,600 ns/iter (+/- 161,281,887)

// TODO Sponza benchmarks.
```

This is the place where you have to differentiate between rebuilding the tree from scratch or trying to optimize the old one.
These tests show the impact of moving around a particular percentage of shapes (`10p` => `10%`).
It is important to note that the randomization process here moves triangles around indiscriminately.
This will also lead to cases where the BVH would have to be restructured completely.

### Intersection after the optimization

These intersection tests are grouped by dataset and by the BVH generation method.
* `_after_optimize` uses a BVH which was kept up to date with calls to `optimize`, while
* `_with_rebuild` uses the same triangle data as `_after_optimize`, but constructs a BVH from scratch.

*120K Triangles*
```rust
// TODO redo all benchmarks comparing nalgebra version and ultraviolet version.
// These are currently the original author's benchmarks using nalgebra.
test bvh::optimization::tests::bench_intersect_120k_after_optimize_00p   ... bench:         968 ns/iter (+/- 31)
test bvh::optimization::tests::bench_intersect_120k_after_optimize_01p   ... bench:     147,160 ns/iter (+/- 12,886)
test bvh::optimization::tests::bench_intersect_120k_after_optimize_10p   ... bench:   1,624,675 ns/iter (+/- 758,933)
test bvh::optimization::tests::bench_intersect_120k_after_optimize_50p   ... bench:   2,775,067 ns/iter (+/- 751,818)

test bvh::optimization::tests::bench_intersect_120k_with_rebuild_00p     ... bench:         964 ns/iter (+/- 38)
test bvh::optimization::tests::bench_intersect_120k_with_rebuild_01p     ... bench:       1,016 ns/iter (+/- 16)
test bvh::optimization::tests::bench_intersect_120k_with_rebuild_10p     ... bench:       2,025 ns/iter (+/- 213)
test bvh::optimization::tests::bench_intersect_120k_with_rebuild_50p     ... bench:       2,373 ns/iter (+/- 251)
```

*Sponza*
```rust
// TODO redo all benchmarks comparing nalgebra version and ultraviolet version.
// These are currently the original author's benchmarks using nalgebra.
test bvh::optimization::tests::bench_intersect_sponza_after_optimize_00p ... bench:       1,824 ns/iter (+/- 114)
test bvh::optimization::tests::bench_intersect_sponza_after_optimize_01p ... bench:       3,791 ns/iter (+/- 308)
test bvh::optimization::tests::bench_intersect_sponza_after_optimize_10p ... bench:       4,794 ns/iter (+/- 212)
test bvh::optimization::tests::bench_intersect_sponza_after_optimize_50p ... bench:       7,492 ns/iter (+/- 807)

test bvh::optimization::tests::bench_intersect_sponza_with_rebuild_00p   ... bench:       1,823 ns/iter (+/- 145)
test bvh::optimization::tests::bench_intersect_sponza_with_rebuild_01p   ... bench:       1,957 ns/iter (+/- 114)
test bvh::optimization::tests::bench_intersect_sponza_with_rebuild_10p   ... bench:       2,414 ns/iter (+/- 209)
test bvh::optimization::tests::bench_intersect_sponza_with_rebuild_50p   ... bench:       3,135 ns/iter (+/- 322)
```

This set of tests shows the impact of randomly moving triangles around and producing degenerated trees.
The *120K Triangles* dataset has been updated randomly. The *Sponza* scene was updated using a method
which has a maximum offset distance for shapes. This simulates a more realistic scenario.

We also see that the *Sponza* scene by itself contains some structures which can be tightly wrapped in a BVH.
By mowing those structures around we destroy the locality of the triangle groups which leads to more branches in the
BVH requiring a check, thus leading to a higher intersection duration.

### Running the benchmark suite
The benchmark suite uses features from the [test crate](https://doc.rust-lang.org/unstable-book/library-features/test.html) and therefore cannot be run on stable rust.
Using a nightly toolchain, run with `cargo bench --features bench` to benchmark them.
Note: If you run stable toolchain by default, you can install a nightly toolchain using rustup and run the program with `cargo +nightly bench --features bench` to run the benchmarks.
