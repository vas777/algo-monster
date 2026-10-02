# Explain how to recognize a two-pointer solution. Compare same-direction pointers with opposite-direction pointers using one example of each.
## How to recognize ?
1. You have an iterable structure and theoretically could apply it.
2. You have to compare structure to itself, like palindrom or finding a middle of linked list problem. Pointers walking toward each other from different end.
3. Related to 3; you are checking a property of the structure. Like are there cycles in linked-list. Pointers are moving at different speed.
4. You have to compare structure to a similar one, which might be a part of the original. Like longest substring problem - solved with sliding window.
Where: sliding window is two pointers separated by some distance. This distance could be constant or varied.
5. Compare structure between themselves; like is a subsequence is part of other one then each structure gets a pointers that walks through it.

## Compare same-direction pointers with opposite-direction pointers using one example of each.
1. same-direction pointers - will include problem like detecting subsequence; longest substring; cycle detection.
Both are moving in the same direction. Meaning if we use indecies to track them then we only increase index of pointers.
Termination could be 
- if one reached the end.
- if they overlap.  
For sliding-window which does sound like stand alone type of problem we still moving or extending window. Which would imply.
That we update them in `the same direction`.

2. opposite direction - is a word a palindrom ?
They are moving from different ends and usually if they meet it will mean termination.

## MISSING
I did not include that very important part of any implementation is to have very precise rules on when to update position of the pointers.
In subsequence problem `slow` pointer that track candidate string will only move if fast (that tracks target) points to the same char as slow.
And fast moves at each iteration.  

# Invariant
If the invariant is true before the loop starts (Initialization) and is restored at the end of every iteration (Maintenance),
then—assuming the loop terminates normally—the invariant is guaranteed to be true immediately after the loop exits (by induction).

`what must still be true after these pointers move?`

## Common Invariants: Where is the correct answer?
1. `builder` - In the constructed prefix/structure (e.g., merging sorted arrays). 
   Invariant: The built prefix contains all consumed elements in sorted order, and every element in the prefix is <= every remaining unprocessed element.

2. `seeker` - In the unexplored range or a tracked `best` variable (e.g., binary search, converging two-sum). 
   Invariant: No valid (or better) solution exists in the discarded regions; if a better solution exists, it still lies within the unexplored range [L, R].

3. `window` - In the active subarray [L, R] synced with an auxiliary state (e.g., longest unique substring). 
   Invariant: At the end of each step, `Set` exactly matches [L, R], [L, R] satisfies the constraint (unique elements), and L is the minimum index that keeps [L, R] valid for the current R. (Temporarily violated mid-step when R expands, then restored by advancing L).

4. `kinematic` - At the position of a target pointer upon termination (e.g., fast/slow pointers). 
   Invariant: After k steps, a fixed mathematical relationship holds between pointer positions (such as pos(fast) = 2 * pos(slow)), so when `fast` reaches the boundary, `slow` sits directly at the answer.

# Same Direction
- Prefix is correct
- speed is correct
- distance is correct 

what to the left is correct

fix in place  -> duplicate, 
remove or compress without extra memory => duplicate
keep only condition  => 
maintain fixed gap => find middle of linked list, find N-th linked list;

# Two pointers

# Sliding windows