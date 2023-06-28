CREATE MIGRATION m1yt2sv6x3a7xfiaqges4wito3gty6y2yqppyyivuvbenu3hvo4ixa
    ONTO m1yzhxilbtfqdsuhfj45ckvftdhduz7vizvxuktojobtkejjwpu7gq
{
  CREATE TYPE default::City {
      CREATE PROPERTY modern_name: std::str;
      CREATE REQUIRED PROPERTY name: std::str;
  };
  ALTER TYPE default::NPC {
      CREATE REQUIRED PROPERTY name: std::str {
          SET REQUIRED USING (<std::str>{});
      };
      CREATE PROPERTY places_visited: array<std::str>;
  };
};
