const path = require('path');

const OUT_DIR = path.join(__dirname, 'public');

module.exports = {
  mode: 'development',
  entry: './main.js',
  output: {
    path: OUT_DIR,
    filename: 'bundle.js'
  },
  experiments: {
    asyncWebAssembly: true,
  },
};