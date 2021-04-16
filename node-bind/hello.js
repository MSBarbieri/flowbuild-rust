const { Engine } = require('./index');

try {
  const instance = new Engine({
    persist_mode: 'Database',
  });
  console.log({ uuid: instance.createProcess() });
} catch (err) {
  console.log({ err });
  process.exit(1);
}

