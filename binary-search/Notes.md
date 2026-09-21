# What I recall before reading ?

It is most efficient way to navigate sorted elements.
By comparing `current` element with element you are looking for 
you can tell which direction - left or right - you must move.
Usually choice is middle element 

- M = L + (R - L)/2;

or 

- M = (L + R)/2

where:
- R > L 
- L - index on the left element;
- R - index on the right element;

Array must be sorted for binary search to work.
Sorting provides / encodes necessary information about the array.

Outcomes during search:

Assuming:

[Left...Middle...Right]

- Hit => we found the element.
- Miss far => element that we currently are looking at is `too` 
big and we need to go back, search in the left side of the remaining
array. Aka, new search is between Left and Middle.
- Miss close => element is smaller than expected, we need to search 
in right side of element. Aka, new search is between Middle and Right.

# What actually is correct ?

This idea can be implemented both iteratively and recursively. However, the major difference is that the iterative version of binary search uses O(1) memory while the recursive version uses O(log(N)) memory.

1. When to terminate ?
Not to skip a potential match we should have while r<=l 
2. Whether/how to update left and right boundary in the if conditions
Division uses floor so 5/2 = 2
so we would need
mid = left+1 
when deciding to change range. 
3. Should I discard the current element?
