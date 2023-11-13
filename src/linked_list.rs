/*
This is an implementation of a singly-linked list with extra verbose comments. These comments are put in place entirely
to make me, the author accountable for my learning goals, which is to understand Rust syntax and programming at a level
beyond what I previously had.

A node in a linked linked list contains a piece of data, and an optional pointer to the next node of the linked list
(if there is more list)
*/

/* 
This is a type definition - it just serves as a shorthand to simplify writing out a compound type every place we want to 
use it. Note that this type definition is templated with a "generic" <T> - it is valid for any type T.

This type definition contains the "Box" data type - Box is a type that specifies we want heap-allocated memory in Rust.
We need to heap-allocate memory because, since the data structure is recursive, we don't know  how much memory the
entire structure will take. We can't perform static allocation on the stack, and thus need a pointer 
(which is fixed size) to a piece of memory on the heap.
*/
type Node<T> = Option<Box<LinkedList<T>>>;

/*
This is a Rust definition for a strucutre - a fixed size piece of memory, which strongly (statically) typed data fields.
This structure is defined again, as a generic <T: Copy>. Here, this means that the piece
*/
// This is the definition for a structure: A fixed 
// The T: Copy is called a "Generic" in Rust - the equivalent of C++ templating
// The "Copy" part says that the generic type T must implement the "Copy" function.
// This requirement is driven by 
pub struct LinkedList<T: Copy> {
   data: T,
   next: Node<T>,
}
// ToDo: Problems with this
// Our current allocation doesn't allow for an empty list.

impl<T: Copy> LinkedList<T> {
    pub fn new(data: T) -> Self {
        LinkedList{data: data, next: Option::None}
    }

    pub fn add(&mut self, data: T) {
        // Add a new node
        if let Some(node) = &mut self.next {
            node.add(data);
        } else {
            self.next = Some(Box::new(LinkedList::<T>::new(data)));
        }
    }

    pub fn last(&mut self) -> T {
        if let Some(node) = &mut self.next {
            return node.last();
        } else {
            return self.data;
        }
    }
}