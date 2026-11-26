function string.trim(str)
  return str:match("^%s*(.-)%s*$")
end

function string.split(str, delim)
  result = {}
  local pattern = string.format("([^%s]+)", delim:gsub("%%", "%%%%"))
  for item in str:gmatch(pattern) do
    table.insert(result, item)
  end
  return result
end
