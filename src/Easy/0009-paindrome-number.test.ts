describe('Paindrome Number', () => {
  test('test', () => {
    expect(solve(121)).toBe(true);
    expect(solve(-121)).toBe(false);
    expect(solve(12345)).toBe(false);
    expect(solve(12321)).toBe(true);
  });
});

const solve = (x: number): boolean => {
  const before = `${x}`;
  const after = before.split('').reverse().join('');
  return before === after;
};
