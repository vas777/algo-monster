
# Checklist
When no pattern fits, ask these five questions in sequence:

Can you work backward from the goal?
- What was the step before final answer?
Do you know a related problem with the same unknown?
- Shortest path / distance between words ? => could be BFS
Can you solve a simpler version, then add the condition back?
- Simplify simplify and then add complexity back.
Did you use all the data, including the constraints?
- Are you accounting for some constrain ? Like input is sorted or unique
Can you try small cases and find the rule?
- Try test case of 10 not 100 like original question


## What to Do When You're Stuck on a Coding Problem
Sometimes you read a problem, you understand the examples, and no pattern that you know fits. The core patterns are still necessary, because you cannot reason from nothing. But when they do not give a plan, you need a method to find one.

This article gives five questions to ask in that situation. The questions come from How to Solve It by George Pólya (1945), a book about how to approach unfamiliar math problems. Each section below gives one question, one problem, and the point in that problem where the question helps. Each section links to the full write-up of the problem.

## Can you work backward?
Start from the goal and ask what had to happen immediately before it. This question helps when you must make a choice at each step going forward, but you cannot tell which choice is correct.

Broken Calculator has two buttons: double the number, and subtract 1. You must go from startValue to target in the minimum number of presses. Going forward from 3 to 10, the obvious plan doubles to 12 and then subtracts twice, for 4 presses. A better plan subtracts before the last double (3 → 6 → 5 → 10), for 3 presses. Going forward, you cannot know when a subtraction is better. The values go up to 10^9, so a search over all sequences is too slow.

Backward, the buttons become "halve" and "add 1". Now each step is almost forced: an odd number cannot come from a double, so you must add 1. From 10 you get 5, then 6, then 3. Each forward step was a guess, but each backward step has only one good option.

## Do you know a related problem?
Look at the unknown, which is the thing the problem asks for. Then think of a problem that you solved before with the same kind of unknown. This question helps when the problem does not look like any category that you know.

Word Ladder asks for the shortest sequence of words from beginWord to endWord, where each step changes one letter and each word is in the word list. If you change letters toward the end word, you soon need a step that does not get closer, such as hot to dot. After that, many paths are possible.

The unknown is "the fewest steps from a start to a goal". That is a shortest path in a graph. Make each word a node, and connect two words with an edge if they differ by one letter. All edges have the same cost, so breadth-first search (BFS) finds the shortest path. This question is useful only if you know many solved problems, because you cannot connect to a problem that you have not seen.

## Can you solve a simpler version?
Remove part of the condition, solve that easier problem, and then add the condition back. This question helps when the full condition makes the direct method too slow.

Valid Palindrome II asks if a string can become a palindrome after you delete at most one character. The direct method deletes each character in turn and checks the remainder. For a string of length 10^5, that is about 10^10 steps.

Remove the deletion first and check a plain palindrome with two pointers, one at each end. The pointers move inward until they find the first pair that does not match. Now add the deletion back. Every pair outside that point already matches, so the deletion must be one of the two characters in the mismatched pair. Check the two remaining substrings. The plain check found the only place where the deletion can go.

## Did you use all the data?
When you have a plan, compare it with the input and the constraints. If one fact is not in your plan, find out why the problem gives it. This question helps when your plan breaks one of the constraints.

Two Sum II asks for two numbers in an array that add up to a target, with only constant extra space. The usual hash map solution uses extra space. A check of every pair is too slow for 3 × 10^4 numbers. Neither plan uses the fact that the array is sorted.

A sorted array tells you where the smallest and the largest values are. Add the smallest and the largest. If the sum is too large, the largest value is too large with every other value, so you can remove it. If the sum is too small, remove the smallest value for the same reason. Each comparison removes one number. Constraints are data too: a sorted input points to binary search or two pointers, and a small n permits a search of all subsets. The runtime summary matches input sizes to solutions.

## Can you try small cases?
Solve the smallest cases by hand, write down the answers, and look for a rule. Then explain why the rule is correct. This question helps when a simulation is too slow and you cannot see a formula.

Bulb Switcher has n bulbs that start off. In round i, you toggle every i-th bulb. The problem asks how many bulbs are on after n rounds. For n up to 10^9, a simulation of all the rounds is too slow.

For n from 1 to 10, the bulbs that stay on are 1, 4, and 9. These are the perfect squares. To explain the rule, look at divisors: bulb k toggles one time for each divisor of k. Divisors usually come in pairs, which gives an even count. A perfect square has one divisor that pairs with itself, which gives an odd count. Thus, the answer is the integer square root of n.
