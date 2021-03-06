import { visit, currentURL, click } from '@ember/test-helpers';
import { setupApplicationTest } from 'ember-qunit';
import { module, test } from 'qunit';

import setupMirage from '../helpers/setup-mirage';

module('Acceptance | Logout', function (hooks) {
  setupApplicationTest(hooks);
  setupMirage(hooks);

  test('successful logout', async function (assert) {
    let user = this.server.create('user', { name: 'John Doe' });
    this.authenticateAs(user);

    await visit('/crates');
    assert.equal(currentURL(), '/crates');
    assert.dom('[data-test-user-menu] [data-test-toggle]').hasText('John Doe');

    await click('[data-test-user-menu] [data-test-toggle]');
    await click('[data-test-user-menu] [data-test-logout-button]');

    assert.equal(currentURL(), '/');
    assert.dom('[data-test-user-menu] [data-test-toggle]').doesNotExist();
  });
});
