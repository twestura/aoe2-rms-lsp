```rms
set_scale_by_groups
```

---

Scales [number_of_clumps](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.7u0osqxg1v3m) to the map size. The unscaled value is relative to a 100x100 map. See the [map sizes reference](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.qannz915qgy5) for scaling ratios.

- In [create_elevation](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.h85o0pyielaj): does not increase the total tile count.
- In [create_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.acnibljecbfg): also scales the total tile count to map size when used with [number_of_tiles](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.qdz0o9mtb2hi).
- **Bug (AoC/UP/HD)**: the behavior of `set_scale_by_groups` and [set_scale_by_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.8vbd2ko0sw7f) is inverted in [create_elevation](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.h85o0pyielaj), each scales as the other should. Fixed in DE.
- If both `set_scale_by_groups` and [set_scale_by_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.8vbd2ko0sw7f) are specified, only the final one applies.
- To scale by both groups and size, use [set_scale_by_groups](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.cl6w98j0bs9h) in [create_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.acnibljecbfg), or use [conditionals](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.vs551a7tyxet) manually for [create_elevation](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.h85o0pyielaj).

**Mutually exclusive with**: [set_scale_by_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.8vbd2ko0sw7f)

**Example**: Create hills with a count that scales to map size, keeping total tiles fixed.
```rms
<ELEVATION_GENERATION>
create_elevation 4 {
  base_terrain GRASS
  number_of_tiles 400
  number_of_clumps 4
  set_scale_by_groups
}
```

**Example 2**: Create 400 tiles worth of lakes, with the number of lakes AND the total number of tiles scaling to map size. On a small map this will be 4 \* 2.1 = 8 clumps with a total of 400 \* 2.1 = 840 tiles.
```rms
<TERRAIN_GENERATION>
create_terrain WATER {
  base_terrain GRASS
  number_of_clumps 4
  number_of_tiles 400
  set_scale_by_groups
}
```

**See more**: [set_scale_by_groups (create_elevation)](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.3vkc0lxd4r4a), [set_scale_by_groups (create_terrain)](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.cl6w98j0bs9h)
