```rms
accumulate_connections
```

---

Reverts a DE-specific behavior change where all connections are based on the terrain state prior to connection generation, making it impossible for later connections to replace terrain created by earlier connections.
With `accumulate_connections` active, each connection can replace terrains created by previous connections.

**Example**: Create a wide gap through a forest, then run a road through it.
```rms
<LAND_GENERATION>
base_terrain FOREST
create_player_lands {
  terrain_type FOREST
  other_zone_avoidance_distance 10
}

<CONNECTION_GENERATION>
accumulate_connections
create_connect_all_lands {
  replace_terrain FOREST LEAVES
  terrain_size FOREST 10 0
}
create_connect_all_lands {
  replace_terrain LEAVES ROAD
  terrain_size LEAVES 1 0
}
```

**See more**: [accumulate_connections](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.bd2l5930vfzf)
