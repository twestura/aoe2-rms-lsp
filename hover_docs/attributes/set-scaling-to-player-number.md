```rms
set_scaling_to_player_number
```

---

Scales [number_of_groups](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.t7eg3wg2xm3w) to the player count, e.g. ×2 for a 2-player game and ×8 for an 8-player game.

If no grouping is present, scaling applies to [number_of_objects](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.btcx5h61er65) instead.

**Mutually exclusive with**: [set_scaling_to_map_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ctsq8l5z99u6)

**Example**: Scale the number of relics by the number of players.
```rms
<OBJECTS_GENERATION>
create_object RELIC {
  number_of_objects 2
  set_scaling_to_player_number
}
```

**See more**: [set_scaling_to_player_number](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.l48a16uing0q)
