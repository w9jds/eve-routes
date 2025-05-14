SELECT
  systems."solarSystemID" as "id",
  systems."solarSystemName" as "name",
  systems."x",
  systems."y",
  systems."z",
  systems."security",
  array_agg(jumps."toSolarSystemID") as neighbors
from
  "mapSolarSystems" systems
  LEFT JOIN "mapSolarSystemJumps" jumps ON jumps."fromSolarSystemID" = systems."solarSystemID"
GROUP BY
  systems."solarSystemID",
  systems."solarSystemName",
  systems."x",
  systems."y",
  systems."z",
  systems."security";