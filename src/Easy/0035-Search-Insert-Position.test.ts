/**
 *  アルゴリズムはO(logN)にする必要がある => 二分探索
 * @param nums 整数のソート済み配列
 * @param target ターゲット要素
 * @return target が見つかればそのインデックス。見つからなければ挿入されたときのインデックスを返す
 */
const searchInsert = (nums: number[], target: number): number => {
  let start = 0;
  let end = nums.length - 1;

  while (start <= end) {
    const middle = start + Math.trunc((end - start) / 2);
    if (nums[middle] === target) {
      return middle;
    } else if (nums[middle] < target) {
      start = middle + 1;
    } else {
      end = middle - 1;
    }
  }

  return start;
};

describe('Search Insert Position', () => {
  test('ex1', () => {
    expect(searchInsert([1, 3, 5, 6], 5)).toBe(2);
  });

  test('ex2', () => {
    expect(searchInsert([1, 3, 5, 6], 2)).toBe(1);
  });

  test('ex3', () => {
    expect(searchInsert([1, 3, 5, 6], 7)).toBe(4);
  });
});
