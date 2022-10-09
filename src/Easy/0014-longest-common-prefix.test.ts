interface Counter {
  [key: string]: number;
}

function longestCommonPrefix(strs: string[]): string {
  if (!strs.length) {
    return '';
  }
  return '';
}

describe('longestCommonPrefix', () => {
  test.only('["flower, "flow", "flight"]', () => {
    expect(longestCommonPrefix([])).toBe('');
  });
});
