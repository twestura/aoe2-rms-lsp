```rms
force_placement
```

---

Allows multiple objects to be placed on the same tile when necessary. Normally only one object can be placed per tile, and remaining objects are not generated if tiles run out. With `force_placement` active, remaining objects are placed on the corners of tiles and then on top of each other.

- Only works for objects that can overlap on the same tile, such as units. Does not work for buildings.
- Disabled when using [set_loose_grouping](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.umboa0q57v9y).

**Example**: Place 50 sheep in the 1-tile radius surrounding a starting outpost.
```rms
<OBJECTS_GENERATION>
create_object OUTPOST {
  set_place_for_every_player
  max_distance_to_players 0
}
create_object SHEEP {
  number_of_objects 50
  set_place_for_every_player
  max_distance_to_players 1
  force_placement
}
```

**See more**: [force_placement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.138scj5wa7v7)
