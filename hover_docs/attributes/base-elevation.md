```rms
base_elevation HEIGHT
```

---

Elevates the entire land to the specified height.
Requires an [<ELEVATION_GENERATION>](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.szmawgkuqtsf) section, which can be empty.

- In HD/DE, does not work for lands with a water [terrain_type](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.lzceesmva36o).
- Up to a height of 9 the surrounding terrains contain the slope. With even higher values, the remaining elevation occurs within the confines of the land.
- In UP/HD, elevations higher than 7 should not be used, as objects fail to render properly.
- In DE, elevations higher than 7 can be used, but may cause terrain rendering issues for certain screen resolutions, especially above approximately 16.
- Negative values maximally elevate a land (not recommended due to rendering issues).

**Arguments**:
- `HEIGHT`: A number 1&ndash;16 (default 0, not elevated).

**Example**: Create a palm desert land with elevation 2.
```rms
<LAND_GENERATION>
create_land {
  terrain_type PALM_DESERT
  number_of_tiles 128
  base_elevation 2
}
<ELEVATION_GENERATION>
```

**See more**: [base_elevation](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ppqecxdcopxb)
