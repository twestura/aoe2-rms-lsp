```rms
terrain_type TERRAIN
```

---

Specifies which terrain the land uses.

**Arguments**:
- `TERRAIN`: A [terrain constant](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.3bdjnf7tryyk) (default `GRASS`).

**Example**: Create player lands made of dirt.
```rms
<LAND_GENERATION>
create_player_lands {
  terrain_type DIRT
  land_percent 20
}
```

**See more**: [terrain_type](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.lzceesmva36o)
