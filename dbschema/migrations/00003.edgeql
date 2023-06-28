CREATE MIGRATION m1acewuowvsna5o4f7fvnsmgixgsvabjp5l2vylrp4gyfs2mm33cha
    ONTO m1yt2sv6x3a7xfiaqges4wito3gty6y2yqppyyivuvbenu3hvo4ixa
{
  ALTER TYPE default::NPC {
      DROP PROPERTY places_visited;
  };
  ALTER TYPE default::NPC {
      CREATE MULTI LINK places_visited: default::City;
  };
};
