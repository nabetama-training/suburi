interface romanMapType {
  [key: string]: number;
}

const romanMap: romanMapType = {
  I: 1,
  V: 5,
  X: 10,
  L: 50,
  C: 100,
  D: 500,
  M: 1000,
};

const romanToInt = (s: string): number => {
  let total = 0;

  const romans = s.split('');

  romans.forEach((roman, i) => {
    const current = romanMap[roman];

    // 右側に記号があるか？
    if (i + 1 < romans.length) {
      const next = romanMap[romans[i + 1]];

      // 現在の値が右側の記号の値以上であれば、現在の記号の値を合計に加算する。
      if (current >= next) {
        total += current;
      } else {
        // 現在の値が右側の記号の値以上であれば、現在の記号の値を合計に加算する。
        total -= current;
      }
    } else {
      total += current;
    }
  });
  return total;
};

describe('romanToInt()', () => {
  test('I', () => {
    expect(romanToInt('I')).toBe(1);
  });

  test('II', () => {
    expect(romanToInt('II')).toBe(2);
  });

  test('III', () => {
    expect(romanToInt('III')).toBe(3);
  });

  test('IV', () => {
    expect(romanToInt('IV')).toBe(4);
  });

  test('VII', () => {
    expect(romanToInt('VII')).toBe(7);
  });

  test('LVIII', () => {
    expect(romanToInt('LVIII')).toBe(58);
  });

  test('MCMXCIV', () => {
    expect(romanToInt('MCMXCIV')).toBe(1994);
  });
});
