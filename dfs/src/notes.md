# What is recursion
‘To understand recursion, you must first understand recursion’

# Order of visits
*Assumption is that movement is left-to-right.

In which order subtrees (left, right) are visited with regards to current node.

   A
  / \
 B   C

ABC order - pre-order ; current A visited PRE BC order
BAC order - in-order; current A visited IN between BC order
BCA order - post-order; current A visited POST BC order 

# Types of trees

Complete - all levels are full; except the last on - nodes there must be 
as far lef as possible.

Perfect - all level are completed

Number of nodes in tree of height n + 1;

Number of leafs = 2^n 
Number of inner nodes = 2^n - 1
Total number = 2*2^n-1 = 2^(n+1)-1 

# Binary Search Tree (BST)

Search is O(h) ( h - height );
Just compare and walk down the tree.

Insertion is cheaper relative to arrays. Because no need to move `neighbours.
Insertion is O(h) ( h - height )
Search and add item
