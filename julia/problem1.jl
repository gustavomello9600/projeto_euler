s = 0

for i = 1:999
    if i % 3 == 0 || i % 5 == 0
        global s += i
    end
end

println(s)
