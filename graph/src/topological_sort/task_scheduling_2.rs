This problem extends Task Scheduling. Each task now has a duration, where times[i] is how long tasks[i] takes.

You can run any number of tasks in parallel, as long as dependencies are respected. If [a, b] appears in requirements, task a must finish before task b can start.

Return the minimum total time required to complete all tasks.

There is guaranteed to be a solution.

Examples
Example 1
Input:

Copy
tasks = ["a", "b", "c", "d"]
times = [1, 1, 2, 1]
requirements = [["a", "b"], ["c", "b"], ["b", "d"]]
Output: 4
Figure
Example 1 dependency graph with task durations

The longest dependency chain is c -> b -> d, so the minimum total time is 2 + 1 + 1 = 4.

fn task_scheduling_2(tasks: Vec<String>, times: Vec<i32>, requirements: Vec<Vec<String>>) -> i32 {
    // WRITE YOUR BRILLIANT CODE HERE
    0
}