import {hello, sum} from '../index';

describe('hello', () => {
  test('hello returns hello', () => {
    expect(hello()).toBe('hello');
  });
});

describe('async sum', () => {
  test('async sum', async () => {
    expect(await sum(1, 2, 3)).toBe(6);
  });
});
