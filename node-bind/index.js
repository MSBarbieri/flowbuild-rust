const { loadBinding } = require('@node-rs/helper');

/**
 * __dirname means load native addon from current dir
 * 'engine' is the name of native addon
 * the second arguments was decided by `napi.name` field in `package.json`
 * the third arguments was decided by `name` field in `package.json`
 * `loadBinding` helper will load `engine.[PLATFORM].node` from `__dirname` first
 * If failed to load addon, it will fallback to load from `@flowbuild/engine-[PLATFORM]`
 */
module.exports = loadBinding(__dirname, 'engine', '@flowbuild/engine');
