
-- Set the firing delay and energy for all planes to 0.
for key, plane in pairs(data.planes) do
  plane.fire_delay = 0
  plane.fire_energy = 0
end

-- Also enable infinite fire for tornado multishot
for key, special in pairs(data.specials) do
  if special.name == "multishot" then
    special.cost = 0
    special.delay = 0
  end
end
