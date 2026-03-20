```rms
create_actor_area X Y ACTOR_AREA_ID RADIUS
```

---

Creates an actor area at a fixed map position.

- Actor areas are created before any [create_object](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.2vz7nxt2afqo) commands, regardless of their position in the script.
- Coordinates are in tiles, not percentages. See the [map sizes reference](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.qannz915qgy5) for dimensions per map size.
- Multiple actor areas may share the same identifier.
- Coordinates outside the map bounds are valid and can be used with a large radius to avoid map edges.
- Will crash the game if no lands are generated on the map.

**Arguments**:
- `X`: X-coordinate in tiles.
- `Y`: Y-coordinate in tiles.
- `ACTOR_AREA_ID`: A number used to reference this actor area in [actor_area_to_place_in](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.d6d6k8uc57zx) and [avoid_actor_area]([URL](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.cgoa0e8x398u)).
- `RADIUS`: Square radius in tiles.

**Example**: Prevent relics from spawning near the center of the map.
```rms
<OBJECTS_GENERATION>
if TINY_MAP create_actor_area 60 60 1234 30
elseif SMALL_MAP create_actor_area 72 72 1234 36
elseif MEDIUM_MAP create_actor_area 84 84 1234 42
elseif LARGE_MAP create_actor_area 100 100 1234 50
elseif HUGE_MAP create_actor_area 110 110 1234 55
elseif GIGANTIC_MAP create_actor_area 120 120 1234 60
elseif LUDIKRIS_MAP create_actor_area 240 240 1234 120
endif

create_object RELIC {
  number_of_objects 500
  set_gaia_object_only
  avoid_actor_area 1234
}
```

**See more**: [create_actor_area](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.u28jmnfojke3)
