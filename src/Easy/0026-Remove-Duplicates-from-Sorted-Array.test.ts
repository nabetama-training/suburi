interface Counter {
  [key: number]: number;
}

/**
 *
 * @param nums 入力かつ破壊する配列
 * @returns 重複を取り除いた配列の要素数
 */
const removeDuplicates = (nums: number[]): number => {
  if (nums.length === 0) {
    return 0;
  }

  const counter: Counter = {};

  nums.forEach(num => {
    if (counter[num]) {
      counter[num] += 1;
    } else {
      counter[num] = 1;
    }
  });

  //   let tmp = 0;
  //   Object.keys(counter).forEach(y => {
  //     const ny = Number(y);
  //     if (tmp < counter[ny]) {
  //       tmp = counter[ny];
  //     }
  //   });

  nums = Object.keys(counter).map(x => Number(x));

  return Object.keys(counter).map<number>(x => Number(x)).length;
};

describe('Remove Duplicates from Sorted Array', () => {
  test('empty array', () => {
    expect(removeDuplicates([])).toBe(0);
  });

  test('3 nums', () => {
    expect(removeDuplicates([1, 1, 2])).toBe(2);
  });

  test('4 nums', () => {
    expect(removeDuplicates([1, 1, 2, 3])).toBe(3);
  });

  test('5 nums', () => {
    expect(removeDuplicates([1, 1, 2, 3, 4])).toBe(4);
  });

  test('10 nums', () => {
    expect(removeDuplicates([1, 1, 2, 3, 4, 4, 4, 5, 6, 8])).toBe(7);
  });
});
