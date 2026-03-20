```rms
override_actor_radius_if_required
```

---

Prevents buildings from overlapping when placed in an [actor area](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.obv5ypy66a57) with a [radius](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.z9i7h4jrjeaf) too small to contain them, by expanding the valid placement area outward.

- Requires [actor_area_to_place_in](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.d6d6k8uc57zx).
- Used in the official maps for Mills in Empire Wars to ensure they are placed properly when they become Folwarks for the Poles.
- Does not work on units.

**Example**: Place a Barracks with the default [actor_area_radius](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.z9i7h4jrjeaf) of&nbsp;1. Then place a House adjacent to but not overlapping the Barracks.
```rms
<OBJECTS_GENERATION>
create_object BARRACKS {
  set_place_for_every_player
  find_closest
  actor_area 2
}
create_object HOUSE {
  set_place_for_every_player
  actor_area_to_place_in 2
  override_actor_radius_if_required
}
```

**See more**: [override_actor_radius_if_required](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.4t8s9kqehrvb)
