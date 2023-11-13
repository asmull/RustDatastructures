use crate::linked_list::LinkedList;

pub mod linked_list;

fn main() {
    let data = 3;
    let mut a_list = LinkedList::<u32>::new(data);


    a_list.add(4);
    a_list.add(5);
    a_list.add(6);

    println!("{}", a_list.last());
    println!("{}", a_list.last());
    println!("{}", a_list.last());
}
