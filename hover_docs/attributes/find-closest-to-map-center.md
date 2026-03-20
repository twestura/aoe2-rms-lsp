```rms
find_closest_to_map_center
```

---

Places the object on the closest free tile to the center of the map, taking all other constraints into account.

- Requires [set_place_for_every_player](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.hch89ipgsvb) or [place_on_specific_land_id](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.34wzlujx4lbv).
- Overridden by [find_closest](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.kzkb7o2yhtk9).
- When used for loosely grouped objects, some group members may fail to spawn if the closest free area is too small.

**Example**: Place a boar at the map center for each player.
```rms
<OBJECTS_GENERATION>
create_object BOAR {
  set_place_for_every_player
  set_gaia_object_only
  find_closest_to_map_center
}
```

**See more**: [find_closest_to_map_center](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.c8jwpxwfx68p)
