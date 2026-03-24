```rms
set_zone_randomly
```

---

Randomly assigns a zone to the land, which may be shared with other lands that also use `set_zone_randomly`, [set_zone_by_team](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.6pxlle5x8e8w), or a manually specified zone in the correct range.

Lands sharing the same zone can grow to touch each other.
If two lands have different zones and [other_zone_avoidance_distance](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.2oebaei6j04a) is specified, land growth will avoid touching the other zone.

The land gets a random zone in the range -8 to (PlayerCount - 9).
This means:
- The land will never share a zone with a land given a positive numeric zone or a non-player land without a zone assigned.
- A non-player land with `set_zone_randomly` will never share a zone with player 1, if player 1 is using their default zone of -9.

**Mutually exclusive with**: [zone](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.li85mvsiskop), [set_zone_by_team](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.6pxlle5x8e8w)

**Example**: Archipelago map where players might share an island with allies, enemies, or nobody.
```rms
<LAND_GENERATION>
base_terrain WATER
create_player_lands {
  terrain_type DIRT
  land_percent 80
  set_zone_randomly
  other_zone_avoidance_distance 10
  top_border 10
  right_border 10
  bottom_border 10
  left_border 10
}
```

**See more**: [set_zone_randomly](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.nweca22q5puk)
