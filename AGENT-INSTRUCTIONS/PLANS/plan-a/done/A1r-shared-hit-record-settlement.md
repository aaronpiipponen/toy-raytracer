# A1r - Shared hit record settlement (frozen contract)

## Guarantees later phases build on

- The shared `HitRecord` design settled with the geometry plan lives in
  `PLANS/shared/plan-a-A1+plan-b-B1.md`. This branch records it; it does not implement the geometry
  plan's half.
- The merged record has `t`, `point`, `normal` (always facing against the ray), `front_face`, `u`,
  `v` and `material: Rc<dyn Material>`. It derives `Clone` only: no `Copy`, no `Debug`, and the
  `Material` trait gains no `Debug` supertrait.
- After the merge there is one construction path:
  `HitRecord::new(ray, t, point, outward_normal, u, v, material)` sets
  `front_face = ray.direction.dot(outward_normal) < 0.0`, flips `normal` to face the ray, and records
  every field. No record literal and no second face-side helper survives, including in A1's
  `src/material.rs` tests.
- The merged `Sphere::hit` tail computes the point, the outward normal from the centre and radius,
  `theta` and `phi` latitude and longitude, then calls
  `HitRecord::new(ray, t, point, outward_normal, phi / (2.0 * PI), theta / PI, Rc::clone(&self.material))`.
- On this branch before the merge, A1 keeps the material field and literal construction and adds no
  face flag or texture coordinates; A2 derives the face side locally
  (`front_face = ray_in.direction.dot(hit.normal) < 0.0`, normal flipped to face the ray) as
  temporary branch code. The merge replaces that derivation with `hit.front_face` and the facing
  `hit.normal`.
- After the merge A1 may rely on `hit.material` and on `hit.normal` facing the incoming ray; A2 may
  rely on `hit.front_face` and that normal, with
  `cos_theta = -ray_in.direction.unit().dot(hit.normal)` positive on entry.
- The release merge's obligations on plan-a's side: drop `Copy` and `Debug` from the record and add
  `front_face`, `u`, `v`; route every record literal, including A1's material-test records, through
  `HitRecord::new` with the outward normal and `u`, `v`; adopt the merged `Sphere::hit` tail; replace
  A2's local derivation with `hit.front_face` and the facing `hit.normal`; give the geometry plan's
  plane and triangle a material field (plane mid-grey, triangle dull green) and pass
  `Rc::clone(&self.material)` last; and update the record's doc comment to name the normal facing the
  ray instead of A1's branch wording "outward normal".
- A1's and A2's existing detail stands unchanged; this plan's only product code is theirs.

## Documentation state

- `full-plan.md` binding rules, scoped entries and carry-overs state the same settlement. The status
  table records A1r as done and A1 active behind it.
- This phase changed plan documents only: no product code, test, `docs/` page, `Cargo.toml` or
  configuration. The verification is the documentation-only gate: the changed plan documents were
  re-read and `git diff --check` was clean.
