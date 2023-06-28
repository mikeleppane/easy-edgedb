module default {
  scalar type Class extending enum<Rogue, Mystic, Merchant>;
  abstract type Person {
    required name: str;
    multi places_visited: City;
  }

  type PC extending Person {
    required class: Class;
  }

  type NPC extending Person {
  }

  type City {
    required name: str;
    modern_name: str;
    important_places: array<str>;
  }

}
