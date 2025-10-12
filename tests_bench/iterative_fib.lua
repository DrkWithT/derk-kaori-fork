local iterations = 1000000
local max_fib = 40


for i = 1, iterations do
    local a, b = 0, 1

    for j = 1, max_fib do
        local temp = a + b
        a = b
        b = temp
    end
end
