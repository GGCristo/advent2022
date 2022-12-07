local function getData(path)
  local pairsOfPairs = {}
  for line in io.lines(path) do
    local pairs = {}
    for match in line:gmatch("([^,]+)") do
      local iterNumber = match:gmatch("([^-]+)")
      table.insert(pairs, { tonumber(iterNumber()), tonumber(iterNumber()) })
    end
    table.insert(pairsOfPairs, pairs)
  end
  return pairsOfPairs
end

local function isAnyContained(pairsOfPair)
  local leftPair, rightPair = pairsOfPair[1], pairsOfPair[2]
  if leftPair[1] >= rightPair[1] and leftPair[2] <= rightPair[2] then
    return true
  elseif rightPair[1] >= leftPair[1] and rightPair[2] <= leftPair[2] then
    return true
  else
    return false
  end
end

local function doesAnyOverlap(pairsOfPair)
  local leftPair, rightPair = pairsOfPair[1], pairsOfPair[2]
  for lvalue = leftPair[1], leftPair[2] do
    if lvalue >= rightPair[1] and lvalue <= rightPair[2] then
      return true
    end
  end
  return false
end

local function run(pairsOfPairs)
  local score1, score2 = 0, 0
  for _, pairsOfPair in ipairs(pairsOfPairs) do
    if isAnyContained(pairsOfPair) then score1 = score1 + 1 end
    if doesAnyOverlap(pairsOfPair) then score2 = score2 + 1 end
  end
  return score1, score2
end

local data = getData("input.txt")
print(("Part1: %i, Part2: %i"):format(run(data)))
