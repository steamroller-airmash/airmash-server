
for idx, missile in pairs(data.missiles) do
  missile.max_speed = 30.0
  missile.base_speed = 30.0
  missile.distance = 10000.0
end

-- Set the firing delay and energy for all planes to 0.
for key, plane in pairs(data.planes) do
  plane.fire_delay = 0.1
  plane.fire_energy = 0
end

for key, special in pairs(data.specials) do
  if special.name == "multishot" then
    special.cost = 0
    special.delay = 0.1
  end
end
