```rms
beach_terrain TERRAIN
```

---

Specifies a terrain to place where the current terrain borders water.

- If a non-beach terrain is specified, players will not be able to build docks on the coastline.
- If a water terrain is specified, it fully replaces the terrain specified in [create_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.acnibljecbfg) (not recommended).
- **Bug**: `beach_terrain` does not work when a [<CONNECTION_GENERATION>](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.urxh5ze1aaoh) section is present.

**Arguments**:
- `TERRAIN`: A [terrain constant](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.3bdjnf7tryyk) (default `BEACH`).

**Example**: Create a dirt island with vegetated beaches.
```rms
<LAND_GENERATION>
base_terrain WATER

<TERRAIN_GENERATION>
create_terrain DIRT {
  number_of_tiles 500
  spacing_to_other_terrain_types 1
  base_terrain WATER
  beach_terrain DLC_BEACH2
}
```

**See more**: [beach_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.qlmjwkuqe8hc)
