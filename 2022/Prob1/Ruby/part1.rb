
def highest_calories
    highest_calories = 0
    temp = 0
    File.foreach('./input.txt') do |line|
        if line == "\n"
            highest_calories = temp if temp > highest_calories
            temp = 0
        else
            temp += line.to_i
        end
    end

    highest_calories
end

puts highest_calories