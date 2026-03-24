```rms
min_distance_to_map_edge DISTANCE
```

---

Minimum distance in tiles that objects stay away from the edge of the map.

- If objects use [set_tight_grouping](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ksq6iglmnili), distance applies only to the group's center tile.
- If the objects use [set_loose_gropuing](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.umboa0q57v9y), distance applies to individual group members.

**Arguments**:
- `DISTANCE`: A number (default 0).

**Example**: Ensure relics stay at least 10 tiles from the edge of the map.
```rms
<OBJECTS_GENERATION>
create_object RELIC {
  set_gaia_object_only
  number_of_objects 500
  min_distance_to_map_edge 10
}
```

**See more**: [min_distance_to_map_edge](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.w2q69orjs3l3)
