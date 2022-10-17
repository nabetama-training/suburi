/**
 *
 * The judge will test your solution with the following code:
int[] nums = [...]; // Input array
int[] expectedNums = [...]; // The expected answer with correct length

int k = removeDuplicates(nums); // Calls your implementation

assert k == expectedNums.length;
for (int i = 0; i < k; i++) {
    assert nums[i] == expectedNums[i];
}
 * @param nums 入力値かつ"破壊しなくてはならない"配列
 * @returns 重複を取り除いた配列の要素数
 */
const removeDuplicates = (nums: number[]): number => {
  let nextNonDuplicateIndex = 1;

  nums.forEach((num, i) => {
    if (nums[nextNonDuplicateIndex - 1] !== nums[i]) {
      nums[nextNonDuplicateIndex] = nums[i];
      nextNonDuplicateIndex += 1;
    }
  });

  return nextNonDuplicateIndex;
};

describe('Remove Duplicates from Sorted Array', () => {
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
