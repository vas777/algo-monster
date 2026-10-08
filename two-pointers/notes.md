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

# Fast and Slow Family Map

Cycle detection.

condition to check
If cycle does not exist fast will reach null.
// check fast and fast.next for null
If cycle exists fast and slow must meet.
// check slow == fast

# Two Pointers Decision Rule
1. Am I going to maintain contiguous part of input ?
Window same direction with constant gap
2. Am I able to elimitate part of input because it is monotonic in some way ?
start on Opposite ends 
3. Am I modifying in place somehow, filtering ?
start from the Same end and move pointers at different speed
4. Linked list
Fast/slow - two pointers in same direction but with lists;

### from article
Ask These Questions
When you see a new problem, ask these questions in order.

1. Am I maintaining a contiguous interval?
If the answer depends on a subarray or substring between left and right, you may be looking at a sliding window problem.

Typical signals:

longest or shortest valid substring
subarray satisfying a running condition
counts, sums, or frequencies over a moving interval
Examples:

Largest Subarray Sum
Longest Subarray Sum Smaller Than or Equal to Target
Longest Substring Without Repeating Characters
2. Can I safely eliminate one side?
If the structure is sorted, symmetric, or has a bottleneck property, you may be looking at opposite-direction pointers.

Typical signals:

sorted array and pair target
palindrome or mirror comparison
boundary optimization where one side becomes provably useless
Examples:

Two Sum Sorted
Valid Palindrome
Container With Most Water
3. Am I rewriting or filtering in place?
If you are scanning the input once while building a cleaned or compacted version at the front, you may be looking at same-direction pointers.

Typical signals:

remove duplicates in-place
move certain elements forward
stable partitioning or compaction
Examples:

Remove Duplicates
Move Zeros
4. Am I reasoning about relative position in a linked list?
If the structure is a linked list and you need the middle, a cycle, or the node before a target position, then fast/slow pointers or a fixed gap is often the right tool.

Examples:

Middle of a Linked List
Remove N-th Node from End of Linked List
Linked List Cycle
