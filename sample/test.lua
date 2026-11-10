function main() 
  print("Hello, mlua!")
  for line in file.lines("Cargo.toml") do
    print(line)
  end

  f = file.open_file("テスト.txt")
  f:write("hello\n")
  f:write("world\n")
  f:write("こんにちは世界\n")

  f:close()


end

main()

