import {twoSum} from '../0001-two-sum';

describe('0001-two-sum', () => {
  test('Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.', () => {
    expect(twoSum([2, 7, 11, 15], 9)).toEqual([0, 1]);
  });
});
