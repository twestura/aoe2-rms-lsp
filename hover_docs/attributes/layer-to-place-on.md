```rms
layer_to_place_on TERRAIN
```

---

Restricts object placement to the specified layering terrain.

- Works for [terrain_mask](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.e0ug99qovffm) 1, but not when set to 2. In that case use [terrain_to_place_on](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.t7m0dcvh1px) instead, since the layer has become the main terrain.
- If used together with [terrain_to_place_on](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.t7m0dcvh1px), the object is placed only where both the base terrain and the layer apply.

**Arguments**:
- `TERRAIN`: A [`terrain constant`](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.3bdjnf7tryyk) (default: any layer).

**Example**: Place rocks on a small patch of layered snow within a larger desert area.
```rms
<LAND_GENERATION>
create_land {
  terrain_type DESERT
  number_of_tiles 500
  land_position 50 50
}

<TERRAIN_GENERATION>
create_terrain SNOW {
  base_terrain DESERT
  number_of_tiles 20
  terrain_mask 1
}

<OBJECTS_GENERATION>
create_object ROCK {
  number_of_objects 300
  layer_to_place_on DESERT
}
```

**See more**: [layer_to_place_on](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ze7b3ms0whcu)
