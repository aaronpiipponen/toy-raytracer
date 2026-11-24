# Materials

A material decides how a ray that hits a surface continues, returning an attenuation colour and a scattered ray or nothing when the ray is absorbed.

## The material contract

Every object in the scene carries a material. The render loop asks that material to scatter the ray at the hit point, and passes the incoming ray, the [hit record](/docs/geometry.md#the-hit-record) and the random generator.

A scatter returns one of two results.

| Result | Effect on the path |
|---|---|
| An attenuation colour and a scattered ray | The path colour is multiplied by the attenuation and continues along the scattered ray. |
| Nothing | The ray is absorbed and the path contributes `(0, 0, 0)`. |

The attenuation is the share of the colour that the surface passes on. An absorbed ray is black whatever colour the path had collected before the hit.

The three materials differ in which results they can return.

| Material | Attenuation | Absorbs |
|---|---|---|
| Diffuse | Its albedo | Never |
| Metal | Its albedo | When the scattered direction points into the surface |
| Glass | White, `(1, 1, 1)` | Never |

The [rendering page](/docs/rendering.md) describes the path colour loop, the bounce limit and the sky colour that ends an unabsorbed path.

## Diffuse material

The diffuse material holds an albedo colour and returns that albedo as the attenuation, with the value unmodified.

The scattered direction is the hit normal plus a random unit vector.

```text
direction = normal + random_unit_vector()
```

The sum of two unit vectors has a length between 0 and 2, and the result is not normalized. The material never tests the direction and always returns a scattered ray.

### The random unit vector

The point is drawn in the cube from -1 to 1 on each axis, with a separate random value per coordinate.

```text
p = (2 * rng.next_f64() - 1, 2 * rng.next_f64() - 1, 2 * rng.next_f64() - 1)
```

`next_f64` returns a value in 0 to 1. The point is kept when `1e-12 < p·p <= 1`, and a kept point is returned as `p / |p|`. Any other point is discarded and the draw repeats. The lower bound rejects the centre of the cube, where the length is zero and the division has no result.

## Metal material

The metal material holds an albedo colour and a fuzz factor in 0 to 1. The attenuation is the albedo, with the value unmodified.

The incoming ray direction is normalized, then mirrored about the normal.

```text
reflected = v - normal * (2 * v·normal)
```

Here `v` is the unit incoming direction and `normal` is the normal that faces against the ray. The fuzz factor scales a random unit vector, and the scattered direction adds it to the reflection.

```text
scattered = reflected + random_unit_vector() * fuzz
```

The sum is not normalized. When the dot product of the scattered direction and the normal is zero or negative, the direction points into the surface and the material returns nothing. A fuzz factor of 0 gives a mirror reflection, and the absorption test then passes for every ray that arrives from outside the surface.

## Glass material

The glass material holds a refractive index. Its attenuation is white, `(1, 1, 1)`, on every scatter, and it never absorbs.

The index ratio depends on the face side. A ray that hits the front face enters the material, and the ratio is `1 / refractive_index`. A ray that hits the back face leaves the material, and the ratio is `refractive_index`. The entering case assumes an index of 1 on the side the ray arrives from, and the material stores one index.

With `v` the unit incoming direction and `normal` the normal that faces against the ray, the angle terms are

```text
cos_theta = min(-v·normal, 1)
sin_theta = sqrt(1 - cos_theta * cos_theta)
```

The material reflects when `ratio * sin_theta > 1`, which is total internal reflection. Otherwise it draws a random value and reflects when that value is below Schlick's approximation of the reflectance at `cos_theta` and the ratio.

```text
r0 = ((1 - ratio) / (1 + ratio))^2
reflectance = r0 + (1 - r0) * (1 - cos_theta)^5
```

Reflection is the mirror image of `v` about the normal.

```text
reflected = v - normal * (2 * v·normal)
```

The random value is drawn only when the total internal reflection test fails, because the two tests are joined by short-circuit evaluation.

Refraction follows Snell's law.

```text
perpendicular = (v + normal * cos_theta) * ratio
parallel = -normal * sqrt(max(1 - perpendicular·perpendicular, 0))
refracted = perpendicular + parallel
```

The formula maps the unit direction `(0.6, 0, -0.8)` and the ratio `1 / 1.5` to `(0.4, 0, -0.9165)`, rounded to four decimal places.

## Materials in the fixed scene

The fixed scene uses all three materials.

| Object | Material | Values |
|---|---|---|
| Central sphere | Diffuse | Albedo `(0.7, 0.3, 0.3)` |
| Large sphere | Diffuse | Albedo `(0.8, 0.8, 0.8)` |
| Small sphere | Metal | Albedo `(0.8, 0.8, 0.8)`, fuzz `0.3` |
| Glass sphere | Glass | Refractive index `1.5` |
| Triangle | Diffuse | Albedo `(0.7, 0.3, 0.3)` |

The [geometry page](/docs/geometry.md) describes the shapes these materials are attached to.
