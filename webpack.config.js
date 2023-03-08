const path = require('path');

module.exports = {
  mode: 'development',
  entry: './userscript/entry.js',
  output: {
    path: path.resolve(__dirname, 'pkg'),
    filename: 'userscript.user.js',
  },
};