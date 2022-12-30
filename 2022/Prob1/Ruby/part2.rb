def evict_lowest(val, arr)
    # Guard clauses
    raise Exception.new ("Array should be of length 3") if arr.size != 3
    return arr if val < arr[0]
    
    return [val, arr[1], arr[2]] unless val > arr[1] # Replace only the lowest
    return [arr[1], val, arr[2]] unless val > arr[2] # The new value is the middle child
    return [arr[1], arr[2], val]
end

def highest_calories
    arr_highest_calories = [0, 0, 0]
    temp = 0
    File.foreach('./input.txt') do |line|
        if line == "\n"
            if temp > arr_highest_calories[0]
                arr_highest_calories = evict_lowest(temp, arr_highest_calories) 
            end
            temp = 0
        else
            temp += line.to_i
        end
    end
    arr_highest_calories
end

print highest_calories
puts