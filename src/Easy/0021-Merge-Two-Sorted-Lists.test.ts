class ListNode {
  val: number;
  next: ListNode | null;
  constructor(val?: number, next?: ListNode | null) {
    this.val = val === undefined ? 0 : val;
    this.next = next === undefined ? null : next;
  }
}

function mergeTwoLists(
  list1: ListNode | null,
  list2: ListNode | null
): ListNode | null {
  if (list1 === null) {
    return list2;
  }
  if (list2 === null) {
    return list1;
  }

  return null;
}

describe('', () => {
  test('list1 is empty', () => {
    const lnode = new ListNode(1, new ListNode(2, new ListNode(4, null)));
    expect(mergeTwoLists(null, lnode)).toEqual(lnode);
  });
  test('list2 is empty', () => {
    const lnode = new ListNode(1, new ListNode(2, new ListNode(4, null)));
    expect(mergeTwoLists(lnode, null)).toEqual(lnode);
  });
});
