x = 0
with open('in.txt', 'r') as f:
    x = f.readlines()
    
def getFirst(line, nums, wNums, i):
    c = 0
    while c < len(line)+1:
        s = line[0:c]
        for num in nums:
            if num in s:
                return num
        for num in wNums.keys():
            if not i:
                if num in s:
                    return wNums[num]
            elif num[::-1] in s:
                return wNums[num]
        
        c+=1     

nums = ["0", '1', '2', '3', '4', '5', '6', '7', '8', '9']
wNums = {"one": '1', "two": '2', "three": '3', "four": '4', "five": '5', "six": '6', "seven": '7', "eight": '8', "nine": '9', "zero": '0'}

sum = 0

for line in x:
    s1 = getFirst(line, nums, wNums, False)
    s2 = getFirst(line[::-1], nums, wNums, True)
    sum += int(s1 + s2)
    
    
print("sum", sum)