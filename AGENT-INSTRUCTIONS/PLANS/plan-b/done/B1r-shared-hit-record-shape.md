# B1r - Shared hit record shape (frozen contract)

## Guarantees later phases build on

- `HitRecord` in `src/hittable.rs` carries `t`, `point`, `normal`, `front_face`, `u` and `v`, and
  keeps the `Clone, Copy, Debug` derives on this branch.
- `normal` always faces against the ray: it is the outward normal on a front-face hit and its
  negation on a back-face hit. `front_face` is true when the ray hit the outside of the surface and
  false when it left, so the flag alone tells entering from leaving.
- `HitRecord::new(ray, t, point, outward_normal, u, v)` is the only way this branch builds a record.
  It sets `front_face` from the sign of `ray.direction.dot(outward_normal)` and flips the normal
  against the ray. Every shape passes the outward normal and its texture coordinates, so every hit
  records the face side and (u, v) the same way.
- `Sphere::hit` computes (u, v) from the outward normal, not from the ray-facing normal, so a
  front-face and a back-face hit of the same surface point get the same texture coordinates.
  Spheres map latitude and longitude with `theta = (-outward_normal.y).acos()` and
  `phi = (-outward_normal.z).atan2(outward_normal.x) + PI`, giving `u = phi / (2.0 * PI)` and
  `v = theta / PI` without clamping or wrapping.
- This branch has no material field and no `Rc`. The constructor takes six arguments here, and the
  release merge adds the hit object's material as the final argument, drops `Copy` and `Debug` and
  keeps this face-side and (u, v) behaviour (shared design
  `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B1.md`).
- `HittableList::hit` copies whole `HitRecord` values and needs no change as the record grows.
- `ray_colour` in `src/main.rs` reads `hit.normal` only. The scene's hits are all front faces, so the
  rendered image is byte-identical to the base commit.

## Documentation state

- `docs/requirements.md` items 7 and 8 state the face side and (u, v) behaviour. No page changed and
  no page refers to a plan.

## Test surface

- `src/sphere.rs` proves the near-surface front-face hit, the back-face hit from inside the sphere,
  the latitude and longitude texture coordinates of the near surface, and a miss beside the sphere.
- `src/main.rs` keeps its two render tests; neither file's test changed for this phase beyond the
  front-face assertion added to the near-surface test.
