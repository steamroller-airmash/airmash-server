
-- Set the firing delay and energy for all planes to 0.
for key, plane in pairs(data.planes) do
  plane.fire_delay = 0
  plane.fire_energy = 0
end
