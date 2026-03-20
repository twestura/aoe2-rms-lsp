```rms
base_layer TERRAIN
```

---

Specifies a layered terrain.

- In [<LAND_GENERATION>](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.15g4dj26anlp): layers a terrain visually on top of [base_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.t4mfjnozvxwf). The layered terrain is visual only and does not affect terrain properties or object placement restrictions. Must be used after [base_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.t4mfjnozvxwf). If used, the same `base_layer` must also be specified in [create_elevation](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.h85o0pyielaj) to generate elevation on the base terrain.
- In [create_elevation](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.h85o0pyielaj): use this attribute in addition to [base_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.y6zq54jntrkn) if and only if a `base_layer` was specified for the map base terrain in [<LAND_GENERATION>](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.15g4dj26anlp).
- In [create_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.acnibljecbfg): specifies a layered terrain on which to place the new terrain. If used together with [base_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ptxp1ht2fh0p), the new terrain is placed only where both the base and the layer apply.

**Arguments**:
- `TERRAIN`: A [terrain constant](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.3bdjnf7tryyk) (default: no layered terrain).

**Example**: Fill the map with dirt, layered with snow. Then elevate that terrain.
```rms
<LAND_GENERATION>
base_terrain DIRT
base_layer SNOW

<ELEVATION_GENERATION>
create_elevation 7 {
  base_terrain DIRT
  base_layer SNOW
  number_of_tiles 9320
  number_of_clumps 20
}
```

**Example 2**: Layer desert on grass, then place water on the layered desert.
```rms
<TERRAIN_GENERATION>
create_terrain DESERT {
  base_terrain GRASS
  land_percent 10
  terrain_mask 1
}
create_terrain WATER {
  base_layer DESERT
}
```

**See more**: [base_layer (<LAND_GENERATION>)](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.p6oqwj1l7flh), [base_layer (create_elevation)](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.my6smlk88r2j), [base_layer (create_terrain)](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.nlwld8oca536)
