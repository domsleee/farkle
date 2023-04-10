const path = require('path');
const webpack = require('webpack');
const packageJson = require('./package.json');
const { writeFileSync } = require('fs');

module.exports = (env, argv) => {
  const isProd = argv.mode === 'production';
  const localPath = path.join(path.resolve(__dirname), 'pkg', 'userscript.user.js');
  const localInclude = isProd
    ? ''
    : `\n// @require       file:${localPath}`;
  const banner = `// ==UserScript==
// @name          Farkle-${isProd ? 'production' : 'development'}
// @version       ${packageJson.version}
// @match         https://cardgames.io/farkle*${localInclude}
// ==/UserScript==`;
  if (!isProd) {
    writeFileSync('./pkg/userscript.development.user.js', banner);
  }

  const commonConfig = {
    mode: 'development',
    entry: './userscript/entry.js',
    optimization: {
      minimize: false
    },
    output: {
      path: path.resolve(__dirname, 'pkg'),
      filename: 'userscript.user.js',
    },
    plugins: [
      new webpack.DefinePlugin({
        SERVER: JSON.stringify(isProd ? 'https://domsleee.github.io/farkle-ai' : 'http://127.0.0.1:8080')
      }),
      new webpack.BannerPlugin({
        banner,
        entryOnly: true,
        raw: true
      })
    ]
  };
  return commonConfig;
};
