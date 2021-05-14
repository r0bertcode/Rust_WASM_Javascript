const path = require('path');

const OUT_DIR = path.join(__dirname, 'public');

module.exports = {
  mode: 'development',
  entry: './main.js',
  output: {
    path: OUT_DIR,
    filename: 'bundle.js'
  },
  module: {
    rules: [
      {
        test: /\.js$/,
        use: 'babel-loader',
        exclude: /node_modules/,
      },
    ]
  },
  experiments: {
    asyncWebAssembly: true,
  },
};