```rms
min_distance_cliffs DISTANCE
```

---

Minimum distance in cliff units between separate cliffs.

- The unit is cliff units, not tiles. 0 is 0&nbsp;tiles, 1 is 3&nbsp;tiles, 2 is 6&nbsp;tiles, etc.

**Arguments**:
- `DISTANCE`: The number of cliff units (default 2).

**Example**: Fill the map with cliffs that stay only 3&nbsp;tiles from other cliffs.
```rms
<CLIFF_GENERATION>
min_number_of_cliffs 9999
max_number_of_cliffs 9999
min_distance_cliffs 1
```

**See more**: [min_distance_cliffs](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.sj0nz9h7pbsy)
