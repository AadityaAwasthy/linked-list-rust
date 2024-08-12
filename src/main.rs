use linked_list::LinkedList;

fn main() {
    let mut my_list = LinkedList::new();
    my_list.append(1);
    my_list.append(2);
    my_list.append(3);
    my_list.append(4);
    my_list.append(5);

    my_list.print_list();
}
