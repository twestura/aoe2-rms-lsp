```rms
number_of_clumps NUMBER
```

---

Number of individual clumps to create.

- In [create_elevation](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.h85o0pyielaj): number of individual hills to create. Adjacent hills may merge. The total [number_of_tiles](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.efs6lqjf3k0x) is divided equally among clumps.
- In [create_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.acnibljecbfg): number of individual terrain patches to create. Adjacent clumps may merge. The total [number_of_tiles](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.qdz0o9mtb2hi) or [land_percent](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.tzpfzbf2ze3w) is divided equally among clumps. A maximum of 9320 should be used when also specifying [set_scale_by_groups](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.cl6w98j0bs9h).

**Arguments**:
- `NUMBER`: A number (default 1).

**Example**: Create 4 hills of 100 tiles each.
```rms
<ELEVATION_GENERATION>
create_elevation 3 {
  base_terrain GRASS
  number_of_tiles 400
  number_of_clumps 4
}
```

**Example 2**: Create 40 clumps on grass terrain.
```rms
<TERRAIN_GENERATION>
create_terrain FOREST {
  base_terrain GRASS
  land_percent 20
  number_of_clumps 40
}
```

**See more**: [number_of_clumps (create_elevation)](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.7u0osqxg1v3m), [number_of_clumps (create_terrain)](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.1tzwe1brcw80)
