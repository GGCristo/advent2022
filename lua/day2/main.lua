local POINTS = { shapes = { rock = 1, paper = 2, scissor = 3 }, outcome = { lose = 0, draw = 3, win = 6 } }

local function getPoints1(left, right)
  local points = POINTS.shapes[right]
  if left == right then
    return points + POINTS.outcome.draw
  end
  if left == "rock" then
    if right == "paper" then
      return points + POINTS.outcome.win
    end -- scissor
    return points + POINTS.outcome.lose
  elseif left == "paper" then
    if right == "rock" then
      return points + POINTS.outcome.lose
    end -- scissor
    return points + POINTS.outcome.win
  end
  -- scissor
  if right == "rock" then
    return points + POINTS.outcome.win
  end -- paper
  return points + POINTS.outcome.lose
end

local function getPoints2(left, outcome)
  if left == "rock" then
    if outcome == "lose" then
      return POINTS.shapes.scissor + POINTS.outcome.lose
    elseif outcome == "draw" then
      return POINTS.shapes.rock + POINTS.outcome.draw
    end -- win
    return POINTS.shapes.paper + POINTS.outcome.win
  elseif left == "paper" then
    if outcome == "lose" then
      return POINTS.shapes.rock + POINTS.outcome.lose
    elseif outcome == "draw" then
      return POINTS.shapes.paper + POINTS.outcome.draw
    end -- win
    return POINTS.shapes.scissor + POINTS.outcome.win
  end
  -- scissor
  if outcome == "lose" then
    return POINTS.shapes.paper + POINTS.outcome.lose
  elseif outcome == "draw" then
    return POINTS.shapes.scissor + POINTS.outcome.draw
  end -- win
  return POINTS.shapes.rock + POINTS.outcome.win
end

local function play(path)
  local ENCODING1 = { A = "rock", B = "paper", C = "scissor", X = "rock", Y = "paper", Z = "scissor" }
  local ENCODING2 = { X = "lose", Y = "draw", Z = "win" }
  local points1, points2 = 0, 0
  for line in io.lines(path) do
    local char_iter = string.gmatch(line, "%a")
    local left, right = char_iter(), char_iter()
    points1 = points1 + getPoints1(ENCODING1[left], ENCODING1[right])
    points2 = points2 + getPoints2(ENCODING1[left], ENCODING2[right])
  end
  return points1, points2
end

local points1, points2 = play("input.txt")
print("Part1:", points1, "Part2:", points2)
