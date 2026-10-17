import assert from 'node:assert/strict';
import test from 'node:test';

test('synthetic Node fixture executes on the configured runtime', () => {
  assert.equal([1, 2, 3, 4].reduce((sum, value) => sum + value, 0), 10);
});
