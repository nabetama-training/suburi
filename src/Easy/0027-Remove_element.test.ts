/**
 *
 * @param nums 整数の配列
 * @param val nums から削除すべき要素
 * @return 削除後の配列要素数
 */
function removeElement(nums: number[], val: number): number {
  // This passed the tests, and `nums` are valid, In local environment.
  // but In leetcode.com, This failed.
  //   nums = nums.filter(num => num !== val);
  //   return nums.length;
  for (let i = 0; i < nums.length; i++) {
    const n = nums[i];
    if (n === val) {
      nums.splice(i, 1);
      // 削除した要素の分配列が短くなるので、ループインデックスを戻す
      i--;
    }
  }
  return nums.length;
}

describe('Remove Element', () => {
  test('ex1', () => {
    expect(removeElement([3, 2, 2, 3], 3)).toBe(2);
  });

  test('ex2', () => {
    expect(removeElement([0, 1, 2, 2, 3, 0, 4, 2], 2)).toBe(5);
  });
});
