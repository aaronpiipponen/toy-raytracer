# Materials

Each surface carries a material that decides how a ray scatters from it and how much attenuation the path collects.

## The material contract

A material reads the incoming ray, the hit record and the seeded random generator. It returns one of two outcomes. It returns an attenuation colour and a scattered ray, or it returns nothing, which absorbs the ray.

The hit record supplies the hit point, the surface normal, the face side and the material of the object. The normal points against the incoming ray, and the face side records whether the ray hit the front face. [Geometry](/docs/geometry.md) covers how each shape builds the record.

The bounce loop calls the material of every surface the scattered ray reaches. It starts with a white running colour, multiplies that colour by each attenuation, and follows the scattered ray. The loop allows at most 50 scatter events. A path that reaches the limit contributes black.

A material that returns nothing contributes black for the whole path, because no further ray leaves the surface.

## Diffuse

A diffuse surface scatters in the direction of the surface normal plus a random unit vector. The random vector is uniform over the unit sphere, drawn from the seeded generator by rejection sampling.

The summed direction is used as the scattered ray's direction without normalisation. The scattered ray starts at the hit point.

The attenuation is the material's albedo. A diffuse surface scatters on every hit and never absorbs the ray.

## Metal

A metal surface reflects the unit incoming direction about the surface normal at the hit point. It adds a random unit vector scaled by the fuzz factor, so a fuzz factor in `0..=1` blurs the reflection. A fuzz factor of zero leaves the mirror reflection.

The material absorbs the ray when the scattered direction does not point away from the surface, where the dot product with the normal is zero or less. Otherwise the attenuation is the material's albedo.

## Glass

A glass surface carries a refractive index. On the front face, where the ray enters the surface, the refraction ratio is the reciprocal of the index. On the back face, where the ray leaves, the ratio is the index.

The material takes the cosine of the angle between the reversed incoming direction and the normal, clamped to at most one, and derives the sine of that angle from it. Total internal reflection occurs when the ratio times the sine exceeds one, and the ray then reflects about the normal.

Below that limit, Schlick's approximation gives a reflectance from the cosine and the ratio. The material draws a random value in the range zero to one and reflects when the reflectance is greater than that value. Otherwise it refracts by Snell's law.

The attenuation is white, so the scattered ray carries the full colour of the path. A glass surface scatters on every hit and never absorbs the ray.

## The fixed scene

The scene holds five objects, and their materials use these values.

| Object | Material | Values |
|---|---|---|
| Red sphere | Diffuse | albedo `(0.7, 0.3, 0.3)` |
| Ground plane | Diffuse | albedo `(0.8, 0.8, 0.8)` |
| Back triangle | Diffuse | albedo `(0.35, 0.5, 0.25)` |
| Metal sphere | Metal | albedo `(0.8, 0.8, 0.8)`, fuzz `0.3` |
| Glass sphere | Glass | refractive index `1.5` |
