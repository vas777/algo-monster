# Prefix sums

a: 1 2 3 4 5  6  7  8
p: 0 1 3 6 10 15 21 28
i: 0 1 2 3 4  5  6  7

prefix[i] = prefix[i - 1] + array[i]

Main property 
for j>i
Sum[i,j) = Sum[0,j) - Sum[0,i)

so sum 3 to 6 is 

Sum[3,6) = Sum[0,6) - Sum[0,3) = 21 - 6 = 15 

1 2 3 4 5 6 - 1 2 3 = 4 + 5 + 6 = 15

it answers question like what is the sum up until certain index 
but not included

Helps to answer range queries

## Example 

Count All  subarrays that equal to certain target sum
arr [3 1 -2 5 4] ; target 8

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

prefix[i] : index 
11 - 8 = 3 

do we have sum 3 somewhere in our prefix sum array ?

# 31.10.2026
my conversation about prefix sum with Gemini

Prefix Sum Knowledge Verification
https://gemini.google.com/app/8167437ce90eec2c

User prompt: I want you to verify my knowledge of prefix sum
Response: I will verify your understanding of prefix sums.
Your reference notes that this technique turns a search into a hash table lookup.
First, let us establish the fundamental mechanism.
How do you mathematically construct a basic prefix sum array from a standard input array?

User prompt: One builds a prefix sum as a sum of each consecutive sum of elements in an array. Let's start why it's called prefix. Basically, it means sum until this index equal to a certain number, sum. But because we calculate before it, before a certain index, it will be a prefix, like in a part of the word. So it's before that index and it's named prefix. The first index in prefix sum array would be zero because there is nothing before the very first element and then so on. The formula will be i + i - 1. So let's say 1 2 3 4. So the first element will be zero in prefix sum. Second will be one because previous one, like 1 + 0 is one. Then the next element will be 2 + previous one, which is one, is three. And then the element number two will be 3 + 3 = 6. And then 6 + 4 = 10 and so on, 15, 21. It's just a consecutive running sum. One can use this structure to have query range queries. A problem that could be used to solve it with is like what's the sum, count how many subarrays of a certain array equal to a certain target sum. And the insight is that sum of certain subarray is the difference between two ranges. So if a sum is between i and j, so it would be equal sum from zero to j, j excluded, minus sum from zero to i. I have more to say, but let's verify this and then we keep more why it will not work very well with if elements are negative, how to search it, etc. But yeah, verify this up to now.
Response: An input array of [1, 2, 3, 4] produces a prefix sum array of [0, 1, 3, 6, 10].
A range sum from index i to j equals prefix[j] - prefix[i].
Your mathematical sequence and range query logic are correct.
You defined the recurrence formula as i + i - 1.
This notation lacks specific array references.
How do you write this exact step using the prefix and arr array variables?

User prompt: prefix[i] = prefix[i - 1] + array[i]
Response: The equation exactly represents the recursive addition step.
This operation builds the array efficiently.
Briefly explain why reading the previous prefix value prevents redundant loop calculations.

User prompt: Because we already did that calculation, we don't need to again loop till i - 1 to calculate the sum. We basically memorize it, and so we reuse already calculated data. Plus, if we calculate sum, just sum of elements up to point i is basically sum of i - 1, i - 2, and so on till the beginning. And like just next step is inductive, like you just add next one and then you have a value at this point.
Response: The previous value prevents redundant looping by storing the accumulated total of all preceding elements.
This mechanism acts as a cache, turning an O(N) nested summation into a single O(1) addition step.
Since you correctly identified the core principle, we can move forward.
You mentioned negative numbers in your initial thought.
Why do negative numbers cause a problem for the specific technique of finding subarrays with a given sum using a two-pointer sliding window?

User prompt: So, with a negative number, one way to query subarrays, find a certain sum in a certain range, range query, would be a two pointers approach with a sliding window, two pointers moving in the same direction, fast and slow. But we will be relying that initially we'll rely on that prefix sum, prefix array is monotonically growing, which means we know how to properly advance our pointers. There would be no problems in growing and shrinking the prefix, the sliding window. So, for example, if we want to find a sum of in between I and J, we first grow our window, and if some in that window is different and the sum is smaller than target, it mean we successfully we still can add more elements, more bigger elements by moving fast pointer forward. As soon as our difference between fast and slow bigger than existing target, we need to subtract something by moving slow pointer forward. But what would happen that with a minus element, we we can have a situation when we by moving slow pointer forward, we expected some to decrease, but because we have negative number, we actually increase the difference. And then we could then we could what? That's kind of where my problem is. We could So, we increase it, but we expected to decrease, and we Yeah, I'm not sure. Yeah, I guess I don't know. I feel like what would happen, I feel like because I'm not sure, it really we expected it to decrease, but it actually increased. And we again could move left pointer forward, which we actually should not do.
Response:
You correctly identified the flaw in the sliding window.
Subtracting a negative number increases the total window sum.
The fluctuating sum breaks the sliding window algorithm.
A running prefix sum turns this search into a hash table lookup.
You previously defined the range sum as prefix[j] - prefix[i] = target.
How can you rearrange this algebraic equation to find the required prefix[i] value?

User prompt: It should be mentioned that j is always bigger than i, so prefix j should be bigger for this to work. With negative numbers, we can search for negative, like we search for complementary. Anywho, prefix j - prefix i = target could be rearranged for a hash map query where target - prefix j = prefix i, so we can immediately check, have we seen before this complementary value in our hash map? So yeah, answer prefix i = target - prefix j, and this value could be immediately will be how many times we've seen this sum or have we seen it at all, like how many also at all. So yeah, we can query for that.
Response:
The original equation is prefix[j] - prefix[i] = target.
Subtracting prefix[j] from both sides yields -prefix[i] = target - prefix[j].
Multiplying both sides by negative one yields prefix[i] = prefix[j] - target.
Your rearranged equation prefix[i] = target - prefix[j] is mathematically incorrect.
Your concept of finding a complementary value in a hash map is theoretically sound.
However, your inverted algebraic formula searches for the wrong integer.
Assume your current prefix[j] equals 10 and your target equals 4.
What specific integer does your formula calculate for the hash map query?
What integer does the correct formula calculate?

User prompt: We must look for prefix j = 4 - target. So, in our case, 10 - 4 will be 6. We have to see if there are 6, but my formula will be inverted. I will be searching for -6. Just subtraction. I know subtraction formulas are hard. Hope you understand humor. But I think you can figure it to not reply to my humor.
Response: Your calculation of -6 proves the inverted formula is incorrect.
The correct expression prefix[j] - target yields the required value of 6.
This correct value represents a previous prefix sum.
The algorithm must search for this exact value in a hash map.
How does the hash map initially acquire these previous prefix sum values during the array iteration?

User prompt: How does the hash map initially acquire this previous prefix sum value during the iteration? We check if we have it. So, the hash map is key-value. Our key is a certain sum, and the value varies based on what we are trying to use. One way is to count how many subarrays there are. We start from value 0 = 1 because 0 at least once. It's an initialization phase, and then we either insert and put the counter, let's say how many, to 1, or we increment. We either insert existing and set to 1 or update the value to have 2. So, increment how many times we've seen this prefix sum before.
Response:
Your explanation of the hash map operations is completely correct.
You correctly identified the key as the running sum.
You correctly identified the value as the frequency count.
You mentioned initializing the hash map with a zero key.
Why must the hash map contain this initial zero key?
What specific subarray case fails without this initial zero value?

User prompt: If I'm not mistaking, I'm just doing the calculation in my head. A reason will be if there is no zero, our continuous calculation will break, meaning from the very beginning to a certain j. If we don't have a zero, we will be subtracting some of the first element, but we don't need to like it has to be still included. So zero is a case to keep a continuous calculation also correct from zero till until j.
Response:
Your reasoning is completely correct.
The initial zero key handles subarrays that begin at the first index.
If a running sum exactly equals the target, the required complement is zero.
Without this initial key, the algorithm misses these valid subarrays.
You have successfully explained the core prefix sum technique.
Now consider the space complexity of this approach.
What is the worst-case auxiliary space complexity for an array of length N?
What specific data structure consumes this memory?

User prompt: Worst case auxiliary space. I think the worst case and the best case are the same here. Prefix sum is basically always n + 1. Because initial zero and then we go through whole elements and then basically each element in the array gets a prefix sum. So n + 1, so big O(n). Hash map consumes this memory. We will be using a hash map. And the hash map gives us a constant lookup. So a bit like an advantage over two pointers array. Two pointers approach plus two pointers problem with a negative, so it eliminates. But what we do is pre-calculation, which could be a problem sometimes. But also pre-calculation means the arrays are static, so it's kind of it depends. Yeah, it's actually pre-calculation could be on a static array, so I assume that's also a drawback of this approach. I mean, not a drawback, it just
Response:
For an array of length N, the algorithm calculates up to N prefix sums.
These prefix sum values act as the keys in the hash map.
Therefore, the hash map stores up to N+1 keys (including the initial zero).
This yields an auxiliary space complexity of O(N).
Your answers for the space complexity and the data structure are correct.
You stated that pre-calculation requires a static array.
If you modify a single element in the original array, what happens to all subsequent values in the prefix sum array?
How does this modification cost affect the O(1) lookup time advantage?

User prompt: I think if one element have changed, that would require us to modify by the difference, meaning if element on position I have changed, like subtracted or added to, all subsequent elements till the end have to be updated again because some until that, some after that element have changed. It doesn't affect lookup, but we would need to update, and update again will be presumed linear. Oh, and we would need to invalidate if it a hash map keys that existed before will not, it will not work anymore because yeah, we look by the key, it's not array. We cannot just update next, there is no next. We have to update all hash map. So lookup cost doesn't change, but it's probably gives you a amortized cost worse because you have to recalculate the hash map.
Response: If you modify a single element in the original array, every subsequent prefix sum from that modified index to the end of the array must be recalculated.
This recalculation takes O(N) linear time in the worst case.
This O(N) update cost completely destroys the O(1) constant time lookup advantage, degrading the overall performance of the algorithm for dynamic data.
You have demonstrated a solid theoretical and practical grasp of prefix sums and their limitations.
Which algorithmic concept or problem type would you like to verify next?

