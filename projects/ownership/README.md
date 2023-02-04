Ownership is a set of rules that helps Rust to manage memory. These memories are stored in the stack or the heap.

For data stored on the stack, the size of the data must be fixed. Data of unknown size of data is stored in the heap instead.

To access the data in the heap, a pointer is used which is a fixed size. It doesn't store the value but just the address that points to the value of the data. This data can now be stored on the stack to be used.

Adding data on the stack is called pushing to the stack while adding to the heap is called allocating on the heap.

OWNERSHIP RULES:
Each value in Rust has a owner.
There can only be one owner at a time.
When the value goes out of scope, the value will be dropped.
