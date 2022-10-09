interface Counter {
  [key: string]: number;
}

function longestCommonPrefix(strs: string[]): string {
  if (!strs.length) {
    return '';
  }
  if (strs.length === 1) {
    return strs[0];
  }

  // no common
  return '';
}

describe.only('longestCommonPrefix', () => {
  test('empty array', () => {
    expect(longestCommonPrefix([])).toBe('');
  });

  test('array contains only one element', () => {
    expect(longestCommonPrefix(['foo'])).toBe('foo');
  });
});
