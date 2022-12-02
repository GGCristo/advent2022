-- foodLog -> {{elf, sumCalories}, ...}
local function getFoodLog(path)
  local elf = 1
  local result = {{elf = elf, calories = 0}}
  for line in io.lines(path) do
    if line == "" then
      elf = elf + 1
      result[elf] = {elf = elf, calories = 0}
    else
      local record = result[elf]
      record["calories"] = record["calories"] + tonumber(line)
    end
  end
  return result
end

local function getMaxCalories(foodLog, window)
  table.sort(foodLog, function (r1, r2)
    return r1.calories > r2.calories
  end)
  return {table.unpack(foodLog, 1, window)}
end

local data = getFoodLog("input.txt")
local top3Calories = getMaxCalories(data, 3)
-- Part 1
print(string.format("The elf %i is the one that carries most calories with %i calories", top3Calories[1].elf,
  top3Calories[1].calories))
-- Part 2
print("Top 3 elves")
local total = 0
for _, record in ipairs(top3Calories) do
  print(string.format("The elf %i carries %i calories", record.elf,
    record.calories))
  total = total + record.calories
end
print("Total:", total)
