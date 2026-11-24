# Geometry

A ray can hit a sphere, an infinite plane or a triangle, and every hit records its ray parameter, point, normal, face side, texture coordinates and material.

## The hit record

A hit record carries the result of one intersection.

| Field | Contents |
|---|---|
| `t` | The ray parameter, so the hit point is `origin + direction * t`. |
| `point` | The hit point. |
| `normal` | The surface normal, turned to face against the ray. |
| `front_face` | Whether the ray met the outward side of the surface. |
| `u`, `v` | Texture coordinates in 0 to 1. |
| `material` | The material at the hit point, described on the [materials page](/docs/materials.md). |

The record is built from an outward normal, which points away from the object. The front-face test is the dot product of the ray direction and the outward normal.

```text
front_face = direction·outward_normal < 0
```

When `front_face` is true the stored normal is the outward normal, and when it is false the stored normal is the outward normal negated. The dot product of the ray direction and the stored normal is therefore zero or negative, so the normal always faces against the ray.

`t` is the distance along the ray only when the direction is a unit vector. Camera rays and diffuse scattered rays are not normalized, and the hit point is `origin + direction * t` in every case.

Each shape fills `u` and `v` by its own rule. The render loop does not read them.

## Spheres

A sphere is a centre, a radius and a material.

### Sphere intersection

With `oc` the vector from the centre to the ray origin, the quadratic terms are

```text
oc = origin - centre
a = direction·direction
half_b = oc·direction
c = oc·oc - radius * radius
discriminant = half_b * half_b - a * c
```

A discriminant below zero means the ray misses the sphere. Otherwise the nearer root is `(-half_b - sqrt(discriminant)) / a`. When that root is at or outside either end of the hit range, the farther root `(-half_b + sqrt(discriminant)) / a` is tried. When both roots are at or outside the range, the sphere is missed. Both ends of the range are exclusive, so a root equal to `t_min` or to `t_max` is rejected.

A ray that starts inside the sphere has a negative nearer root, so it takes the farther root and records a hit on the back face.

### Sphere texture coordinates

The outward normal is `(point - centre) / radius`. The texture coordinates come from the direction of that normal.

```text
theta = acos(-normal.y)
phi = atan2(-normal.z, normal.x) + PI
u = phi / (2 * PI)
v = theta / PI
```

`v` is 0 at the south pole, where `normal.y` is -1, and 1 at the north pole, where `normal.y` is 1. `u` runs around the equator and lies in 0 to 1 with 0 excluded.

### Sphere bounding box

The box runs from `centre - (radius, radius, radius)` to `centre + (radius, radius, radius)`.

## Infinite planes

A plane is a point, a normal and a material, and it has no edge. The point and the normal are stored as given.

### Plane intersection

```text
denominator = direction·normal
t = ((plane_point - origin)·normal) / denominator
```

A denominator of exactly zero means the ray runs parallel to the plane and misses. The hit is rejected when `t` is at or outside either end of the hit range, so both ends of the range are exclusive.

The outward normal is the plane's normal.

### Plane texture coordinates

The tangent lies in the plane. When the absolute value of the normal's x component exceeds that of its y component, the tangent is `(-normal.z, 0, normal.x)` normalized, and otherwise it is `(0, -normal.z, normal.y)` normalized. The bitangent is `normal × tangent`.

```text
u = (point·tangent) mod 1
v = (point·bitangent) mod 1
```

The modulo is `rem_euclid`, which returns a value in 0 to 1 with 1 excluded. The pattern repeats every unit of distance along the tangent and the bitangent.

## Triangles

A triangle is three vertices and a material. The vertices run counter-clockwise seen from the front face.

### Triangle intersection

With `a`, `b` and `c` the vertices, the two edges from `a` are `edge1 = b - a` and `edge2 = c - a`. The intersection test uses the barycentric form.

```text
h = direction × edge2
determinant = edge1·h
inverse = 1 / determinant
s = origin - a
u = inverse * (s·h)
q = s × edge1
v = inverse * (direction·q)
t = inverse * (edge2·q)
```

A determinant with an absolute value below `1e-9` is treated as parallel to the plane of the triangle, and the ray misses. The hit is rejected in four further cases, tested in this order.

| Test | Meaning |
|---|---|
| `u < 0` | The crossing lies beyond the edge from `a` to `c`. |
| `u > 1` | The crossing lies beyond the vertex `b`. |
| `v < 0` | The crossing lies beyond the edge from `a` to `b`. |
| `u + v > 1` | The crossing lies beyond the edge from `b` to `c`. |
| `t` at or outside either end of the hit range | The crossing lies outside the range. |

The outward normal is `(edge1 × edge2)` normalized, which is the front face for the counter-clockwise vertex order.

### Triangle texture coordinates

The texture coordinates are the barycentric weights `u` of `b` and `v` of `c`. The weight of `a` is `1 - u - v`.

### Triangle bounding box

On each axis the box runs from the smallest of the three vertex coordinates to the largest.

## The bounding box test

A box is two opposite corners, one low on every axis and one high on every axis. The ray test is a slab test over the three axes.

The test starts with `t_enter = t_min` and `t_exit = t_max`. For one axis with a nonzero direction component,

```text
near = (min - origin) / direction
far = (max - origin) / direction
swap near and far when near > far
t_enter = max(t_enter, near)
t_exit = min(t_exit, far)
```

A negative direction component reverses the order of the two plane crossings on that axis, and the swap puts the nearer crossing first. The ray misses when `t_enter > t_exit`. The ray hits the box when all three axes are processed without that test failing.

When the direction component on an axis is exactly zero, the ray runs parallel to that pair of box faces. The ray misses if the origin lies below the minimum or above the maximum on that axis, and the axis is otherwise skipped.

## The scene list

The scene list holds its objects in the order they are added and returns the nearest hit.

The list visits every object. For each object, `limit` is the `t` of the nearest hit so far, or `t_max` when no object has been hit. An object that reports a bounding box has its box tested first, over the range from `t_min` to `limit`, and the object is skipped when the box is missed. The object is then tested over the same range. A hit replaces the nearest hit, so `limit` shrinks to the ray parameter of the nearest hit so far.

A plane reports no bounding box and is always tested. The list returns nothing when no object is hit.

The [rendering page](/docs/rendering.md) describes the loop that casts these rays, and the [materials page](/docs/materials.md) describes what happens at each hit.
