const hello = (): string => {
  return 'hello';
};

const sum = async (...nums: number[]) => {
  return nums.reduce((acum, curr) => acum + curr, 0);
};

export {hello, sum};
