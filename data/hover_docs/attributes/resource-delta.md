```rms
resource_delta NUMBER
```

---

Modifies the amount of food, wood, gold, or stone in an object.

- Does not work for farms.
- Does not appear when testing a map from the scenario editor.
- In pre-DE versions, resource amount will overflow past 32,767. In DE, it will overflow past 2,147,483,647.
- Negative values can be used to reduce resources.

**Arguments**:
- `NUMBER`: A number (default 0).

**Example**: Create gold piles with 100 less gold, and stone mines with 100 more stone.
```rms
<OBJECTS_GENERATION>
create_object GOLD {
  number_of_objects 7
  resource_delta -100
}
create_object STONE {
  number_of_objects 7
  resource_delta 100
}
```

**See more**: [resource_delta](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ndaw6icg9cnp)
