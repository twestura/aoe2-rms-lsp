```rms
make_indestructible
```

---

Makes a building indestructible. The building receives 9,999 HP, 1,000/1,000 armor, and cannot be attacked, damaged, or deleted.

- Has no effect on units or other non-building objects.
- Can be used to create neutral gaia markets, docks, or whole cities that cannot be attacked.
- **Bug**: causes the game to crash when attempting to "test map" from the scenario editor.

**Example**: Make the starting town center indestructible.
```rms
<OBJECTS_GENERATION>
create_object TOWN_CENTER {
  set_place_for_every_player
  max_distance_to_players 0
  make_indestructible
}
```

**See more**: [make_indestructible](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.a9ken9hkekd6)
