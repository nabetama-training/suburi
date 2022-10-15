import exp = require('constants');

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
  if (!list1) {
    return list2;
  }

  if (!list2) {
    return list1;
  }

  // マージ後の先頭要素
  let mergedHead: ListNode | null = null;

  // 両方のリンクリストの最初のノードを比較して先頭要素を決める
  if (list1.val <= list2.val) {
    mergedHead = list1;
    list1 = list1.next;
  } else {
    mergedHead = list2;
    list2 = list2.next;
  }

  // マージ後の末尾要素
  let mergedTail = mergedHead;

  // 残りの後続要素（個数不定）すべてについて
  while (list1 && list2) {
    let temp: ListNode | null = null;
    // 小さい要素を選択してマージ後の要素の末尾に追加していく
    if (list1.val <= list2.val) {
      temp = list1;
      list1 = list1.next;
    } else {
      temp = list2;
      list2 = list2.next;
    }

    // 要素が指していたポインタを1つ移動させる
    mergedTail.next = temp;
    mergedTail = temp;
  }

  if (list1) {
    mergedTail.next = list1;
  } else if (list2) {
    mergedTail.next = list2;
  }

  return mergedHead;
}

describe('Merge Two Sorted Lists', () => {
  test('list1 is empty', () => {
    const lnode = new ListNode(1, new ListNode(2, new ListNode(4, null)));
    expect(mergeTwoLists(null, lnode)).toEqual(lnode);
  });
  test('list2 is empty', () => {
    const lnode = new ListNode(1, new ListNode(2, new ListNode(4, null)));
    expect(mergeTwoLists(lnode, null)).toEqual(lnode);
  });

  test('merged list', () => {
    const lnode1 = new ListNode(1, new ListNode(2, new ListNode(4, null)));
    const lnode2 = new ListNode(1, new ListNode(2, new ListNode(4, null)));

    const want = new ListNode(
      1,
      new ListNode(
        1,
        new ListNode(2, new ListNode(2, new ListNode(4, new ListNode(4, null))))
      )
    );
    expect(mergeTwoLists(lnode1, lnode2)).toEqual(want);
  });
});
