module default {
  type NPC {
    required name: str;
    multi places_visited: City;
  }
  type City {
    required name: str;
    modern_name: str;
  }

}
