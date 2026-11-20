function dprt(v)
	local function dbg(v)
		local buf, t, i = "", type(v), 0
		if t == "table" then
			buf = "table{"
			for i in pairs(v) do
				if v[i] ~= nil then
					buf = string.format("%s \n\t%s: %s", buf, i,dbg(v[i]))
				end
			end
			buf = string.format("%s \n},", buf)
		elseif t == "userdata" then
			buf = "userdata{"
			for i=0,#v-1 do
				buf = string.format("%s %s: %s", buf, i, dbg(v[i]))
			end
			buf = string.format("%s \n},", buf)
		elseif t == "number" then
			buf = string.format("%s,", v)
		elseif t == "string" then
			buf = string.format("'%s',", v)
		elseif t == "boolean" then
			if v then
				buf = "true,"
			else
				buf = "false,"
			end
		elseif t == "function" then
			buf = "function{"
			local info = debug.getinfo(v)
			for i in pairs(info) do
				buf = string.format("%s, %s: %s", buf, i, info[i])
			end
			buf = string.format("%s },", buf)
		elseif t == "nil" then
			buf = "nil,"
		end 
		return buf
	end
	print(dbg(v))
end


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


-- print(file.getcwd())

-- p = [[.\**\*.rs]]
-- for path in file.glob(p) do
--   print(file.dirname(path))
--   print(file.basename(path))
--   print("")
-- end
--]]

--main()

--t = '{"name":"LuaMate","version":1,"ar":[9,10,11],"b":false}'
--text = json.encode(j)
--print(text)
--
-- data = json.decode(t)
-- print(data.name)
-- print(data.version)
-- print(#data.ar)
-- if data.b == true then
--   print("true")
-- else
--   print("false")
-- end

-- j = json.encode(data)
-- print(j)

--prent(a)
--

--[[
a, b = regex.match("([a-z]+)([0-9]+)", "abc124")
print(a)
print(b)
--]]
--
--
--dprt(terminal)

h = terminal.color("Hello, luamate!", "blue", nil, {"bold","italic","blink"})
print(h)
--
a = input()

audio.play_wait("sample/ok2.wav")

--[[
a = "  abc "
print(a)
print(text.trim(a))
print("please enter")
b = input()
clipboard.set(b)
c = clipboard.get()
print(c)
--]]
--
--
--[[
dialog.msgbox("test")
dialog.msgbox("msg", "Hello, world!" )
r = dialog.yes_no("確認", "よろしいですか?")
if r then
  print("true")
else
  print("false")
end

f = dialog.select_dirs("select directories")

if f then
  for i, v in pairs(f) do
    print(v)
  end
end
--]]
