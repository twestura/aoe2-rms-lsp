```rms
set_facet FACET_NUMBER
```

---

Selects which frame of an object's sprite to use when placing it.

- For units, this corresponds to the angle they are facing.
- For other objects, this may correspond to alternative appearances.
- Facets can be cycled using the rotate feature in the scenario editor.
- Frames can be viewed using external tools such as [SLX Studio](https://aok.heavengames.com/blacksmith/showfile.php?fileid=13179).

**Arguments**:
- `FACET_NUMBER`: A number (default 0, random facet).
  - Corresponds to the index - 1 of the desired frame in the sprite.
  - To get the first frame, use a facet number below 0 or above the maximum for that object.

**Example**: Generate 10 jungle straggler trees that all use the same appearance.
```rms
<OBJECTS_GENERATION>
create_object JUNGLETREE {
  number_of_objects 10
  set_facet 30
}
```

**See more**: [set_facet](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.qbzrlsoh7lg7)
