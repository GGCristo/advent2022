local function getData(path)
  local data = {}
  for line in io.lines(path) do
    table.insert(data, line)
  end
  return data
end

local function getPriority(c)
  local ascii = c:byte()
  if ascii >= 97 and ascii <= 122 then -- lowercase
    return ascii - 96
  end -- uppercase
  return ascii - 38
end

local function part1(data)
  local totalPriority = 0
  for _, line in ipairs(data) do
    local lh, rh = line:sub(1, #line / 2), line:sub((#line / 2) + 1)
    local set = {}
    for c in rh:gmatch(".") do
      set[c] = true
    end
    for c in lh:gmatch(".") do
      if set[c] then
        totalPriority = totalPriority + getPriority(c)
        break
      end
    end
  end
  return totalPriority
end

local function part2(data, groupNumber)
  local totalPriority = 0
  for i = 1, #data, groupNumber do
    local countChar = {}
    for j = 0, groupNumber - 1 do
      local charSet = {}
      for c in data[i + j]:gmatch(".") do
        charSet[c] = true
      end
      for c in pairs(charSet) do
        countChar[c] = (countChar[c] or 0) + 1
        if countChar[c] == groupNumber then
          totalPriority = totalPriority + getPriority(c)
          break -- This will break the outer loop as well, bcs is the last iteration
        end
      end
    end
  end
  return totalPriority
end

local data = getData("input.txt")
print(part1(data))
print(part2(data, 3))
