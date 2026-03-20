```rms
avoid_cliff_zone DISTANCE
```

---

Keeps objects the specified number of tiles away from cliffs.

- Because of the size of cliff objects, a distance of at least 2 is needed to create a visible gap between cliffs and the object.
- If objects use [set_tight_grouping](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ksq6iglmnili), distance applies only to the group's center tile.
- If the objects use [set_loose_gropuing](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.umboa0q57v9y), distance applies to individual group members.
- Defaults to 1 if the argument is omitted.

**Arguments**:
- `DISTANCE`: A number (default: no avoidance).
  - If the attribute is not specified, there is no avoidance.
  - If the attribute is specified without an argument, uses a default distance of 1.

**Example**: Fill the map with Stone that stays 3 tiles away from cliffs.
```rms
<CLIFF_GENERATION>

<OBJECTS_GENERATION>
create_object STONE {
  number_of_groups 99999
  avoid_cliff_zone 4
}
```

**See more**: [avoid_cliff_zone](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.t2916w9l2cff)
