# A scrip for my explanation of prefix_sum

## What do I remember ?

array
0 1 2 3 4 5 6 7
1 2 3 4 5 6 7 8

prefix sum
0 1 2 3 4  5  6  7  8
0 1 3 6 10 15 21 28 36

it answers question like what is the sum up until certain index 
but not included

original array = 1 2 3
prefix_sum = 0 1 3
prefix_sum[2] = 3

so 1 + 2 = 3 but index arr[2] is not included into calculation

## What I forgot

Helps to answer range queries

Main property 
for j>i
Sum[i,j) = Sum[0,j) - Sum[0,i)

so sum 3 to 6 is 

Sum[3,6) = Sum[0,6) - Sum[0,3) = 21 - 6 = 15 

1 2 3 4 5 6 - 1 2 3 = 4 + 5 + 6 = 15

## Example 

Count All  subarrays that equal to certain target sum
arr [3 1 2 5 4] ; target 8

prefix [0 3 4 6 11 15]


on `ALL` positive arrays one can use two-pointer approach to 
look for solution 

while l <= r 
let diff prefix[r] - prefix[l]
if diff < target 
r++
else if diff > target
l++
else 
count_solution 

BUT array with two pointer solution break two pointer logic if there are negative 
numbers because `negative` number will increase difference and wrong decision will be made.
aka. prefix array won't be monotonic as expected by two pointers

3 -4 2 5 4
0 3 -1 1 6 10



hashmap for lookup

arr [3 1 2 5 4] ; target 8

prefix [0 3 4 6 11 15]

prefix[j] - prefix[i] = target
prefix[i] = prefix[j] - target

11 - 8 = 3 

do we have sum 3 somewhere in our prefix sum array ?