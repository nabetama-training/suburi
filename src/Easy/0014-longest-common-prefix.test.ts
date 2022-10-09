function longestCommonPrefix(strs: string[]): string {
  if (!strs.length) {
    return '';
  }
  if (strs.length === 1) {
    return strs[0];
  }

  strs.sort();
  let result = '';

  // 配列はソート済みなので、先頭要素の文字列と末尾要素の文字列で共通する接頭辞は、
  // 配列のすべての要素で共通であることが保証されている
  for (let i = 0; i < strs[0].length; i++) {
    if (strs[0][i] === strs[strs.length - 1][i]) {
      result = result.concat(strs[0][i]);
    } else {
      break;
    }
  }

  // no common
  return result;
}

describe('longestCommonPrefix', () => {
  test('empty array', () => {
    expect(longestCommonPrefix([])).toBe('');
  });

  test('array contains only one element', () => {
    expect(longestCommonPrefix(['foo'])).toBe('foo');
  });

  test('array contains multiple elements there have common prefix', () => {
    expect(longestCommonPrefix(['flower', 'flow', 'flight'])).toBe('fl');
  });

  test('array contains multiple elements, which have a common prefix', () => {
    expect(longestCommonPrefix(['flower', 'flow', 'flight'])).toBe('fl');
  });

  test('array contains multiple elements but no common prefix', () => {
    expect(longestCommonPrefix(['dog', 'racecar', 'car'])).toBe('');
  });
});
