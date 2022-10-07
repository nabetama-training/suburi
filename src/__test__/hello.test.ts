import {hello} from '../index';

describe('hello', () => {
  test('hello returns hello', () => {
    expect(hello()).toBe('hello');
  });
});
