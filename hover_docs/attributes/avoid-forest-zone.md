```rms
avoid_forest_zone DISTANCE
```

---

Keeps objects the specified number of tiles away from any trees (including straggler trees and scenario editor trees).

- Used to keep resources away from forests.
- Note that the forest trees themselves are being avoided, so for sparse forests (especially baobab) larger distances may be necessary.
- If objects use [set_tight_grouping](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ksq6iglmnili), distance applies only to the group's center tile.
- If the objects use [set_loose_gropuing](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.umboa0q57v9y), distance applies to individual group members.
- Defaults to 1 if the argument is omitted.

**Mutually exclusive with**: [place_on_forest_zone](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.38vodsu87lbp)

**Arguments**:
- `DISTANCE`: A number (default: no avoidance).
  - If the attribute is not specified, there is no avoidance.
  - If the attribute is specified without an argument, uses a default distance of 1.

**Example**: Fill the map with Gold, except near trees.
```rms
<OBJECTS_GENERATION>
create_object GOLD {
  number_of_groups 99999
  avoid_forest_zone 3
}
```

**See more**: [avoid_forest_zone](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ym7v1j9vbnle)
