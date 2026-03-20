```rms
base_terrain TERRAIN
```

---

Specifies a base terrain.

- In [<LAND_GENERATION>](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.15g4dj26anlp): fills the entire map with the specified terrain before land generation.
- In [create_elevation](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.h85o0pyielaj): specifies the terrain on which hills generate. Only considers terrains from [<LAND_GENERATION>](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.15g4dj26anlp).
- In [create_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.acnibljecbfg): specifies the existing terrain on which the new terrain is painted.

**Arguments**:
- `TERRAIN`: A [terrain constant](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.3bdjnf7tryyk) (default `GRASS`).

**Example**: Fill the map with water.
```rms
<LAND_GENERATION>
base_terrain WATER
```

**Example 2**: Create one hill on water terrain.
```rms
<ELEVATION_GENERATION>
create_elevation 7 {
  base_terrain WATER
  number_of_tiles 600
}
```

**Example 3**: Create a large clump of forest terrain on grass terrain, then create water on the forest.
```rms
<TERRAIN_GENERATION>
create_terrain FOREST {
  base_terrain GRASS
  land_percent 10
}
create_terrain WATER {
  base_terrain FOREST
}
```

**See more**: [base_terrain (<LAND_GENERATION>)](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.t4mfjnozvxwf), [base_terrain (create_elevation)](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.y6zq54jntrkn), [base_terrain (create_terrain)](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ptxp1ht2fh0p)
