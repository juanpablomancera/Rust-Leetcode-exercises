// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl MergeSortedLists {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut array1 = Vec::new();
        let mut l1ptr = list1;
        while let Some(entry) = l1ptr {
            array1.push(entry.val);
            l1ptr = entry.next;
        }
        array1 = array1.into_iter().rev().collect();

        let mut array2 = Vec::new();
        let mut l2ptr = list2;
        while let Some(entry) = l2ptr {
            array2.push(entry.val);
            l2ptr = entry.next;
        }
        array2 = array2.into_iter().rev().collect();
        array1.extend(array2);
        array1.sort();

        let mut current = None;
        for num in array1.into_iter().rev(){
            let mut new_node = Box::new(ListNode {
                val:num,
                next:current,
            });
            current = Some(new_node);
        }
        current
    }
}