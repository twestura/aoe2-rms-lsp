```rms
min_connected_tiles NUMBER
```

---

Prevents grouped objects from being placed in an area with fewer connected tiles than the specified number.
Intended to keep objects off tiny islands and out of tiny forest clearings.

**Bug**: Objects will be extremely biased towards being placed in the top left of the map, making this attribute unreliable for its intended purpose. Use [max_distance_to_other_zones](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.qf90qwpxyzrs) or [avoid_forest_zone](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ym7v1j9vbnle) instead.

**Arguments**:
- `NUMBER`: A number (default 0).

**See more**: [min_connected_tiles](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.6rc3lgpd171k)
