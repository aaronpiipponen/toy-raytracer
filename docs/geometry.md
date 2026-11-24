# Geometry

A ray is tested against spheres, an infinite plane and triangles, and the nearest hit supplies the normal, the face side and the texture coordinates.

## The hit record

Every shape reports a hit as a record with the same fields.

| Field | Meaning |
|---|---|
| `t` | The ray parameter of the hit |
| `point` | The hit point, `origin + direction * t` |
| `normal` | A unit normal at the hit point that points against the ray |
| `front_face` | True when the ray meets the outside of the surface |
| `u`, `v` | Texture coordinates in 0 to 1 |
| `material` | The material of the object that was hit |

The hit point follows from the ray equation. A ray starts at its origin and reaches `origin + direction * t` at the parameter `t`, and `t` is the distance along the ray when the direction has unit length.

The record carries one normal and a flag that names the side. Each shape supplies its own outward normal, and one constructor builds the record from it. When the ray direction opposes the outward normal, the hit is on the outside, the flag is true and the stored normal is the outward normal. When the ray direction does not oppose the outward normal, the flag is false and the stored normal is the reversed outward normal. The stored normal therefore always points against the ray.

A hit is reported only when its `t` is greater than `t_min` and less than `t_max`. Both bounds are exclusive.

## Testing an object

### Bounding boxes

An axis-aligned bounding box holds two corners, the minimum and the maximum. A ray meets the box when its span inside the box overlaps the ray's `t` range on all three axes. A ray whose direction has a zero component on an axis and an origin outside the box's range on that axis misses the box.

The box test rejects an object the ray cannot reach, so the shape test runs only when the hit is possible.

### The scene list

The scene list holds every object and returns the nearest hit. It visits the objects in order and takes four steps for each one.

1. Reads the object's bounding box.
2. Skips the object when the ray misses that box.
3. Tests the object with `t_min` and an upper bound, which is the nearest `t` found so far or `t_max` when no hit has been found.
4. Stores the reported hit as the nearest hit.

An object tested after a hit therefore has to be nearer than every hit before it. An infinite plane reports no bounding box, so the list tests it against every ray.

## Shapes

| Shape | Defined by | Texture coordinates | Bounding box |
|---|---|---|---|
| Sphere | A centre and a radius | Longitude and latitude of the outward normal | The centre plus and minus the radius on each axis |
| Infinite plane | A point and a unit normal | The hit point along the two axes not aligned with the normal, wrapped into 0 to 1 | None, so every ray is tested against it |
| Triangle | Three vertices in counter-clockwise order seen from outside | The barycentric coordinates of the hit | The box through the three vertices |

### Spheres

A sphere is defined by a centre and a radius. Its intersection solves the quadratic for the ray parameter. The smaller root is the entry point on the near side, and the larger root is the exit point. The sphere takes the smaller root when it lies inside the ray's range and the larger root otherwise. When neither root lies inside the range, or when the discriminant is negative, the sphere is missed.

The outward normal is the vector from the centre to the hit point divided by the radius. The texture coordinates come from that normal. The longitude divided by `2π` gives `u`, and the latitude divided by `π` gives `v`. A ray from `(0, 0, 0)` along `(0, 0, -1)` through a sphere of radius 0.5 centred at `(0, 0, -2)` hits at `t = 1.5`, with the normal `(0, 0, 1)`, `u = 0.25` and `v = 0.5`.

The bounding box holds the whole sphere. It spans the centre minus the radius and the centre plus the radius on each axis.

### The infinite plane

An infinite plane is defined by a point on the plane and a normal, which is scaled to unit length when the plane is built. A ray parallel to the plane is missed, and the test treats a dot product between the normal and the ray direction below `1e-9` in absolute value as parallel.

The texture coordinates come from the hit point. The plane chooses the two axes that the largest absolute component of the normal does not use, then wraps each value into 0 to 1. A plane through `(0, -0.5, 0)` with the normal `(0, 1, 0)` hit at `(1.25, -0.5, -0.25)` gives `u = 0.25` and `v = 0.75`.

An infinite plane reports no bounding box, so the scene list tests it against every ray.

### Triangles

A triangle is defined by three vertices in counter-clockwise order seen from outside. The intersection computes the barycentric coordinates first and rejects a hit outside the triangle. A ray parallel to the triangle's plane is missed, and the test treats a determinant below `1e-8` in absolute value as parallel.

The outward normal is the cross product of the two edges from the first vertex, scaled to unit length. The vertices' order fixes the side that the outward normal points to. A hit from either side is reported, and the face flag records which side the ray met.

The texture coordinates are the barycentric coordinates of the hit. A ray through the point `(0.25, 0.25)` of the triangle with vertices `(0, 0, -2)`, `(1, 0, -2)` and `(0, 1, -2)` hits at `t = 1` with `u = 0.25` and `v = 0.25`.

The bounding box spans the minimum and the maximum of each coordinate across the three vertices.

## The fixed scene

The render tests one scene that holds five objects.

| Object | Definition | Material |
|---|---|---|
| Red sphere | Centre `(0, 0, -1)`, radius 0.5 | Diffuse, colour `(0.7, 0.3, 0.3)` |
| Ground plane | Through `(0, -0.5, 0)`, normal `(0, 1, 0)` | Diffuse, colour `(0.8, 0.8, 0.8)` |
| Back triangle | Vertices `(-2, -0.5, -2)`, `(2, -0.5, -2)` and `(0, 1.5, -2)` | Diffuse, colour `(0.35, 0.5, 0.25)` |
| Metal sphere | Centre `(0.85, -0.25, -1.3)`, radius 0.25 | Metal, colour `(0.8, 0.8, 0.8)`, fuzz factor 0.3 |
| Glass sphere | Centre `(-0.85, -0.1, -1.3)`, radius 0.4 | Glass, refractive index 1.5 |

The hit record's material field selects how the ray continues. [Materials](/docs/materials.md) states how a material scatters a ray, and [rendering](/docs/rendering.md) states how the render turns a hit into a pixel colour.
