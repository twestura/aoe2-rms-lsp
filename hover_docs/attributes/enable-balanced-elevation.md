```rms
enable_balanced_elevation
```

---

Reduces the southward bias of hill placement.
Disabled by default.
Elevation still is slightly biased toward the south even with this attribute.

**Example**: Spread hills evenly across the map.
```rms
<ELEVATION_GENERATION>
create_elevation 7 {
  base_terrain GRASS
  number_of_tiles 600
  number_of_clumps 4
  enable_balanced_elevation
}
```

**See more**: [enable_balanced_elevation](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.izx21xcrwjlg)
