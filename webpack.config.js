const path = require('path');
const webpack = require('webpack');

module.exports = (env, argv) => {
  const isProd = argv.mode === 'production';

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
        SERVER: JSON.stringify(isProd ? 'https://github.com/domsleee/farkle/raw/gh-pages' : 'http://127.0.0.1:8080')
      }),
      new webpack.BannerPlugin({
        banner: `// ==UserScript==
// @name          Farkle
// @match         https://cardgames.io/farkle*
// ==/UserScript==`,
        entryOnly: true,
        raw: true
      })
    ]
  };
  return commonConfig;
};
