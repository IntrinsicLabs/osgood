const path = require('path');
module.exports = {
  entry: './preamble.js',
  mode: process.env.NODE_ENV || 'development',
  output: {
    filename: 'preamble.js',
    path: path.join(__dirname, 'dist'),
  },
};
