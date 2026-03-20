```rms
require_path DEVIATION
```

---

Requires that objects have a path to the origin of their associated land. Use this attribute to prevent player resources from being trapped in or behind forests.

- Requires [set_place_for_every_player](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.hch89ipgsvb) or [place_on_specific_land_id](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.34wzlujx4lbv).
- 0 imposes no further restrictions beyond preventing a completely inaccessible location.
- 1 means the object must additionally have a mostly direct path to the origin.
- Larger values allow paths that are less direct. Maximum effective value depends on how constricted the path is.
- Walls and gates count as obstructing a path.
  - `require_path` can spawn objects outside of the walled area if used before the walls are placed.
  - `require_path` only spawns objects within the walled area after the walls are placed.
- For grouped objects, only applies to the first object of a group, meaning some members of loose groups may not have a path.

**Arguments**:
- `DEVIATION`: A number (default 0).
  - 0 allows indirect paths.
  - 1 requires mostly direct paths.
  - Larger values allow progressively less direct paths.

**Example**: Ensure a player Boar is not located behind a nearby woodline.
```rms
<OBJECTS_GENERATION>
create_object BOAR {
  set_place_for_every_player
  set_gaia_object_only
  require_path 1
  min_distance_to_players 16
  max_distance_to_players 22
}
```

**See more**: [require_path](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.woysch92a2oh)
