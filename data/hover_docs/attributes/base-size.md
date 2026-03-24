```rms
base_size RADIUS
```

---

Square radius of the initially placed land origin, before any growth.

- The default of 3 results in a 7x7 land origin (49 tiles total).
- Produces a perfect square when used with [land_percent](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ipg3ruf70nn4) 0 or [number_of_tiles](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.bzvr6x5i10na) 0.
- Can be turned into a circle with [set_circular_base](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.xl67z7pma1vl).
- `base_size` is the minimum distance a land is placed from the edge of the map.
- Land bases are placed sequentially. If they are large and overlap, the land placed last will be visible in the overlapping region.
- Non-player land bases do not overlap with each other, unless `base_size` is too large, in which case the land fails to find a valid position and is placed at the center of the map.

**Arguments**:
- `RADIUS`: A number (default 3).

**Example**: Create a 13x13 square of ice.
```rms
<LAND_GENERATION>
create_land {
  terrain_type ICE
  base_size 6
  number_of_tiles 0
}
```

**See more**: [base_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.b9qafcmyygh)
