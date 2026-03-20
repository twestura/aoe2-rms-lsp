```rms
spacing NUMBER
```

---

Number of tiles between each elevation level. Values greater than 1 produce rings of flat terrain on each level of a hill.

**Arguments**:
- `NUMBER`: A number 1+ (default 1, no spacing).

**Example**: Create a hill with visible flat rings between elevation levels.
```rms
<ELEVATION_GENERATION>
create_elevation 7 {
  base_terrain GRASS
  number_of_tiles 3000
  spacing 4
}
```

**See more**: [spacing](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.hqjpcx4o099o)
