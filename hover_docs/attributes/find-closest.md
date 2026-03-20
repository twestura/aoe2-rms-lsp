```rms
find_closest
```

---

Places the object on the closest free tile to the center of the land, taking all other constraints into account.

- Requires [set_place_for_every_player](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.hch89ipgsvb) or [place_on_specific_land_id](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.34wzlujx4lbv).
- Uses circular (Euclidean) distance, while other distance constraints use square distance by default. Using both `find_closest` and [min_distance_to_players](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.asuv2zzhmbsd) without further constraints often places objects at 90° to each other because the corners of the square are farther from the center than the edges. Use [set_circular_placement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.15fez3e52vqr) combined with [enable_tile_shuffling](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ewsg05tiznb0) to solve this issue.
- When used for loosely grouped objects, some group members may fail to spawn if the closest free area is too small.
- **Bug**: previously placed objects directly on the origin, but now places objects one tile away. Use [max_distance_to_players](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.v2aq68odkdzl) 0 if the object is needed right on the origin.

**Example**: Give each player a fishing ship on the closest free water tile.
```rms
<OBJECTS_GENERATION>
create_object FISHING_SHIP {
  set_place_for_every_player
  ignore_terrain_restrictions
  terrain_to_place_on WATER
  find_closest
}
```

**See more**: [find_closest](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.kzkb7o2yhtk9)
