import http from 'k6/http';
import { sleep } from 'k6';

const BASE_URL = 'http://localhost:8000';
const JSON_HEADERS = { headers: { 'Content-Type': 'application/json' } };

// Justin Bieber's post (author_id 26, shard 2). Every vote/comment on this
// post routes to shard 2 no matter which shard the voter/commenter is on.
const CELEBRITY_POST_ID = 14;

export const options = {
  scenarios: {
    // Light, steady baseline spread across a handful of ordinary posts,
    // so shards 0/1/3 show some normal activity instead of total silence.
    background_trickle: {
      executor: 'constant-arrival-rate',
      rate: 5,
      timeUnit: '1s',
      duration: '90s',
      preAllocatedVUs: 10,
      maxVUs: 20,
      exec: 'backgroundTrickle',
    },
    // The celebrity moment: ramps up to 50 concurrent virtual users all
    // voting/commenting on the same post, all landing on shard 2.
    celebrity_spike: {
      executor: 'ramping-vus',
      startVUs: 0,
      stages: [
        { duration: '30s', target: 50 },
        { duration: '30s', target: 50 },
        { duration: '15s', target: 0 },
      ],
      exec: 'celebritySpike',
      startTime: '15s', // let the background trickle warm up first
    },
  },
};

// Creates a handful of ordinary users + their own posts, spread across
// whichever shards their usernames happen to hash to.
export function setup() {
  const regularPostIds = [];
  const numRegularUsers = 12;

  for (let i = 0; i < numRegularUsers; i++) {
    const username = `loaduser_${Date.now()}_${i}`;
    const userRes = http.post(`${BASE_URL}/users`, JSON.stringify({ username }), JSON_HEADERS);
    const userId = userRes.json('id');

    const postRes = http.post(
      `${BASE_URL}/posts`,
      JSON.stringify({ author_id: userId, title: `Regular post ${i}`, body: 'just a normal post' }),
      JSON_HEADERS,
    );
    regularPostIds.push(postRes.json('id'));
  }

  return { regularPostIds };
}

// votes.user_id and comments.author_id have no FK anymore (cross-shard, see
// the celebrity-problem writeup), so we can use synthetic ids to simulate
// many distinct voters/commenters without pre-creating real accounts.
export function backgroundTrickle(data) {
  const postId = data.regularPostIds[Math.floor(Math.random() * data.regularPostIds.length)];
  const fakeUserId = 1_000_000 + __VU * 100_000 + __ITER;

  if (Math.random() < 0.7) {
    http.post(
      `${BASE_URL}/votes`,
      JSON.stringify({ post_id: postId, user_id: fakeUserId, value: 1 }),
      JSON_HEADERS,
    );
  } else {
    http.post(
      `${BASE_URL}/comments`,
      JSON.stringify({ post_id: postId, author_id: fakeUserId, body: 'nice post' }),
      JSON_HEADERS,
    );
  }
}

export function celebritySpike() {
  const fakeUserId = 2_000_000 + __VU * 100_000 + __ITER;

  if (Math.random() < 0.7) {
    http.post(
      `${BASE_URL}/votes`,
      JSON.stringify({ post_id: CELEBRITY_POST_ID, user_id: fakeUserId, value: 1 }),
      JSON_HEADERS,
    );
  } else {
    http.post(
      `${BASE_URL}/comments`,
      JSON.stringify({ post_id: CELEBRITY_POST_ID, author_id: fakeUserId, body: 'omg love this!!' }),
      JSON_HEADERS,
    );
  }

  sleep(0.1);
}
