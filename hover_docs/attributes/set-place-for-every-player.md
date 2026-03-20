```rms
set_place_for_every_player
```

---

Places the object as a personal object for each player (specifically for each player land).

- Objects that cannot be owned by players (Boar, Gold, trees, etc.) also require [set_gaia_object_only](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.bnzkfqaopnv9) to be placed for every player.
- Only works for player lands or lands assigned to players. Disabled by [land_id](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.c97q6t5q24lj).
- Objects are placed only where they are not separated from the origin of their land by a terrain they are restricted on, unless [ignore_terrain_restrictions](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.fx1jh8glz0tl) is used. Road terrains do not form a separation even though resources cannot be placed on them.
- If [avoid_other_land_zones](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.axswyohsolzw) is specified, the object is placed only on tiles belonging to the player's land.
- Water objects (e.g. Docks or ships) can be placed if the player land is made of a dirt terrain type.

**Mutually exclusive with**: [place_on_specific_land_id](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.34wzlujx4lbv)

**Example**: Give every player their starting Villagers.
```rms
<OBJECTS_GENERATION>
create_object VILLAGER {
  set_place_for_every_player
  min_distance_to_players 6
  max_distance_to_players 6
}
```

**See more**: [set_place_for_every_player](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.hch89ipgsvb)
