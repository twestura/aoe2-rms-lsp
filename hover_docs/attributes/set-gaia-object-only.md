```rms
set_gaia_object_only
```

---

Use with [set_place_for_every_player](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.hch89ipgsvb) to place gaia (neutral) objects on a per-player basis. Required when placing gold, stone, berries, deer, or boar for every player.

- Can be used for controllable objects such as sheep.
- Units and buildings permanently join the player who first finds them, unless [set_gaia_unconvertible](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.g4mzdyb4izbo) also is specified.

**Example**: Give every player four gaia sheep close to their starting town.
```rms
<OBJECTS_GENERATION>
create_object SHEEP {
  number_of_objects 4
  set_loose_grouping
  set_gaia_object_only
  set_place_for_every_player
  min_distance_to_players 7
  max_distance_to_players 8
}
```

**See more**: [set_gaia_object_only](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.bnzkfqaopnv9)
