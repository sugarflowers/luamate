function main() 
  print("Hello, mlua!")
  for line in file.lines("Cargo.toml") do
    print(line)
  end

 --[[ 
  f = file.open_file("テスト.txt")
  f:write("hello\n")
  f:write("world\n")
  f:write("こんにちは世界\n")

  f:close()
--]]
--
--[[
  print("\n\n-----")
  print(file.read("Cargo.toml"))
  print("-----")

  if file.exists("Cargo.toml2") then
    print("toml2 is exists!")
  else
    print("toml2 is not exists!")
    file.makedirs("testdir")
    print("create testdir!")
    file.rmdir("testdir")
    print("remove dir testdir")

    print(file.getcwd())
    print(file.exepath())
  end

  if file.isfile("Cargo.toml") then
    print("Cargo.toml is file!")
  else
    print("Cargo.toml is not file!")
  end

  if file.isfile("target") then
    print("target is file!")
  else
    print("target is not file!")
  end
--]]
end



print(type(json))

main()

--t = '{"name"="LuaMate","version"=1,"ar"=[9,10,11]}'
t = '{"name":"LuaMate","version":1,"ar":[9,10,11],"b":false}'
--text = json.encode(j)

--print(text)
data = json.decode(t)
print(data.name)
print(data.version)
print(#data.ar)
if data.b == true then
  print("true")
else
  print("false")
end

j = json.encode(data)
print(j)
