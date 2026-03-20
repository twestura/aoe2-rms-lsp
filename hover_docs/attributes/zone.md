```rms
zone ZONE_NUMBER
```

---

Sets a numeric zone for the land.

Lands sharing the same zone can grow to touch each other. If two lands have different zones and [other_zone_avoidance_distance](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.2oebaei6j04a) is specified, land growth will avoid touching the other zone.

- By default, lands from [create_player_lands](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.esy5dq29i0wr) are each in their own unique zone (PlayerNumber - 10), while lands created with [create_land](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.te4jg2upqvlx) all share the same zone (-10).
- Land with zone -12 will not belong to any zone. It will occupy less space than specified and will not count as a unique zone for the purpose of [other_zone_avoidance_distance](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.2oebaei6j04a). Other zones are able to fully expand to cover it.
- **Bug (AoC/UP/HD)**: zone 99 will crash the game. Every 6th negative zone decreasing from -25 (-31, -37, -43, etc.) will also crash the game. Fixed in DE.

**Mutually exclusive with**: [set_zone_by_team](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.6pxlle5x8e8w), [set_zone_randomly](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.nweca22q5puk)

**Arguments**:
- `ZONE_NUMBER`: A number. A maximum of 245 should be used for lands connected with [create_connect_land_zones](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.mua0127k3zel).

**Example**: All players on the same continent, with the rest of the map as water.
```rms
<LAND_GENERATION>
base_terrain WATER
create_player_lands {
  terrain_type DIRT
  land_percent 60
  zone 1
}
```

**See more**: [zone](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.li85mvsiskop)
