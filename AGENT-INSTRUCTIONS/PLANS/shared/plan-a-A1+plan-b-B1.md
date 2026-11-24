# Shared design: plan-a-A1 + plan-b-B1 (`HitRecord`)

Settled between the plan-a and plan-b orchestrators. Both plans reach this design on their own
branches; commits never move between branches, no branch is merged into another, and the release
merges both into the integration branch and resolves the textual conflicts in `src/hittable.rs` and
`src/sphere.rs` by the shape below.

## Target merged record

```rust
pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,      // always faces against the ray
    pub front_face: bool,  // true when the ray hit the outside
    pub u: f64,
    pub v: f64,
    pub material: Rc<dyn Material>,
}
```

Derives `Clone` only. `Copy` is gone with the `Rc` field, and `Debug` is dropped because
`Rc<dyn Material>` is not `Debug` and nothing prints a hit record. The `Material` trait gains no
`Debug` supertrait.

`normal` always faces against the ray (`docs/requirements.md` item 7). `front_face` is true when the
ray hit the outside of the surface; that flag, not the normal's direction, tells entering from
leaving.

## One construction path

```rust
HitRecord::new(ray, t, point, outward_normal, u, v, material)
```

- `front_face = ray.direction.dot(outward_normal) < 0.0`;
- `normal = if front_face { outward_normal } else { -outward_normal }`;
- records every field.

No record literal and no second face-side helper survives the merge on either branch. That rule
covers A1's `src/material.rs` tests too: the merge routes their records through
`HitRecord::new(...)` with the outward normal and `u`, `v`.

## Merged `Sphere::hit` tail

```rust
let point = ray.at(t);
let outward_normal = (point - self.center) / self.radius;
let theta = (-outward_normal.y).acos();
let phi = (-outward_normal.z).atan2(outward_normal.x) + PI;
Some(HitRecord::new(
    ray,
    t,
    point,
    outward_normal,
    phi / (2.0 * PI),
    theta / PI,
    Rc::clone(&self.material),
))
```

## What each plan does on its own branch

- **plan-b / B1:** the record gains `front_face`, `u`, `v` and keeps its current derives; the
  constructor takes six arguments, without a material; `Sphere::hit` fills `u` and `v` from the
  outward normal's latitude and longitude, so a back-face hit of the same point gets the same texture
  coordinates. No material exists on this branch. Later plan-b shapes (B2/B3) use the same
  constructor without a material and take the object's material as the final argument once the
  material seam is merged in.
- **plan-a / A1:** unchanged as detailed. The record keeps its material field and literal
  construction on this branch. The merge combines the two: the record gains `front_face`, `u`, `v`
  and loses `Copy` and `Debug`, and `Sphere::hit` becomes the merged tail above.

## What A2 may rely on after the merge

- `hit.normal` always faces the ray; `hit.front_face` is true on entry, false when leaving.
- Glass decides entering versus leaving from `hit.front_face` and works with `hit.normal` in the
  glass frame: `cos_theta = -ray_in.direction.unit().dot(hit.normal)`, positive on entry. No outward
  normal and no second face flag.
- A1's `Lambertian` and `Metal` formulas stay as written; both already use a normal that faces the
  incoming ray. Their comments say "the normal facing the ray" after the merge, not "outward
  normal".

## A2 on plan-a's branch before the merge

Glass derives the face side locally from the outward normal A1 leaves on the record
(`front_face = ray_in.direction.dot(hit.normal) < 0.0`, normal flipped from it), uses
`cos_theta = -ray_in.direction.unit().dot(normal)`, and **the release replaces those two lines with
`hit.front_face` and the facing `hit.normal` as part of the merge itself**, not as optional cleanup.
That local derivation is temporary branch code, never part of the merged shape.

## Materials for the new shapes at the merge

Requirement 1 says every object has a material, but plan-b's branch has no material seam when B2 and
B3 add planes and triangles. The release gives each new shape a material field and passes
`Rc::clone(&self.material)` as the constructor's last argument, and names a scene material for it.

Pending user decision: which material and colour the ground plane and the triangle get. Settled
before B2 is detailed; it does not block B1's resolution and changes nothing in A1's or B1's
resolution scope.
