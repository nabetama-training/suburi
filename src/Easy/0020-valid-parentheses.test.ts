function isValid(s: string): boolean {
  // 空文字は invalid
  if (s === '') {
    return false;
  }

  const stack: string[] = [];
  const str = s.split('');

  for (let i = 0; i < str.length; i++) {
    if (str[i] === '(' || str[i] === '[' || str[i] === '{') {
      stack.push(str[i]);
    } else {
      if (stack.length < 1) {
        // 1文字目が開き括弧でなければ invalid
        return false;
      } else {
        // 先頭の開き括弧に対応する閉じ括弧は必ず末尾にある
        const left = stack[stack.length - 1];
        if (
          (str[i] === ')' && left === '(') ||
          (str[i] === ']' && left === '[') ||
          (str[i] === '}' && left === '{')
        ) {
          stack.pop();
        } else {
          return false;
        }
      }
    }
  }

  if (stack.length > 0) {
    return false;
  }
  return true;
}

describe('isValid', () => {
  test('empty', () => {
    expect(isValid('')).toEqual(false);
  });

  test('nothing pair', () => {
    expect(isValid('{')).toEqual(false);
  });

  test('invalid', () => {
    expect(isValid('(])')).toEqual(false);
  });

  test('valid parens', () => {
    expect(isValid('{}')).toEqual(true);
  });

  test('valid-1', () => {
    expect(isValid('{}()[]')).toEqual(true);
  });

  test('valid-2', () => {
    expect(isValid('{([])}')).toEqual(true);
  });

  test('valid-3', () => {
    expect(isValid('{([][]{})}')).toEqual(true);
  });
});
