import http from 'k6/http';
import { check, sleep } from 'k6';

// export const options = {
//   stages: [
//     { duration: '40s', target: 1000 },
//     { duration: '1m', target: 600 },
//     { duration: '1m30s', target: 500 },
//   ],
// };

// export const options = {
//   stages: [
//     { duration: '40s', target: 2000 },
//     { duration: '1m', target: 1500 },
//     { duration: '1m30s', target: 1000 },
//   ],
// };

// export const options = {
//   stages: [
//     { duration: '1ms', target: 4000 },
//     { duration: '2m', target: 3000 },
//     { duration: '2m30s', target: 2000 },
//   ],
// };

export const options = {
  stages: [
    { duration: '1m', target: 8000 },
    { duration: '2m', target: 6000 },
    { duration: '2m30s', target: 4000 },
  ],
};

export default function () {
  const res = http.get('http://20.79.240.83:3000');
  check(res, { 'status was 200': (r) => r.status == 200 });

  sleep(1);
}