```rms
set_zone_by_team
```

---

Assigns the same zone to all members of the same team. Used with [create_player_lands](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.esy5dq29i0wr).

Lands sharing the same zone can grow to touch each other. If two lands have different zones and [other_zone_avoidance_distance](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.2oebaei6j04a) is specified, land growth will avoid touching the other zone.

- If used with [create_land](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.te4jg2upqvlx), it will assign the land to the same zone as the team of player 1, even if the land is a non-player land or is the assigned land for a member of a different team (not recommended).
- Team zones correspond to (TeamNumber - 9), so team 1 is in zone -8.

**Mutually exclusive with**: [zone](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.li85mvsiskop), [set_zone_randomly](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.nweca22q5puk)

**Example**: Team Islands.
```rms
<LAND_GENERATION>
base_terrain WATER
create_player_lands {
  terrain_type DIRT
  land_percent 80
  set_zone_by_team
  other_zone_avoidance_distance 10
  top_border 10
  right_border 10
  bottom_border 10
  left_border 10
}
```

**See more**: [set_zone_by_team](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.6pxlle5x8e8w)
