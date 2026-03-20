```rms
replace_terrain OLD_TERRAIN NEW_TERRAIN
```

---

Replaces a specific terrain along the connection path with another terrain.

- Can and should be used multiple times for different terrains.
- A terrain can be replaced with itself.
- Connections can pass through terrains even if they are not specified with `replace_terrain`.
- In DE, `OLD_TERRAIN` refers to the terrain present at the beginning of [<CONNECTION_GENERATION>](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.urxh5ze1aaoh), even if that terrain has already been replaced by a previous connection command. Use [accumulate_connections](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.bd2l5930vfzf) to disable this behavior.

**Arguments**:
- `OLD_TERRAIN`: A [terrain constant](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.3bdjnf7tryyk).
- `NEW_TERRAIN`: A [terrain constant](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.3bdjnf7tryyk).

**Example**: Replace several terrains along a connection path.
```rms
<CONNECTION_GENERATION>
create_connect_all_players_land {
  replace_terrain GRASS DIRT2
  replace_terrain FOREST LEAVES
  replace_terrain SNOW_FOREST GRASS_SNOW
  replace_terrain DIRT DIRT3
  replace_terrain WATER SHALLOW
}
```

**See more**: [replace_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.5h9ggnuativl)
