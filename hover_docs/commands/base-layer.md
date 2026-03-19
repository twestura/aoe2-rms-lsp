```rms
base_layer TERRAIN
```

---

Specifies a terrain to layer visually on top of [base_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.t4mfjnozvxwf).
The layered terrain is visual only and does not affect terrain properties or object placement restrictions.
DE only.

- Must be used after [base_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.t4mfjnozvxwf).
- If used, the same `base_layer` must also be specified in [create_elevation](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.h85o0pyielaj) to generate elevation on the base terrain.

**Arguments**:
- `TERRAIN`: A [terrain constant](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.t4mfjnozvxwf).

**Example**: Fill the map with `DIRT`, layered with `SNOW.
```rms
<LAND_GENERATION>
base_terrain DIRT
base_layer SNOW
```

**See more**: [base_layer](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.p6oqwj1l7flh)
