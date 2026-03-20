```rms
create_connect_land_zones ZONE_1 ZONE_2 {
  /* Attributes */
}
```

---

Generates connections between all lands belonging to the specified zones.

- By default, player lands are each in their own zone&nbsp;(PlayerNumber - 10), and all non-player lands share zone&nbsp;(-10).
- Multiple lands sharing the same zone are connected to each other.
- Zone id numbers must not exceed&nbsp;245.

**Arguments**:
- `ZONE_1`: Zone ID at the start of the connection.
- `ZONE_2`: Zone ID at the end of the connection.

**Attributes**:
- [default_terrain_replacement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.p16vd5cszmhm)
- [replace_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.5h9ggnuativl)
- [terrain_cost](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.pw0ckpmic7kh)
- [terrain_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.o6c6lz9vb2w7)

**Example**: Connect a forest and a desert land to each other.
```rms
<LAND_GENERATION>
create_land {
  terrain_type FOREST
  land_percent 5
  zone 1
}
create_land {
  terrain_type DESERT
  land_percent 5
  zone 1
}
create_land {
  terrain_type WATER
  land_percent 5
  zone 50
}

<CONNECTION_GENERATION>
create_connect_land_zones 1 50 {
  default_terrain_replacement ICE
}
```

**Example 2**: Connect player&nbsp;2 and player&nbsp;4.
```rms
<LAND_GENERATION>
create_player_lands {
  terrain_type DESERT
  land_percent 5
}

<CONNECTION_GENERATION>
create_connect_land_zones -6 -8 {
  default_terrain_replacement ROAD
}
```

**See more**: [create_connect_land_zones](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.mua0127k3zel)
