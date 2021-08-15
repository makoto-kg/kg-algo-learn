const performance = require('perf_hooks').performance;

const LOOP_MAX = 10000000;
const TEST_MAX = 100;

const pi = () => {
  const start = performance.now();

  let cnt = 0;

  for (let i = 0; i < LOOP_MAX; ++i) {
    const x = Math.random();
    const y = Math.random();

    if (x * x + y * y <= 1.0) {
      cnt += 1;
    }
  }

  const end = performance.now();
  const elapsed = (end - start) / 1000.0;

  console.log(4.0 * cnt / LOOP_MAX);
  console.log(elapsed);
  return elapsed;
};

const main = () => {
  console.log('hello');
  let ave = 0.0;

  for (let i = 1; i <= TEST_MAX; ++i) {
    console.log(`${i} times`);
    ave += pi();
  }

  console.log(`AVE: ${ave / TEST_MAX}`);
}

main();